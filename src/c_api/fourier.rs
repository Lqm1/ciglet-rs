//! Planar Fourier buffers and reusable, synchronized opaque plans.
use super::{boundary, disjoint_outputs, range};
use crate::{
    Error,
    fourier::{Complex, Direction, FftPlan},
    statistics::Scalar,
};
use std::{
    panic::{RefUnwindSafe, UnwindSafe},
    sync::Mutex,
};

/// Opaque reusable binary32 FFT plan, with serialized scratch access.
pub struct CigletRsFftF32 {
    length: usize,
    plan: Mutex<FftPlan<f32>>,
}
/// Opaque reusable binary64 FFT plan, with serialized scratch access.
pub struct CigletRsFftF64 {
    length: usize,
    plan: Mutex<FftPlan<f64>>,
}

pub(super) trait Handle: RefUnwindSafe {
    type Value: Scalar + RefUnwindSafe + UnwindSafe;
    fn new(length: usize) -> Result<Self, Error>
    where
        Self: Sized;
    fn length(&self) -> usize;
    fn plan(&self) -> &Mutex<FftPlan<Self::Value>>;
}
impl Handle for CigletRsFftF32 {
    type Value = f32;
    fn new(length: usize) -> Result<Self, Error> {
        Ok(Self {
            length,
            plan: Mutex::new(FftPlan::new(length)?),
        })
    }
    fn length(&self) -> usize {
        self.length
    }
    fn plan(&self) -> &Mutex<FftPlan<f32>> {
        &self.plan
    }
}
impl Handle for CigletRsFftF64 {
    type Value = f64;
    fn new(length: usize) -> Result<Self, Error> {
        Ok(Self {
            length,
            plan: Mutex::new(FftPlan::new(length)?),
        })
    }
    fn length(&self) -> usize {
        self.length
    }
    fn plan(&self) -> &Mutex<FftPlan<f64>> {
        &self.plan
    }
}

fn direction(value: u32) -> Result<Direction, u32> {
    match value {
        0 => Ok(Direction::Forward),
        1 => Ok(Direction::Inverse),
        _ => Err(3),
    }
}

unsafe fn transform<T: Scalar + RefUnwindSafe, F>(
    real: *const T,
    imaginary: *const T,
    length: usize,
    output_real: *mut T,
    output_imaginary: *mut T,
    compute: F,
) -> u32
where
    F: FnOnce(Option<&[T]>, Option<&[T]>) -> Result<Vec<Complex<T>>, Error> + UnwindSafe,
{
    if length > i32::MAX as usize
        || length > isize::MAX as usize / std::mem::size_of::<Complex<T>>()
    {
        return 2;
    }
    for pointer in [
        real,
        imaginary,
        output_real.cast_const(),
        output_imaginary.cast_const(),
    ] {
        if !pointer.is_null()
            && let Err(status) = range(pointer, length)
        {
            return status;
        }
    }
    if !output_real.is_null()
        && !output_imaginary.is_null()
        && let Err(status) = disjoint_outputs(
            output_real.cast_const(),
            length,
            output_imaginary.cast_const(),
            length,
        )
    {
        return status;
    }
    let result = boundary(|| {
        // SAFETY: Every present input is initialized in one stable allocation.
        // Checked size/alignment precede borrowing; absent components mean zeros.
        let (real, imaginary) = unsafe {
            (
                if real.is_null() {
                    None
                } else if length == 0 {
                    Some(&[][..])
                } else {
                    Some(std::slice::from_raw_parts(real, length))
                },
                if imaginary.is_null() {
                    None
                } else if length == 0 {
                    Some(&[][..])
                } else {
                    Some(std::slice::from_raw_parts(imaginary, length))
                },
            )
        };
        compute(real, imaginary)
    });
    match result {
        Ok(values) => {
            if values.len() != length {
                return 3;
            }
            for (index, value) in values.iter().enumerate() {
                // SAFETY: Computation ended all input borrows. Present outputs
                // are distinct writable allocations/ranges and may overlap input.
                unsafe {
                    if !output_real.is_null() {
                        output_real.add(index).write(value.re);
                    }
                    if !output_imaginary.is_null() {
                        output_imaginary.add(index).write(value.im);
                    }
                }
            }
            0
        }
        Err(status) => status,
    }
}

pub(super) unsafe fn fft<T: Scalar + RefUnwindSafe>(
    length: usize,
    real: *const T,
    imaginary: *const T,
    mode: u32,
    output_real: *mut T,
    output_imaginary: *mut T,
) -> u32 {
    let direction = match direction(mode) {
        Ok(direction) => direction,
        Err(status) => return status,
    };
    // SAFETY: Forward initialized input and writable caller output contracts.
    unsafe {
        transform(
            real,
            imaginary,
            length,
            output_real,
            output_imaginary,
            |real, imaginary| FftPlan::new(length)?.transform(real, imaginary, direction),
        )
    }
}

pub(super) unsafe fn inverse_dft<T: Scalar + RefUnwindSafe>(
    length: usize,
    real: *const T,
    imaginary: *const T,
    output_real: *mut T,
    output_imaginary: *mut T,
) -> u32 {
    // SAFETY: Forward initialized input and writable caller output contracts.
    unsafe {
        transform(
            real,
            imaginary,
            length,
            output_real,
            output_imaginary,
            |real, imaginary| crate::fourier::inverse_dft(length, real, imaginary),
        )
    }
}

pub(super) unsafe fn new<H: Handle>(length: usize, output: *mut *mut H) -> u32 {
    if let Err(status) = range(output.cast_const(), 1) {
        return status;
    }
    match boundary(|| H::new(length).map(Box::new)) {
        Ok(plan) => {
            // SAFETY: Caller supplies an independently writable pointer slot.
            unsafe { output.write(Box::into_raw(plan)) };
            0
        }
        Err(status) => status,
    }
}

pub(super) unsafe fn plan_length<H: Handle>(handle: *const H, output: *mut usize) -> u32 {
    if let Err(status) = range(handle, 1).and_then(|()| range(output.cast_const(), 1)) {
        return status;
    }
    // SAFETY: Caller holds a live, correctly typed plan and independent output.
    unsafe { output.write((&*handle).length()) };
    0
}

pub(super) unsafe fn process<H: Handle>(
    handle: *const H,
    real: *const H::Value,
    imaginary: *const H::Value,
    length: usize,
    mode: u32,
    output_real: *mut H::Value,
    output_imaginary: *mut H::Value,
) -> u32 {
    let direction = match direction(mode) {
        Ok(direction) => direction,
        Err(status) => return status,
    };
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Caller holds the live plan through this call and any concurrent
    // transforms. Only the internally synchronized scratch buffer is mutable.
    let plan = unsafe { &*handle };
    if length != plan.length() {
        return 2;
    }
    // SAFETY: Forward caller buffer contracts; the plan mutex is acquired inside
    // the unwind boundary. Poisoning is an error, not a memory-safety assumption.
    unsafe {
        transform(
            real,
            imaginary,
            length,
            output_real,
            output_imaginary,
            |real, imaginary| {
                plan.plan()
                    .lock()
                    .map_err(|_| Error("FFT plan is poisoned"))?
                    .transform(real, imaginary, direction)
            },
        )
    }
}

pub(super) unsafe fn release<H: Handle>(slot: *mut *mut H) -> u32 {
    if let Err(status) = range(slot.cast_const(), 1) {
        return status;
    }
    // SAFETY: Caller grants an initialized, exclusively writable pointer slot.
    let handle = unsafe { slot.read() };
    if handle.is_null() {
        return 0;
    }
    if let Err(status) = range(handle.cast_const(), 1) {
        return status;
    }
    // SAFETY: Caller transfers sole ownership and excludes outstanding plan uses.
    unsafe { slot.write(std::ptr::null_mut()) };
    match boundary(|| {
        // SAFETY: This allocation originated from this library's Box::into_raw
        // for the same type and is reconstructed exactly once for destruction.
        unsafe { drop(Box::from_raw(handle)) };
        Ok(())
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn poisoned_plan_rejects_reuse_without_writing_output() {
        let mut slot = std::ptr::null_mut();
        let mut output = [71.0_f64; 4];
        // SAFETY: This correctly typed handle is owned locally and released only
        // after all temporary borrows and the deliberate poisoned-lock test end.
        unsafe {
            assert_eq!(super::new::<super::CigletRsFftF64>(4, &mut slot), 0);
            let plan = &*slot;
            let result = std::panic::catch_unwind(|| {
                let _guard = plan.plan.lock().unwrap();
                panic!("deliberate FFT scratch panic");
            });
            assert!(result.is_err());
            assert!(plan.plan.is_poisoned());
            assert_eq!(
                super::process(
                    slot,
                    std::ptr::null(),
                    std::ptr::null(),
                    4,
                    0,
                    output.as_mut_ptr(),
                    std::ptr::null_mut()
                ),
                3
            );
            assert_eq!(output, [71.0; 4]);
            assert_eq!(super::release(&mut slot), 0);
            assert!(slot.is_null());
        }
    }
}

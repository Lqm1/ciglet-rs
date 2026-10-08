//! Chirp-Z buffers and reusable synchronized plans.
use super::{boundary, fourier, range};
use crate::{Error, chirp_z::CztPlan, fourier::Direction, statistics::Scalar};
use std::{
    panic::{RefUnwindSafe, UnwindSafe},
    sync::Mutex,
};

/// Opaque binary32 chirp-Z plan with a fixed grid and direction.
pub struct CigletRsCztF32 {
    length: usize,
    plan: Mutex<CztPlan<f32>>,
}
/// Opaque binary64 chirp-Z plan with a fixed grid and direction.
pub struct CigletRsCztF64 {
    length: usize,
    plan: Mutex<CztPlan<f64>>,
}

pub(super) trait Handle: RefUnwindSafe {
    type Value: Scalar + RefUnwindSafe + UnwindSafe;
    fn new(length: usize, step: Self::Value, direction: Direction) -> Result<Self, Error>
    where
        Self: Sized;
    fn length(&self) -> usize;
    fn plan(&self) -> &Mutex<CztPlan<Self::Value>>;
}

impl Handle for CigletRsCztF32 {
    type Value = f32;
    fn new(length: usize, step: f32, direction: Direction) -> Result<Self, Error> {
        Ok(Self {
            length,
            plan: Mutex::new(CztPlan::new(length, step, direction)?),
        })
    }
    fn length(&self) -> usize {
        self.length
    }
    fn plan(&self) -> &Mutex<CztPlan<f32>> {
        &self.plan
    }
}

impl Handle for CigletRsCztF64 {
    type Value = f64;
    fn new(length: usize, step: f64, direction: Direction) -> Result<Self, Error> {
        Ok(Self {
            length,
            plan: Mutex::new(CztPlan::new(length, step, direction)?),
        })
    }
    fn length(&self) -> usize {
        self.length
    }
    fn plan(&self) -> &Mutex<CztPlan<f64>> {
        &self.plan
    }
}

pub(super) unsafe fn one_shot<T: Scalar + RefUnwindSafe>(
    length: usize,
    step: T,
    mode: u32,
    real: *const T,
    imaginary: *const T,
    output_real: *mut T,
    output_imaginary: *mut T,
) -> u32 {
    let direction = match fourier::direction(mode) {
        Ok(direction) => direction,
        Err(status) => return status,
    };
    // SAFETY: Forward the caller's initialized-input and writable-output contracts.
    unsafe {
        fourier::transform(
            real,
            imaginary,
            length,
            output_real,
            output_imaginary,
            |real, imaginary| CztPlan::new(length, step, direction)?.transform(real, imaginary),
        )
    }
}
pub(super) unsafe fn new<H: Handle>(
    length: usize,
    step: H::Value,
    mode: u32,
    output: *mut *mut H,
) -> u32 {
    if let Err(status) = range(output.cast_const(), 1) {
        return status;
    }
    let direction = match fourier::direction(mode) {
        Ok(direction) => direction,
        Err(status) => return status,
    };
    match boundary(|| H::new(length, step, direction).map(Box::new)) {
        Ok(plan) => {
            // SAFETY: Caller supplies an independent exclusively writable pointer slot.
            unsafe { output.write(Box::into_raw(plan)) };
            0
        }
        Err(status) => status,
    }
}
pub(super) unsafe fn length<H: Handle>(handle: *const H, output: *mut usize) -> u32 {
    if let Err(status) = range(handle, 1).and_then(|()| range(output.cast_const(), 1)) {
        return status;
    }
    // SAFETY: Correctly typed live handle and independent writable scalar.
    unsafe { output.write((&*handle).length()) };
    0
}
pub(super) unsafe fn process<H: Handle>(
    handle: *const H,
    real: *const H::Value,
    imaginary: *const H::Value,
    length: usize,
    output_real: *mut H::Value,
    output_imaginary: *mut H::Value,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Caller keeps a correctly typed plan live through all outstanding calls.
    let plan = unsafe { &*handle };
    if length != plan.length() {
        return 2;
    }
    // SAFETY: Forward caller buffer contracts. The mutex guard is acquired inside
    // the unwind boundary; poison is an error rather than a safety assumption.
    unsafe {
        fourier::transform(
            real,
            imaginary,
            length,
            output_real,
            output_imaginary,
            |real, imaginary| {
                plan.plan()
                    .lock()
                    .map_err(|_| Error("CZT plan is poisoned"))?
                    .transform(real, imaginary)
            },
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn poisoned_czt_plan_preserves_output_and_releases_ownership() {
        let mut slot = std::ptr::null_mut();
        let mut output = [71.0_f64; 3];
        // SAFETY: One locally owned plan stays live until all temporary borrows end.
        unsafe {
            assert_eq!(super::new::<super::CigletRsCztF64>(3, 0.0, 0, &mut slot), 0);
            let plan = &*slot;
            let poisoned = std::panic::catch_unwind(|| {
                let _guard = plan.plan.lock().unwrap();
                panic!("deliberate CZT scratch panic");
            });
            assert!(poisoned.is_err());
            assert!(plan.plan.is_poisoned());
            assert_eq!(
                super::process(
                    slot,
                    std::ptr::null(),
                    std::ptr::null(),
                    3,
                    output.as_mut_ptr(),
                    std::ptr::null_mut()
                ),
                3
            );
            assert_eq!(output, [71.0; 3]);
            assert_eq!(super::fourier::release(&mut slot), 0);
            assert!(slot.is_null());
        }
    }
}

//! Owned instantaneous-frequency detectors with checked atomic configuration.
use super::{boundary, disjoint_outputs, range, scalar_value_result};
use crate::{
    Error,
    instantaneous_frequency::{Detector, Kernels, Policy},
    statistics::Scalar,
};
use std::panic::{RefUnwindSafe, UnwindSafe};
/// Opaque binary32 instantaneous-frequency detector.
pub struct CigletRsIfDetectorF32(Detector<f32>);
/// Opaque binary64 instantaneous-frequency detector.
pub struct CigletRsIfDetectorF64(Detector<f64>);
pub(super) trait Handle: RefUnwindSafe {
    type Value: Scalar + RefUnwindSafe + UnwindSafe;
    fn new(detector: Detector<Self::Value>) -> Self;
    fn detector(&self) -> &Detector<Self::Value>;
    fn detector_mut(&mut self) -> &mut Detector<Self::Value>;
}
impl Handle for CigletRsIfDetectorF32 {
    type Value = f32;
    fn new(detector: Detector<f32>) -> Self {
        Self(detector)
    }
    fn detector(&self) -> &Detector<f32> {
        &self.0
    }
    fn detector_mut(&mut self) -> &mut Detector<f32> {
        &mut self.0
    }
}
impl Handle for CigletRsIfDetectorF64 {
    type Value = f64;
    fn new(detector: Detector<f64>) -> Self {
        Self(detector)
    }
    fn detector(&self) -> &Detector<f64> {
        &self.0
    }
    fn detector_mut(&mut self) -> &mut Detector<f64> {
        &mut self.0
    }
}
fn copied<T: Copy>(input: &[T]) -> Result<Vec<T>, Error> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(input.len())
        .map_err(|_| Error("detector kernel allocation failed"))?;
    values.extend_from_slice(input);
    Ok(values)
}
fn cloned<T: Scalar>(kernels: &Kernels<T>) -> Result<Kernels<T>, Error> {
    Ok(Kernels {
        real: copied(&kernels.real)?,
        imaginary: copied(&kernels.imaginary)?,
        derivative_real: copied(&kernels.derivative_real)?,
        derivative_imaginary: copied(&kernels.derivative_imaginary)?,
    })
}
unsafe fn input<'a, T>(pointer: *const T, count: usize) -> &'a [T] {
    if count == 0 {
        &[]
    } else {
        // SAFETY: Caller retains checked initialized stable readable allocation.
        unsafe { std::slice::from_raw_parts(pointer, count) }
    }
}
unsafe fn kernels<T: Scalar>(planes: [*const T; 4], count: usize) -> Result<Kernels<T>, Error> {
    // SAFETY: All four input ranges were validated and remain initialized/stable.
    unsafe {
        Ok(Kernels {
            real: copied(input(planes[0], count))?,
            imaginary: copied(input(planes[1], count))?,
            derivative_real: copied(input(planes[2], count))?,
            derivative_imaginary: copied(input(planes[3], count))?,
        })
    }
}
pub(super) unsafe fn create<H: Handle>(
    output: *mut *mut H,
    compute: impl FnOnce() -> Result<Detector<H::Value>, Error> + UnwindSafe,
) -> u32 {
    if let Err(status) = range(output.cast_const(), 1) {
        return status;
    }
    match boundary(|| compute().map(H::new).map(Box::new)) {
        Ok(value) => {
            // SAFETY: Independent exclusive initialized ownership slot.
            unsafe {
                output.write(Box::into_raw(value));
            }
            0
        }
        Err(status) => status,
    }
}
pub(super) unsafe fn from_kernels<H: Handle>(
    frequency: H::Value,
    planes: [*const H::Value; 4],
    count: usize,
    output: *mut *mut H,
) -> u32 {
    for plane in planes {
        if let Err(status) = range(plane, count) {
            return status;
        }
    }
    // SAFETY: Checked readable planes and independent writable ownership slot.
    unsafe {
        create(output, || {
            Detector::from_kernels(frequency, kernels(planes, count)?)
        })
    }
}
pub(super) unsafe fn clone_owner<H: Handle>(handle: *const H, output: *mut *mut H) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Retained immutable typed live owner and independent ownership slot.
    unsafe {
        create(output, || {
            let detector = (&*handle).detector();
            Detector::from_kernels(detector.center_frequency(), cloned(detector.kernels())?)
        })
    }
}
pub(super) unsafe fn kernel_count<H: Handle>(handle: *const H, output: *mut usize) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Retained immutable typed owner; exclusive independent scalar output.
    unsafe { scalar_value_result(output, || Ok((&*handle).detector().kernels().real.len())) }
}
pub(super) unsafe fn center<H: Handle>(handle: *const H, output: *mut H::Value) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Retained immutable typed owner; exclusive independent scalar output.
    unsafe { scalar_value_result(output, || Ok((&*handle).detector().center_frequency())) }
}
pub(super) unsafe fn set_center<H: Handle>(handle: *mut H, value: H::Value) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Caller excludes all other access to the retained live typed owner.
    boundary(|| unsafe { (&mut *handle).detector_mut().set_center_frequency(value) })
        .map_or_else(|status| status, |()| 0)
}
pub(super) unsafe fn replace<H: Handle>(
    handle: *mut H,
    planes: [*const H::Value; 4],
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    for plane in planes {
        if let Err(status) = range(plane, count) {
            return status;
        }
    }
    let result = boundary(|| {
        // SAFETY: Exclusive typed live owner; input arrays are checked, initialized,
        // stable and independent of private owner storage. Candidate is fully built.
        unsafe {
            let detector = (&mut *handle).detector_mut();
            let candidate =
                Detector::from_kernels(detector.center_frequency(), kernels(planes, count)?)?;
            *detector = candidate;
        }
        Ok(())
    });
    result.map_or_else(|status| status, |()| 0)
}
pub(super) unsafe fn copy_kernels<H: Handle>(
    handle: *const H,
    planes: [*mut H::Value; 4],
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Retained immutable live owner; caller excludes mutation/release.
    if count != unsafe { &*handle }.detector().kernels().real.len() {
        return 2;
    }
    for (i, &plane) in planes.iter().enumerate() {
        if let Err(status) = range(plane.cast_const(), count) {
            return status;
        }
        for &other in &planes[i + 1..] {
            if let Err(status) =
                disjoint_outputs(plane.cast_const(), count, other.cast_const(), count)
            {
                return status;
            }
        }
    }
    let result = boundary(|| {
        // SAFETY: Immutable typed owner remains live; outputs exclude private storage.
        let kernels = unsafe { &*handle }.detector().kernels();
        let source = [
            &kernels.real,
            &kernels.imaginary,
            &kernels.derivative_real,
            &kernels.derivative_imaginary,
        ];
        if source.iter().any(|values| values.len() != count) {
            return Err(Error("detector kernel lengths differ"));
        }
        // All validation precedes writes. Copying primitive scalars cannot allocate
        // or produce a recoverable error, so no temporary kernel vectors are needed.
        for (values, output) in source.iter().zip(planes) {
            if count != 0 {
                // SAFETY: Checked disjoint exclusive caller ranges, independent of
                // immutable private owner storage. No mutation/release is concurrent.
                unsafe {
                    std::ptr::copy_nonoverlapping(values.as_ptr(), output, count);
                }
            }
        }
        Ok(())
    });
    result.map_or_else(|status| status, |()| 0)
}

pub(super) unsafe fn estimate<H: Handle>(
    handle: *const H,
    samples: *const H::Value,
    count: usize,
    policy: Option<u32>,
    output: *mut H::Value,
) -> u32 {
    if let Err(status) = range(handle, 1).and_then(|()| range(samples, count)) {
        return status;
    }
    // SAFETY: Retained immutable typed owner; checked readable samples; all borrows
    // end before committing independent exclusive scalar output (may overlap samples).
    unsafe {
        scalar_value_result(output, || {
            let detector = (&*handle).detector();
            let samples = input(samples, count);
            match policy {
                None => detector.estimate(samples),
                Some(0) => detector.estimate_with(samples, Policy::Stable),
                Some(1) => detector.estimate_with(samples, Policy::Legacy),
                _ => Err(Error("unknown detector arithmetic policy")),
            }
        })
    }
}

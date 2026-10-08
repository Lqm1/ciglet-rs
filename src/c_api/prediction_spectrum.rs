//! Serialized prediction FFT scratch, paired estimates and full-grid spectra.
use super::{
    boundary, generated_result, prediction::arithmetic, prediction_estimate, prediction_scaled,
    range,
};
use crate::{Error, prediction::SpectrumPlan, statistics::Scalar};
use std::{
    panic::{RefUnwindSafe, UnwindSafe},
    sync::Mutex,
};

/// Opaque reusable binary32 prediction spectrum plan.
pub struct CigletRsPredictionSpectrumF32 {
    length: usize,
    plan: Mutex<SpectrumPlan<f32>>,
}
/// Opaque reusable binary64 prediction spectrum plan.
pub struct CigletRsPredictionSpectrumF64 {
    length: usize,
    plan: Mutex<SpectrumPlan<f64>>,
}
pub(super) trait Handle: RefUnwindSafe {
    type Value: Scalar + RefUnwindSafe + UnwindSafe;
    type Scaled: prediction_scaled::Handle<Value = Self::Value>;
    fn new(length: usize) -> Result<Self, Error>
    where
        Self: Sized;
    fn length(&self) -> usize;
    fn plan(&self) -> &Mutex<SpectrumPlan<Self::Value>>;
}
impl Handle for CigletRsPredictionSpectrumF32 {
    type Value = f32;
    type Scaled = prediction_scaled::CigletRsScaledPredictionF32;
    fn new(length: usize) -> Result<Self, Error> {
        Ok(Self {
            length,
            plan: Mutex::new(SpectrumPlan::new(length)?),
        })
    }
    fn length(&self) -> usize {
        self.length
    }
    fn plan(&self) -> &Mutex<SpectrumPlan<f32>> {
        &self.plan
    }
}
impl Handle for CigletRsPredictionSpectrumF64 {
    type Value = f64;
    type Scaled = prediction_scaled::CigletRsScaledPredictionF64;
    fn new(length: usize) -> Result<Self, Error> {
        Ok(Self {
            length,
            plan: Mutex::new(SpectrumPlan::new(length)?),
        })
    }
    fn length(&self) -> usize {
        self.length
    }
    fn plan(&self) -> &Mutex<SpectrumPlan<f64>> {
        &self.plan
    }
}
pub(super) unsafe fn new<H: Handle>(length: usize, output: *mut *mut H) -> u32 {
    if let Err(status) = range(output.cast_const(), 1) {
        return status;
    }
    match boundary(|| H::new(length).map(Box::new)) {
        Ok(plan) => {
            // SAFETY: Independent initialized aligned exclusively writable owner slot.
            unsafe { output.write(Box::into_raw(plan)) };
            0
        }
        Err(status) => status,
    }
}
pub(super) unsafe fn shape<H: Handle>(handle: *const H, output: *mut usize, count: usize) -> u32 {
    if count != 2 {
        return 2;
    }
    if let Err(status) = range(handle, 1).and_then(|()| range(output.cast_const(), 2)) {
        return status;
    }
    // SAFETY: Correctly typed live owner, retained through the call excluding release.
    let length = unsafe { &*handle }.length();
    // SAFETY: Two exclusively writable elements, independent of owner allocation.
    unsafe {
        output.write(length);
        output.add(1).write(length / 2 + 1);
    }
    0
}
#[derive(Clone, Copy)]
pub(super) struct Estimate {
    pub order: usize,
    pub nyquist: u32,
    pub arithmetic: u32,
}
pub(super) unsafe fn estimate<H: Handle>(
    handle: *const H,
    input: *const H::Value,
    count: usize,
    options: Estimate,
    output: prediction_estimate::Output<H::Value>,
) -> u32 {
    if let Err(status) = range(handle, 1)
        .and_then(|()| range(input, count))
        .and_then(|()| prediction_estimate::outputs(output, options.order))
    {
        return status;
    }
    let nyquist = match prediction_estimate::nyquist(options.nyquist) {
        Ok(value) => value,
        Err(status) => return status,
    };
    let arithmetic = match arithmetic(options.arithmetic) {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Typed live owner retained while computation and input borrowing proceed.
    let plan = unsafe { &*handle };
    let result = boundary(|| {
        let values = if count == 0 {
            &[]
        } else {
            // SAFETY: Caller initialized stable readable single-allocation input.
            unsafe { std::slice::from_raw_parts(input, count) }
        };
        plan.plan()
            .lock()
            .map_err(|_| Error("prediction spectrum plan is poisoned"))?
            .estimate_with_arithmetic(values, options.order, nyquist, arithmetic)
    });
    // SAFETY: Input borrowing and mutex guard ended; both output planes were checked.
    unsafe { prediction_estimate::commit(output, result) }
}
pub(super) unsafe fn estimate_scaled<H: Handle>(
    handle: *const H,
    input: *const H::Value,
    count: usize,
    order: usize,
    nyquist: u32,
    output: *mut *mut H::Scaled,
) -> u32 {
    if let Err(status) = range(handle, 1).and_then(|()| range(input, count)) {
        return status;
    }
    let nyquist = match prediction_estimate::nyquist(nyquist) {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Live retained typed plan; separate writable initialized ownership slot.
    let plan = unsafe { &*handle };
    unsafe {
        prediction_scaled::publish(output, || {
            let values = if count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(input, count)
            };
            plan.plan()
                .lock()
                .map_err(|_| Error("prediction spectrum plan is poisoned"))?
                .estimate_scaled(values, order, nyquist)
        })
    }
}
pub(super) unsafe fn spectrum<H: Handle>(
    handle: *const H,
    input: *const H::Value,
    count: usize,
    gain: H::Value,
    output: *mut H::Value,
    output_count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1).and_then(|()| range(input, count)) {
        return status;
    }
    // SAFETY: Correctly typed live owner retained through all outstanding calls.
    let plan = unsafe { &*handle };
    if output_count != plan.length() {
        return 2;
    }
    // SAFETY: Staged owned output ends input borrowing before committing. Caller
    // supplies independent owner storage and initialized input/writable output.
    unsafe {
        generated_result(output, output_count, || {
            let values = if count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(input, count)
            };
            plan.plan()
                .lock()
                .map_err(|_| Error("prediction spectrum plan is poisoned"))?
                .spectrum(values, gain)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poisoned_scratch_retains_outputs_and_releases_owner() {
        let mut handle = std::ptr::null_mut();
        // SAFETY: Independent initialized writable ownership slot.
        assert_eq!(
            unsafe { new::<CigletRsPredictionSpectrumF64>(4, &mut handle) },
            0
        );
        let panic = std::panic::catch_unwind(|| {
            // SAFETY: Typed owner remains live until all computation ends.
            let _guard = unsafe { &*handle }.plan.lock().unwrap();
            panic!("induce prediction scratch poison for boundary verification");
        });
        assert!(panic.is_err());
        let input = [1.0; 3];
        let mut coefficients = [71.0; 2];
        let mut correlations = [71.0; 2];
        let mut spectrum_output = [71.0; 4];
        let mut shape_output = [71; 2];
        let mut scaled = std::ptr::null_mut();
        // SAFETY: Live correctly typed owner and distinct initialized writable
        // outputs. No caller retains a mutex guard or accesses released ownership.
        unsafe {
            assert_eq!(
                estimate(
                    handle,
                    input.as_ptr(),
                    3,
                    Estimate {
                        order: 1,
                        nyquist: 0,
                        arithmetic: 0
                    },
                    prediction_estimate::Output {
                        coefficients: coefficients.as_mut_ptr(),
                        correlations: correlations.as_mut_ptr(),
                        count: 2
                    }
                ),
                3
            );
            assert_eq!(coefficients, [71.0; 2]);
            assert_eq!(correlations, [71.0; 2]);
            assert_eq!(
                estimate_scaled(handle, input.as_ptr(), 3, 1, 0, &mut scaled),
                3
            );
            assert!(scaled.is_null());
            assert_eq!(
                spectrum(
                    handle,
                    coefficients.as_ptr(),
                    2,
                    1.0,
                    spectrum_output.as_mut_ptr(),
                    4
                ),
                3
            );
            assert_eq!(spectrum_output, [71.0; 4]);
            assert_eq!(shape(handle, shape_output.as_mut_ptr(), 2), 0);
            assert_eq!(shape_output, [4, 3]);
            assert_eq!(super::super::fourier::release(&mut handle), 0);
            assert!(handle.is_null());
        }
    }
}

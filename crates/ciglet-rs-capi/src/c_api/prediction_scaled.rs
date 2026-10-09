//! Owned scaled predictions preserve powers outside the selected scalar range.
use super::{boundary, disjoint_outputs, generated_result, range, scalar_value_result};
use crate::{Error, prediction::ScaledPrediction, statistics::Scalar};
use std::panic::{RefUnwindSafe, UnwindSafe};

/// Opaque immutable binary32 prediction with individually scaled correlations.
pub struct CigletRsScaledPredictionF32 {
    prediction: ScaledPrediction<f32>,
}
/// Opaque immutable binary64 prediction with individually scaled correlations.
pub struct CigletRsScaledPredictionF64 {
    prediction: ScaledPrediction<f64>,
}
pub(super) trait Handle: RefUnwindSafe {
    type Value: Scalar + RefUnwindSafe + UnwindSafe;
    fn owned(prediction: ScaledPrediction<Self::Value>) -> Self;
    fn prediction(&self) -> &ScaledPrediction<Self::Value>;
}
impl Handle for CigletRsScaledPredictionF32 {
    type Value = f32;
    fn owned(prediction: ScaledPrediction<f32>) -> Self {
        Self { prediction }
    }
    fn prediction(&self) -> &ScaledPrediction<f32> {
        &self.prediction
    }
}
impl Handle for CigletRsScaledPredictionF64 {
    type Value = f64;
    fn owned(prediction: ScaledPrediction<f64>) -> Self {
        Self { prediction }
    }
    fn prediction(&self) -> &ScaledPrediction<f64> {
        &self.prediction
    }
}

/// Publish ownership only after all computation and input borrowing succeeds.
pub(super) unsafe fn publish<H: Handle>(
    output: *mut *mut H,
    compute: impl FnOnce() -> Result<ScaledPrediction<H::Value>, Error> + UnwindSafe,
) -> u32 {
    if let Err(status) = range(output.cast_const(), 1) {
        return status;
    }
    match boundary(|| compute().map(|value| Box::new(H::owned(value)))) {
        Ok(value) => {
            // SAFETY: Independent aligned caller slot is exclusively writable;
            // old ownership is retained by the caller before successful replacement.
            unsafe { output.write(Box::into_raw(value)) };
            0
        }
        Err(status) => status,
    }
}
pub(super) unsafe fn samples<H: Handle>(
    input: *const H::Value,
    count: usize,
    order: usize,
    output: *mut *mut H,
) -> u32 {
    if let Err(status) = range(input, count) {
        return status;
    }
    // SAFETY: Caller provides initialized stable input and independent writable slot.
    unsafe {
        publish(output, || {
            let values = if count == 0 {
                &[]
            } else {
                // SAFETY: Checked initialized stable single-allocation sample range.
                std::slice::from_raw_parts(input, count)
            };
            crate::prediction::from_samples_scaled(values, order)
        })
    }
}
pub(super) unsafe fn clone_handle<H: Handle>(input: *const H, output: *mut *mut H) -> u32 {
    if let Err(status) = range(input, 1) {
        return status;
    }
    // SAFETY: Caller retains typed immutable handle, excluding concurrent release.
    unsafe { publish(output, || Ok((&*input).prediction().clone())) }
}
pub(super) unsafe fn length<H: Handle>(input: *const H, output: *mut usize) -> u32 {
    if let Err(status) = range(input, 1) {
        return status;
    }
    // SAFETY: Typed live immutable handle and independent writable scalar.
    unsafe { scalar_value_result(output, || Ok((&*input).prediction().coefficients().len())) }
}
pub(super) unsafe fn coefficients<H: Handle>(
    input: *const H,
    output: *mut H::Value,
    count: usize,
) -> u32 {
    if let Err(status) = range(input, 1) {
        return status;
    }
    // SAFETY: Typed live handle, excluding release; output is independent of owner.
    let prediction = unsafe { &*input }.prediction();
    if count != prediction.coefficients().len() {
        return 2;
    }
    // SAFETY: Checked count; staging ends owner borrow before caller output write.
    unsafe {
        generated_result(output, count, || {
            let mut values = crate::vector::buffer(count)?;
            values.extend_from_slice(prediction.coefficients());
            Ok(values)
        })
    }
}
pub(super) unsafe fn correlation_parts<H: Handle>(
    input: *const H,
    fractions: *mut f64,
    exponents: *mut i32,
    count: usize,
) -> u32 {
    if let Err(status) = range(input, 1).and_then(|()| {
        disjoint_outputs(fractions.cast_const(), count, exponents.cast_const(), count)
    }) {
        return status;
    }
    // SAFETY: Caller retains typed immutable handle, excluding release.
    let prediction = unsafe { &*input }.prediction();
    if count != prediction.coefficients().len() {
        return 2;
    }
    let result = boundary(|| {
        let mut parts = crate::vector::buffer(count)?;
        parts.extend(prediction.autocorrelation_parts());
        Ok(parts)
    });
    match result {
        Ok(parts) => {
            // SAFETY: Pairwise-disjoint initialized writable output ranges,
            // independent of handle allocation; complete values staged before commit.
            unsafe {
                for (index, (fraction, exponent)) in parts.into_iter().enumerate() {
                    fractions.add(index).write(fraction);
                    exponents.add(index).write(exponent);
                }
            }
            0
        }
        Err(status) => status,
    }
}
pub(super) unsafe fn gain_parts<H: Handle>(
    input: *const H,
    fraction: *mut f64,
    exponent: *mut i32,
) -> u32 {
    if let Err(status) = range(input, 1)
        .and_then(|()| disjoint_outputs(fraction.cast_const(), 1, exponent.cast_const(), 1))
    {
        return status;
    }
    match boundary(|| {
        // SAFETY: Caller retains typed immutable handle, excluding release.
        unsafe { &*input }.prediction().gain_parts()
    }) {
        Ok((value, power)) => {
            // SAFETY: Distinct writable scalars, independent of owner allocation.
            unsafe {
                fraction.write(value);
                exponent.write(power);
            }
            0
        }
        Err(status) => status,
    }
}
pub(super) unsafe fn gain<H: Handle>(input: *const H, output: *mut H::Value) -> u32 {
    if let Err(status) = range(input, 1) {
        return status;
    }
    // SAFETY: Live typed immutable handle, independent exclusively writable scalar.
    unsafe { scalar_value_result(output, || (&*input).prediction().gain()) }
}
pub(super) unsafe fn ordinary<H: Handle>(
    input: *const H,
    coefficients: *mut H::Value,
    correlations: *mut H::Value,
    count: usize,
) -> u32 {
    if let Err(status) = range(input, 1).and_then(|()| {
        disjoint_outputs(
            coefficients.cast_const(),
            count,
            correlations.cast_const(),
            count,
        )
    }) {
        return status;
    }
    // SAFETY: Typed immutable live owner; caller excludes release and output aliases.
    let prediction = unsafe { &*input }.prediction();
    if count != prediction.coefficients().len() {
        return 2;
    }
    // Cloning preserves the original successful scaled estimate on any conversion error.
    match boundary(|| prediction.clone().into_prediction()) {
        Ok(value) => {
            // SAFETY: Distinct writable arrays independent of handle allocation.
            // Both owned arrays are complete before the first output commit.
            unsafe {
                std::ptr::copy_nonoverlapping(value.coefficients.as_ptr(), coefficients, count);
                std::ptr::copy_nonoverlapping(value.autocorrelation.as_ptr(), correlations, count);
            }
            0
        }
        Err(status) => status,
    }
}

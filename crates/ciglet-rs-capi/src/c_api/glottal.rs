//! Complete owned LF model configuration and reusable calculation boundary.
use super::{boundary, disjoint_outputs, generated_result, range};
use crate::{
    Error,
    glottal::{Model, Precision, Prepared, Spectrum},
    statistics::Scalar,
};
use std::panic::{RefUnwindSafe, UnwindSafe};

/// Opaque reusable binary32 LF model.
pub struct CigletRsGlottalF32(Prepared<f32>);
/// Opaque reusable binary64 LF model.
pub struct CigletRsGlottalF64(Prepared<f64>);

/// Fast LF spectrum exponential/trigonometric approximation.
pub const CIGLET_RS_GLOTTAL_FAST: u32 = 0;
/// Original LF spectrum exponential/trigonometric approximation.
pub const CIGLET_RS_GLOTTAL_APPROXIMATE: u32 = 1;
/// Host-libm LF spectrum exponential/trigonometric evaluation.
pub const CIGLET_RS_GLOTTAL_EXACT: u32 = 2;

pub(super) trait Handle: RefUnwindSafe {
    type Value: Scalar + RefUnwindSafe + UnwindSafe;
    fn new(value: Prepared<Self::Value>) -> Self;
    fn prepared(&self) -> &Prepared<Self::Value>;
}
impl Handle for CigletRsGlottalF32 {
    type Value = f32;
    fn new(value: Prepared<f32>) -> Self {
        Self(value)
    }
    fn prepared(&self) -> &Prepared<f32> {
        &self.0
    }
}
impl Handle for CigletRsGlottalF64 {
    type Value = f64;
    fn new(value: Prepared<f64>) -> Self {
        Self(value)
    }
    fn prepared(&self) -> &Prepared<f64> {
        &self.0
    }
}

fn model_range<T>(fields: *const T, count: usize) -> Result<(), u32> {
    if count != 5 {
        return Err(2);
    }
    range(fields, count)
}
unsafe fn model<T: Scalar>(fields: *const T) -> Model<T> {
    // SAFETY: Caller retains five checked initialized readable scalar elements.
    unsafe {
        Model {
            period: fields.read(),
            closure: fields.add(1).read(),
            peak: fields.add(2).read(),
            return_time: fields.add(3).read(),
            excitation: fields.add(4).read(),
        }
    }
}
unsafe fn model_result<T: Scalar>(
    output: *mut T,
    count: usize,
    compute: impl FnOnce() -> Result<Model<T>, Error> + UnwindSafe,
) -> u32 {
    if let Err(status) = model_range(output.cast_const(), count) {
        return status;
    }
    match boundary(compute) {
        Ok(value) => {
            let fields = [
                value.period,
                value.closure,
                value.peak,
                value.return_time,
                value.excitation,
            ];
            // SAFETY: Exclusive checked five-element output, independent stack source.
            unsafe {
                std::ptr::copy_nonoverlapping(fields.as_ptr(), output, 5);
            }
            0
        }
        Err(status) => status,
    }
}
pub(super) unsafe fn from_shape<T: Scalar + UnwindSafe>(
    shape: T,
    period: T,
    excitation: T,
    output: *mut T,
    count: usize,
) -> u32 {
    // SAFETY: Forward independent exclusive caller output storage.
    unsafe {
        model_result(output, count, move || {
            Model::from_shape(shape, period, excitation)
        })
    }
}
pub(super) unsafe fn create<H: Handle>(
    fields: *const H::Value,
    count: usize,
    output: *mut *mut H,
) -> u32 {
    if let Err(status) = model_range(fields, count).and_then(|()| range(output.cast_const(), 1)) {
        return status;
    }
    match boundary(|| {
        // SAFETY: Checked readable model configuration remains stable during copying.
        unsafe { model(fields) }.prepare().map(H::new).map(Box::new)
    }) {
        Ok(value) => {
            // SAFETY: Independent initialized exclusive ownership slot.
            unsafe {
                output.write(Box::into_raw(value));
            }
            0
        }
        Err(status) => status,
    }
}
pub(super) unsafe fn clone_owner<H: Handle>(handle: *const H, output: *mut *mut H) -> u32 {
    if let Err(status) = range(handle, 1).and_then(|()| range(output.cast_const(), 1)) {
        return status;
    }
    match boundary(|| {
        // SAFETY: Retained live immutable owner; no concurrent release.
        Ok(Box::new(H::new(unsafe { &*handle }.prepared().clone())))
    }) {
        Ok(value) => {
            // SAFETY: Independent exclusive initialized ownership output slot.
            unsafe {
                output.write(Box::into_raw(value));
            }
            0
        }
        Err(status) => status,
    }
}
pub(super) unsafe fn get_model<H: Handle>(
    handle: *const H,
    output: *mut H::Value,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live immutable owner, independent exclusive checked output storage.
    unsafe { model_result(output, count, || Ok((&*handle).prepared().model())) }
}
fn precision(code: u32) -> Result<Precision, Error> {
    match code {
        CIGLET_RS_GLOTTAL_FAST => Ok(Precision::Fast),
        CIGLET_RS_GLOTTAL_APPROXIMATE => Ok(Precision::Approximate),
        CIGLET_RS_GLOTTAL_EXACT => Ok(Precision::Exact),
        _ => Err(Error("unknown LF spectrum precision")),
    }
}
unsafe fn spectrum_result<T: Scalar + RefUnwindSafe>(
    frequencies: *const T,
    count: usize,
    magnitude: *mut T,
    phase: *mut T,
    output_count: usize,
    compute: impl FnOnce(&[T]) -> Result<Spectrum<T>, Error> + UnwindSafe,
) -> u32 {
    if output_count != count {
        return 2;
    }
    if let Err(status) = range(frequencies, count)
        .and_then(|()| disjoint_outputs(magnitude.cast_const(), count, phase.cast_const(), count))
    {
        return status;
    }
    let result = boundary(|| {
        let values = if count == 0 {
            &[]
        } else {
            // SAFETY: Checked initialized readable frequency storage remains stable.
            unsafe { std::slice::from_raw_parts(frequencies, count) }
        };
        compute(values)
    });
    match result {
        Ok(value) => {
            if value.magnitude.len() != count || value.phase.len() != count {
                return 3;
            }
            if count != 0 {
                // SAFETY: Disjoint exclusive output planes, independent owned source.
                // Input borrowing ended. Both lengths validated before either write.
                unsafe {
                    std::ptr::copy_nonoverlapping(value.magnitude.as_ptr(), magnitude, count);
                    std::ptr::copy_nonoverlapping(value.phase.as_ptr(), phase, count);
                }
            }
            0
        }
        Err(status) => status,
    }
}
pub(super) unsafe fn prepared_spectrum<H: Handle>(
    handle: *const H,
    frequencies: *const H::Value,
    count: usize,
    code: Option<u32>,
    magnitude: *mut H::Value,
    phase: *mut H::Value,
    output_count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live immutable owner and caller-owned checked arrays; no release.
    unsafe {
        spectrum_result(
            frequencies,
            count,
            magnitude,
            phase,
            output_count,
            |values| {
                let prepared = (&*handle).prepared();
                match code {
                    None => prepared.spectrum(values),
                    Some(code) => prepared.spectrum_with(values, precision(code)?),
                }
            },
        )
    }
}
pub(super) unsafe fn model_spectrum<T: Scalar + RefUnwindSafe>(
    fields: *const T,
    field_count: usize,
    frequencies: *const T,
    count: usize,
    magnitude: *mut T,
    phase: *mut T,
    output_count: usize,
) -> u32 {
    if let Err(status) = model_range(fields, field_count) {
        return status;
    }
    // SAFETY: Checked stable model fields, frequencies and independent output planes.
    unsafe {
        spectrum_result(
            frequencies,
            count,
            magnitude,
            phase,
            output_count,
            |values| model(fields).spectrum(values),
        )
    }
}
pub(super) unsafe fn prepared_period<H: Handle>(
    handle: *const H,
    rate: i32,
    count: usize,
    output: *mut H::Value,
    output_count: usize,
) -> u32 {
    if count != output_count {
        return 2;
    }
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live immutable owner; independent checked exclusive output storage.
    unsafe {
        generated_result(output, count, || {
            (&*handle).prepared().sampled_period(rate, count)
        })
    }
}
pub(super) unsafe fn model_period<T: Scalar + RefUnwindSafe>(
    fields: *const T,
    field_count: usize,
    rate: i32,
    count: usize,
    output: *mut T,
    output_count: usize,
) -> u32 {
    if count != output_count {
        return 2;
    }
    if let Err(status) = model_range(fields, field_count) {
        return status;
    }
    // SAFETY: Checked stable fields, output borrows begin only after model copied.
    unsafe { generated_result(output, count, || model(fields).sampled_period(rate, count)) }
}

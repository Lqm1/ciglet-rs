//! Serialized real-cepstrum scratch and staged scalar-plane outputs.
use super::{boundary, generated_result, range};
use crate::{
    Error,
    real_cepstrum::{self, Lifter, NyquistPolicy, Plan},
    statistics::Scalar,
};
use std::{
    panic::{RefUnwindSafe, UnwindSafe},
    sync::Mutex,
};

/// Opaque binary32 real-cepstrum plan with reusable synchronized scratch.
pub struct CigletRsRealCepstrumF32 {
    length: usize,
    plan: Mutex<Plan<f32>>,
}
/// Opaque binary64 real-cepstrum plan with reusable synchronized scratch.
pub struct CigletRsRealCepstrumF64 {
    length: usize,
    plan: Mutex<Plan<f64>>,
}
pub(super) trait Handle: RefUnwindSafe {
    type Value: Scalar + RefUnwindSafe + UnwindSafe;
    fn new(length: usize) -> Result<Self, Error>
    where
        Self: Sized;
    fn length(&self) -> usize;
    fn plan(&self) -> &Mutex<Plan<Self::Value>>;
}
impl Handle for CigletRsRealCepstrumF32 {
    type Value = f32;
    fn new(length: usize) -> Result<Self, Error> {
        Ok(Self {
            length,
            plan: Mutex::new(Plan::new(length)?),
        })
    }
    fn length(&self) -> usize {
        self.length
    }
    fn plan(&self) -> &Mutex<Plan<f32>> {
        &self.plan
    }
}
impl Handle for CigletRsRealCepstrumF64 {
    type Value = f64;
    fn new(length: usize) -> Result<Self, Error> {
        Ok(Self {
            length,
            plan: Mutex::new(Plan::new(length)?),
        })
    }
    fn length(&self) -> usize {
        self.length
    }
    fn plan(&self) -> &Mutex<Plan<f64>> {
        &self.plan
    }
}
fn policy(code: u32) -> Result<NyquistPolicy, u32> {
    match code {
        0 => Ok(NyquistPolicy::Preserve),
        1 => Ok(NyquistPolicy::Legacy),
        _ => Err(3),
    }
}
fn lifter(code: u32) -> Result<Lifter, u32> {
    match code {
        0 => Ok(Lifter::Causal),
        1 => Ok(Lifter::Legacy),
        _ => Err(3),
    }
}
pub(super) unsafe fn new<H: Handle>(length: usize, output: *mut *mut H) -> u32 {
    if let Err(status) = range(output.cast_const(), 1) {
        return status;
    }
    match boundary(|| H::new(length).map(Box::new)) {
        Ok(plan) => {
            // SAFETY: Caller owns an independent aligned writable pointer slot.
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
    // SAFETY: Caller retains a correctly typed live plan, excluding release.
    let length = unsafe { &*handle }.length();
    let shape = [length, length / 2 + 1];
    // SAFETY: Independent output is exclusively writable for two usize values.
    unsafe { std::ptr::copy_nonoverlapping(shape.as_ptr(), output, 2) };
    0
}
#[derive(Clone, Copy)]
pub(super) enum Operation {
    Analyze,
    Reconstruct,
    MinimumPhase(u32),
    ToCepstra(usize),
    ToSpectrogram { frames: usize, coefficients: usize },
}
pub(super) unsafe fn process<H: Handle>(
    handle: *const H,
    input: *const H::Value,
    input_count: usize,
    mode: u32,
    output: *mut H::Value,
    output_count: usize,
    operation: Operation,
) -> u32 {
    if let Err(status) = range(handle, 1).and_then(|()| range(input, input_count)) {
        return status;
    }
    let policy = match policy(mode) {
        Ok(value) => value,
        Err(status) => return status,
    };
    let lift = if let Operation::MinimumPhase(code) = operation {
        match lifter(code) {
            Ok(value) => value,
            Err(status) => return status,
        }
    } else {
        Lifter::Causal
    };
    // SAFETY: Caller keeps a live typed plan through all outstanding calls.
    let handle = unsafe { &*handle };
    let length = handle.length();
    let bins = length / 2 + 1;
    let expected = match operation {
        Operation::Analyze | Operation::MinimumPhase(_) => {
            if input_count != bins {
                return 3;
            }
            length
        }
        Operation::Reconstruct => {
            if input_count > length {
                return 3;
            }
            length
        }
        Operation::ToCepstra(count) => {
            if count > length || !input_count.is_multiple_of(bins) {
                return 3;
            }
            match (input_count / bins).checked_mul(count) {
                Some(n) => n,
                None => return 2,
            }
        }
        Operation::ToSpectrogram {
            frames,
            coefficients,
        } => {
            if coefficients > length || frames.checked_mul(coefficients) != Some(input_count) {
                return 3;
            }
            match frames.checked_mul(bins) {
                Some(n) => n,
                None => return 2,
            }
        }
    };
    if output_count != expected {
        return 2;
    }
    // SAFETY: Caller input is initialized and stable, including while waiting
    // for scratch; output is writable and independent of handle-owned memory.
    // Staged results end input borrowing before output commit, permitting overlap.
    unsafe {
        generated_result(output, output_count, || {
            let input = if input_count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(input, input_count)
            };
            let mut plan = handle
                .plan()
                .lock()
                .map_err(|_| Error("real cepstrum plan is poisoned"))?;
            match operation {
                Operation::Analyze => plan.analyze(input, policy),
                Operation::Reconstruct => plan.reconstruct(input, policy),
                Operation::MinimumPhase(_) => plan.minimum_phase_with(input, policy, lift),
                Operation::ToCepstra(count) => plan.spectrogram_to_cepstra(input, count, policy),
                Operation::ToSpectrogram {
                    frames,
                    coefficients,
                } => plan.cepstra_to_spectrogram(input, frames, coefficients, policy),
            }
        })
    }
}
#[derive(Clone, Copy)]
pub(super) enum OneShot {
    Analyze,
    Reconstruct,
    MinimumPhase,
}
pub(super) unsafe fn one_shot<T: Scalar + RefUnwindSafe>(
    input: *const T,
    input_count: usize,
    output: *mut T,
    output_count: usize,
    operation: OneShot,
) -> u32 {
    if let Err(status) = range(input, input_count) {
        return status;
    }
    if !matches!(operation, OneShot::Reconstruct) {
        let Some(expected) = input_count
            .checked_sub(1)
            .filter(|&n| n > 0)
            .and_then(|n| n.checked_mul(2))
        else {
            return 3;
        };
        if output_count != expected {
            return 2;
        }
    }
    // SAFETY: Forward initialized stable input and writable caller output contracts.
    unsafe {
        generated_result(output, output_count, || {
            let input = if input_count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(input, input_count)
            };
            match operation {
                OneShot::Analyze => real_cepstrum::from_log_spectrum(input),
                OneShot::Reconstruct => real_cepstrum::to_log_spectrum(input, output_count),
                OneShot::MinimumPhase => real_cepstrum::minimum_phase(input),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn poisoned_scratch_retains_output_and_releases() {
        let mut slot = std::ptr::null_mut();
        // SAFETY: Locally owned typed handle remains live until all borrows end.
        unsafe {
            assert_eq!(
                super::new::<super::CigletRsRealCepstrumF64>(4, &mut slot),
                0
            );
            let poison = std::panic::catch_unwind(|| {
                let _guard = (*slot).plan.lock().unwrap();
                panic!("deliberate real cepstrum scratch panic");
            });
            assert!(poison.is_err());
            let mut output = [71.0; 4];
            let input = [0.0; 3];
            assert_eq!(
                super::process(
                    slot,
                    input.as_ptr(),
                    3,
                    0,
                    output.as_mut_ptr(),
                    4,
                    super::Operation::Analyze
                ),
                3
            );
            assert_eq!(output, [71.0; 4]);
            assert_eq!(super::super::fourier::release(&mut slot), 0);
            assert!(slot.is_null());
        }
    }
}

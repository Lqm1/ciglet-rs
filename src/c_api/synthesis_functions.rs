//! Centered sinusoid synthesis with staged scalar buffers.
use super::{generated_result, range};
use crate::{
    Error,
    statistics::Scalar,
    synthesis::{self, Sinusoid},
};
use std::panic::RefUnwindSafe;

#[allow(clippy::too_many_arguments)]
unsafe fn multiple<T: Scalar + RefUnwindSafe>(
    frequencies: *const T,
    amplitudes: *const T,
    phases: *const T,
    tone_count: usize,
    sample_rate: u32,
    length: usize,
    output: *mut T,
    count: usize,
) -> u32 {
    if length != count {
        return 2;
    }
    if let Err(status) = range(frequencies, tone_count)
        .and_then(|()| range(amplitudes, tone_count))
        .and_then(|()| range(phases, tone_count))
    {
        return status;
    }
    // SAFETY: Caller retains stable initialized checked input allocations. All
    // borrowing ends before staged output commit, permitting input/output overlap.
    unsafe {
        generated_result(output, count, || {
            let mut tones = Vec::new();
            tones
                .try_reserve_exact(tone_count)
                .map_err(|_| Error("sinusoid parameter allocation failed"))?;
            if tone_count != 0 {
                let frequencies = std::slice::from_raw_parts(frequencies, tone_count);
                let amplitudes = std::slice::from_raw_parts(amplitudes, tone_count);
                let phases = std::slice::from_raw_parts(phases, tone_count);
                tones.extend(frequencies.iter().zip(amplitudes).zip(phases).map(
                    |((&frequency_hz, &amplitude), &phase_radians)| Sinusoid {
                        frequency_hz,
                        amplitude,
                        phase_radians,
                    },
                ));
            }
            synthesis::sinusoids(&tones, sample_rate, length)
        })
    }
}

/// Synthesize one sinusoid with phase centered at floor(length / 2).
/// Frequencies and amplitudes may be negative. Native amplitude and short-length
/// corrections apply. Output count must equal length. Native failures preserve
/// output. Allocation, panic and pointer-validity limitations are unchanged.
///
/// # Safety
/// Output is aligned exclusively writable single-allocation storage for count
/// scalars; null is allowed for zero count.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_sinusoid_f32(
    frequency: f32,
    amplitude: f32,
    phase: f32,
    sample_rate: u32,
    length: usize,
    output: *mut f32,
    count: usize,
) -> u32 {
    if length != count {
        return 2;
    }
    // SAFETY: Caller provides exclusive output; native result is fully staged.
    unsafe {
        generated_result(output, count, || {
            synthesis::sinusoid(
                Sinusoid {
                    frequency_hz: frequency,
                    amplitude,
                    phase_radians: phase,
                },
                sample_rate,
                length,
            )
        })
    }
}

/// Sum centered sinusoids in parameter order using native scalar recurrence.
/// Parameter arrays share tone_count. Empty tone lists produce silence; native
/// sample-rate and finite-parameter checks still apply for empty output. Output
/// count equals length. Complete staging permits parameter/output overlap and
/// preserves output on failure. Native allocation/panic limits remain unchanged.
///
/// # Safety
/// Each input is aligned initialized stable readable single-allocation storage
/// for tone_count scalars; output is exclusively writable for count scalars.
/// Null pointers are accepted only for zero associated counts. Data inputs may
/// overlap one another or output; caller mutation waits until the call returns.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_sinusoids_f32(
    frequencies: *const f32,
    amplitudes: *const f32,
    phases: *const f32,
    tone_count: usize,
    sample_rate: u32,
    length: usize,
    output: *mut f32,
    count: usize,
) -> u32 {
    // SAFETY: Forward checked initialized readable inputs and staged exclusive output.
    unsafe {
        multiple(
            frequencies,
            amplitudes,
            phases,
            tone_count,
            sample_rate,
            length,
            output,
            count,
        )
    }
}

/// Synthesize one sinusoid with phase centered at floor(length / 2).
/// Frequencies and amplitudes may be negative. Native amplitude and short-length
/// corrections apply. Output count must equal length. Native failures preserve
/// output. Allocation, panic and pointer-validity limitations are unchanged.
///
/// # Safety
/// Output is aligned exclusively writable single-allocation storage for count
/// scalars; null is allowed for zero count.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_sinusoid_f64(
    frequency: f64,
    amplitude: f64,
    phase: f64,
    sample_rate: u32,
    length: usize,
    output: *mut f64,
    count: usize,
) -> u32 {
    if length != count {
        return 2;
    }
    // SAFETY: Caller provides exclusive output; native result is fully staged.
    unsafe {
        generated_result(output, count, || {
            synthesis::sinusoid(
                Sinusoid {
                    frequency_hz: frequency,
                    amplitude,
                    phase_radians: phase,
                },
                sample_rate,
                length,
            )
        })
    }
}

/// Sum centered sinusoids in parameter order using native scalar recurrence.
/// Parameter arrays share tone_count. Empty tone lists produce silence; native
/// sample-rate and finite-parameter checks still apply for empty output. Output
/// count equals length. Complete staging permits parameter/output overlap and
/// preserves output on failure. Native allocation/panic limits remain unchanged.
///
/// # Safety
/// Each input is aligned initialized stable readable single-allocation storage
/// for tone_count scalars; output is exclusively writable for count scalars.
/// Null pointers are accepted only for zero associated counts. Data inputs may
/// overlap one another or output; caller mutation waits until the call returns.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_sinusoids_f64(
    frequencies: *const f64,
    amplitudes: *const f64,
    phases: *const f64,
    tone_count: usize,
    sample_rate: u32,
    length: usize,
    output: *mut f64,
    count: usize,
) -> u32 {
    // SAFETY: Forward checked initialized readable inputs and staged exclusive output.
    unsafe {
        multiple(
            frequencies,
            amplitudes,
            phases,
            tone_count,
            sample_rate,
            length,
            output,
            count,
        )
    }
}

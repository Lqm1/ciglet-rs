//! Synchronized caller-owned random sampling and original draw transformations.
use super::{range, scalar_value_result};
use crate::{
    Error,
    random::{self, Generator, Policy},
};
use std::{panic::UnwindSafe, sync::Mutex};

/// Opaque seeded ChaCha8 sampler. Calls serialize access to its owned state.
pub struct CigletRsRandom {
    state: Mutex<Generator>,
}
impl CigletRsRandom {
    pub(super) fn generator(&self) -> &Mutex<Generator> {
        &self.state
    }
}

/// Stable Gaussian calculation; zero first draws are retried when sampling.
pub const CIGLET_RS_RANDOM_STABLE: u32 = 0;
/// Original approximate Gaussian arithmetic, including the zero-draw artifact.
pub const CIGLET_RS_RANDOM_LEGACY: u32 = 1;

pub(super) fn policy(mode: u32) -> Result<Policy, Error> {
    match mode {
        0 => Ok(Policy::Stable),
        1 => Ok(Policy::Legacy),
        _ => Err(Error("unknown Gaussian policy")),
    }
}

pub(super) unsafe fn sample<T: Copy>(
    handle: *const CigletRsRandom,
    output: *mut T,
    compute: impl FnOnce(&mut Generator) -> Result<T, Error> + UnwindSafe,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Caller retains a live immutable owner; its internal state is locked.
    unsafe {
        scalar_value_result(output, || {
            let mut state = (&*handle)
                .state
                .lock()
                .map_err(|_| Error("random state mutex is poisoned"))?;
            compute(&mut state)
        })
    }
}

/// Create a default-grid sampler from exactly 32 seed bytes, without OS entropy.
/// Output is unchanged on error. Successful creation transfers ownership.
///
/// # Safety
/// Seed is stable initialized storage for seed_count bytes. Output is aligned
/// exclusively writable pointer-slot storage; caller must release any old owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_random_create(
    seed: *const u8,
    seed_count: usize,
    output: *mut *mut CigletRsRandom,
) -> u32 {
    // SAFETY: Forward seed and output ownership-slot contracts.
    unsafe { ciglet_rs_random_create_with_maximum(seed, seed_count, i32::MAX as u32, output) }
}

/// Create a seeded sampler with inclusive integer grid maximum in 1..=i32::MAX.
/// The portable ChaCha8 stream is distinct from platform-dependent C rand.
///
/// # Safety
/// Seed is initialized stable storage for exactly 32 bytes. Output is an aligned
/// exclusively writable pointer slot, independent of input, for a new owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_random_create_with_maximum(
    seed: *const u8,
    seed_count: usize,
    maximum: u32,
    output: *mut *mut CigletRsRandom,
) -> u32 {
    if seed_count != 32 {
        return 2;
    }
    if let Err(status) = range(seed, seed_count) {
        return status;
    }
    // SAFETY: Borrowed seed ends before publication to the exclusive output slot.
    unsafe {
        scalar_value_result(output, || {
            let bytes: [u8; 32] = std::slice::from_raw_parts(seed, seed_count)
                .try_into()
                .map_err(|_| Error("seed length"))?;
            let state = random::Sampler::new(Generator::from_seed(bytes).into_rng(), maximum)?;
            Ok(Box::into_raw(Box::new(CigletRsRandom {
                state: Mutex::new(state),
            })))
        })
    }
}

/// Clone the current continuation into an independent owned sampler.
///
/// # Safety
/// Handle is a live retained owner. Output is aligned exclusively writable
/// pointer-slot storage independent of the owner, for a new allocation.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_random_clone(
    handle: *const CigletRsRandom,
    output: *mut *mut CigletRsRandom,
) -> u32 {
    // SAFETY: Live owner is locked while cloning; output is independently writable.
    unsafe {
        sample(handle, output, |state| {
            Ok(Box::into_raw(Box::new(CigletRsRandom {
                state: Mutex::new(state.clone()),
            })))
        })
    }
}

/// Release an owner and clear its pointer slot. A null owner is already released.
///
/// # Safety
/// Slot is aligned initialized exclusively writable pointer storage. Its nonnull
/// owner was created here and is exclusively transferred, with no outstanding calls.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_random_release(slot: *mut *mut CigletRsRandom) -> u32 {
    if let Err(status) = range(slot.cast_const(), 1) {
        return status;
    }
    // SAFETY: Caller supplies an initialized ownership slot and transfers its owner.
    unsafe {
        let handle = slot.read();
        if handle.is_null() {
            return 0;
        }
        if let Err(status) = range(handle.cast_const(), 1) {
            return status;
        }
        slot.write(std::ptr::null_mut());
        drop(Box::from_raw(handle));
    }
    0
}

/// Read the inclusive integer draw-grid maximum without consuming a draw.
///
/// # Safety
/// Handle is live and retained; output is aligned exclusively writable scalar storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_random_maximum(
    handle: *const CigletRsRandom,
    output: *mut u32,
) -> u32 {
    // SAFETY: Forward live owner and independent output contracts.
    unsafe { sample(handle, output, |state| Ok(state.maximum())) }
}

/// Consume one unbiased inclusive grid draw. Invalid output consumes none.
///
/// # Safety
/// Handle is live and retained; output is aligned exclusively writable scalar storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_random_draw(
    handle: *const CigletRsRandom,
    output: *mut u32,
) -> u32 {
    // SAFETY: Forward live owner and independent output contracts.
    unsafe { sample(handle, output, |state| Ok(state.draw())) }
}

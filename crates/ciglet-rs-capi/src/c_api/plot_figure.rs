//! Native Figure ownership, serialized stream operations and lossless exit states.
use super::{
    boundary,
    byte_payload::CigletRsBytes,
    plot_process_options::{CigletRsPlotProcessOptions, os_string, prepare},
    range, scalar_value_result,
    wave_path::CigletRsPath,
};
use crate::{Error, plotting::Figure};
use std::{panic::UnwindSafe, process::ExitStatus, sync::Mutex};

/// Opaque native process/stream owner. Operations serialize through its mutex.
/// Release/close require exclusive ownership and invalidate every alias.
pub struct CigletRsFigure {
    state: Mutex<Option<Figure>>,
}
/// Complete process exit state. code is meaningful only when has_code is one.
/// raw_format zero means Unix wait status; one means Windows exit code bits.
/// Nonzero child exit is an ordinary successful close, not an ABI error.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CigletRsPlotExitStatus {
    pub success: u32,
    pub has_code: u32,
    pub code: i32,
    pub raw_status: u32,
    pub raw_format: u32,
}
fn status(value: ExitStatus) -> CigletRsPlotExitStatus {
    #[cfg(unix)]
    let (raw_status, raw_format) = {
        use std::os::unix::process::ExitStatusExt;
        (value.into_raw() as u32, 0)
    };
    #[cfg(windows)]
    let (raw_status, raw_format) = (
        value.code().expect("Windows exit code is always present") as u32,
        1,
    );
    CigletRsPlotExitStatus {
        success: u32::from(value.success()),
        has_code: u32::from(value.code().is_some()),
        code: value.code().unwrap_or(0),
        raw_status,
        raw_format,
    }
}

#[cfg(all(test, windows))]
mod tests {
    #[test]
    fn windows_raw_exit_code_retains_all_bits() {
        use std::os::windows::process::ExitStatusExt;
        for raw in [0, 7, 0x8000_0007, u32::MAX] {
            let value = super::status(std::process::ExitStatus::from_raw(raw));
            assert_eq!(value.raw_status, raw);
            assert_eq!(value.raw_format, 1);
            assert_eq!(value.has_code, 1);
            assert_eq!(value.code as u32, raw);
            assert_eq!(value.success, u32::from(raw == 0));
        }
    }
}
pub(super) unsafe fn with<R>(
    handle: *const CigletRsFigure,
    operation: impl FnOnce(&mut Figure) -> Result<R, Error> + UnwindSafe,
) -> Result<R, u32> {
    range(handle, 1)?;
    boundary(|| {
        // SAFETY: Live owner is retained, with no conflicting close/release.
        let mut state = unsafe { &*handle }
            .state
            .lock()
            .map_err(|_| Error("plot mutex is poisoned"))?;
        operation(state.as_mut().ok_or(Error("plot figure is closed"))?)
    })
}
unsafe fn create(
    output: *mut *mut CigletRsFigure,
    operation: impl FnOnce() -> Result<Figure, Error> + UnwindSafe,
) -> u32 {
    // SAFETY: Independent writable ownership slot; publication happens after successful spawn.
    unsafe {
        scalar_value_result(output, || {
            operation().map(|figure| {
                Box::into_raw(Box::new(CigletRsFigure {
                    state: Mutex::new(Some(figure)),
                }))
            })
        })
    }
}
/// Launch the native platform default Gnuplot with -p. Output is unchanged on error.
///
/// # Safety
/// Output is independent aligned exclusively writable owner-slot storage. Release old owners first.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_figure_open(output: *mut *mut CigletRsFigure) -> u32 {
    // SAFETY: Forward exclusive output ownership and publication contract.
    unsafe {
        create(output, || {
            Figure::open().map_err(|_| Error("default plot spawn failed"))
        })
    }
}
/// Launch a counted executable directly with -p, preserving native OS names.
///
/// # Safety
/// Program bytes are stable initialized counted storage; output is an independent exclusive owner slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_figure_open_with(
    program: CigletRsPath,
    output: *mut *mut CigletRsFigure,
) -> u32 {
    if let Err(error) = range(output.cast_const(), 1) {
        return error;
    }
    // SAFETY: Retained readable counted program bytes.
    let program = match boundary(|| Ok(unsafe { os_string(program) })).and_then(|v| v) {
        Ok(v) => v,
        Err(e) => return e,
    };
    if program.is_empty() {
        return 3;
    }
    // SAFETY: The owned OS string remains valid and output is independent.
    unsafe {
        create(output, || {
            Figure::open_with(program).map_err(|_| Error("plot spawn failed"))
        })
    }
}
/// Launch custom arguments/environment/output routing. All configuration is validated before files open.
/// Spawn/I/O failures can leave redirected files created/truncated; output owner is unchanged on error.
///
/// # Safety
/// Options, nested descriptor tables and bytes are stable readable independent storage;
/// output is an independent aligned exclusive owner slot. No pointers are retained.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_figure_spawn(
    options: CigletRsPlotProcessOptions,
    output: *mut *mut CigletRsFigure,
) -> u32 {
    if let Err(error) = range(output.cast_const(), 1) {
        return error;
    }
    // SAFETY: All caller configuration and nested counted inputs remain stable.
    let process = match boundary(|| Ok(unsafe { prepare(options) })).and_then(|v| v) {
        Ok(v) => v,
        Err(e) => return e,
    };
    // SAFETY: Prepared data is owned; output is independent exclusive storage.
    unsafe {
        create(output, || {
            Figure::spawn(&mut process.command()?).map_err(|_| Error("custom plot spawn failed"))
        })
    }
}
/// Obtain the native child process id; owner remains retained.
///
/// # Safety
/// Figure is a live retained owner; output is independent aligned exclusive u32 storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_figure_id(
    figure: *const CigletRsFigure,
    output: *mut u32,
) -> u32 {
    if let Err(e) = range(output.cast_const(), 1) {
        return e;
    }
    // SAFETY: Live owner and writable independent output.
    match unsafe { with(figure, |v| Ok(v.id())) } {
        Ok(id) => {
            unsafe { output.write(id) };
            0
        }
        Err(e) => e,
    }
}
unsafe fn take(slot: *mut *mut CigletRsFigure) -> Result<Box<CigletRsFigure>, u32> {
    range(slot.cast_const(), 1)?;
    // SAFETY: Slot is initialized exclusive storage containing a live owner or null.
    unsafe {
        let figure = *slot;
        range(figure, 1)?;
        slot.write(std::ptr::null_mut());
        Ok(Box::from_raw(figure))
    }
}
unsafe fn finish(
    slot: *mut *mut CigletRsFigure,
    output: *mut CigletRsPlotExitStatus,
    captures: Option<(*mut *mut CigletRsBytes, *mut *mut CigletRsBytes)>,
) -> u32 {
    if let Err(e) = range(output.cast_const(), 1) {
        return e;
    }
    if let Some((stdout, stderr)) = captures
        && let Err(e) = range(stdout.cast_const(), 1).and_then(|()| range(stderr.cast_const(), 1))
    {
        return e;
    }
    // SAFETY: Exclusive initialized owner slot. Validate all outputs before consuming.
    let owner = match unsafe { take(slot) } {
        Ok(v) => v,
        Err(e) => return e,
    };
    let result = boundary(|| {
        let figure = owner
            .state
            .into_inner()
            .map_err(|_| Error("plot mutex is poisoned"))?
            .ok_or(Error("plot figure is closed"))?;
        let result = figure
            .close_with_output()
            .map_err(|_| Error("plot close failed"))?;
        let state = status(result.status);
        let captures = captures.map(|_| {
            (
                Box::new(CigletRsBytes(result.stdout)),
                Box::new(CigletRsBytes(result.stderr)),
            )
        });
        Ok((state, captures))
    });
    match result {
        Ok((state, bytes)) => {
            // SAFETY: All independently writable outputs were validated before closing; publication cannot fail.
            unsafe {
                if let (Some((stdout, stderr)), Some((a, b))) = (captures, bytes) {
                    stdout.write(Box::into_raw(a));
                    stderr.write(Box::into_raw(b));
                }
                output.write(state);
            }
            0
        }
        Err(e) => e,
    }
}
/// Flush, close stdin and wait. Consumes/clears the owner even on I/O failure.
/// Invalid output pointers do not consume the figure. Exit status is published only on success.
/// The selected child can keep this call blocked after EOF.
///
/// # Safety
/// Slot exclusively contains a live owner; status is independent aligned exclusive storage.
/// No concurrent operations or retained handle use after consumption are permitted.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_figure_close(
    slot: *mut *mut CigletRsFigure,
    output: *mut CigletRsPlotExitStatus,
) -> u32 {
    // SAFETY: Forward exclusive owner and independent output contracts.
    unsafe { finish(slot, output, None) }
}
/// Close with complete captured stdout/stderr byte owners and exit status.
/// Noncaptured streams yield empty owners. All outputs are published together on success;
/// I/O failure still consumes the figure but leaves every result output unchanged.
///
/// # Safety
/// Slot exclusively contains a live owner. Status and both owner outputs are aligned,
/// exclusively writable independent storage and must not overlap each other or slot.
/// Existing byte owners must be released first. No figure aliases may be used after close.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_figure_close_output(
    slot: *mut *mut CigletRsFigure,
    output: *mut CigletRsPlotExitStatus,
    stdout_output: *mut *mut CigletRsBytes,
    stderr_output: *mut *mut CigletRsBytes,
) -> u32 {
    // SAFETY: Forward exclusive owner and all independent output contracts.
    unsafe { finish(slot, output, Some((stdout_output, stderr_output))) }
}
/// Drop the figure pipe without killing the child or waiting for completion.
/// Native background reaping continues. An already null slot is a successful no-op.
/// Buffer flushing may perform I/O; use explicit close to observe I/O and exit errors.
///
/// # Safety
/// Slot is initialized exclusive owner storage. Live handle has no concurrent users;
/// release invalidates every alias, clears the slot and does not retain caller pointers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_figure_release(slot: *mut *mut CigletRsFigure) -> u32 {
    if let Err(e) = range(slot.cast_const(), 1) {
        return e;
    }
    // SAFETY: Initialized exclusive slot; null owner is a no-op.
    if unsafe { (*slot).is_null() } {
        return 0;
    }
    // SAFETY: Exclusive live owner is transferred for drop.
    match unsafe { take(slot) } {
        Ok(owner) => match boundary(|| {
            drop(owner);
            Ok(())
        }) {
            Ok(()) => 0,
            Err(e) => e,
        },
        Err(e) => e,
    }
}

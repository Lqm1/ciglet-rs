//! Counted native process configuration; validate before opening redirected files.
use super::{
    range,
    wave_path::{CigletRsPath, path},
};
use crate::Error;
use std::{
    ffi::OsString,
    fs::OpenOptions,
    process::{Command, Stdio},
};

/// Child output inherits the corresponding parent stream.
pub const CIGLET_RS_PLOT_INHERIT: u32 = 0;
/// Child output is discarded.
pub const CIGLET_RS_PLOT_NULL: u32 = 1;
/// Child output is drained concurrently and returned by close_output.
pub const CIGLET_RS_PLOT_CAPTURE: u32 = 2;
/// Child output creates or truncates the specified file.
pub const CIGLET_RS_PLOT_FILE: u32 = 3;
/// Child output creates or appends to the specified file.
pub const CIGLET_RS_PLOT_APPEND: u32 = 4;

/// Counted OS strings use CigletRsPath encodings; null value removes a variable.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CigletRsPlotEnvironment {
    pub key: CigletRsPath,
    pub value: *const CigletRsPath,
}
/// Custom child configuration. Stdin is always the native Figure pipe.
/// Arguments are literal OS strings; no shell or implicit -p argument is added.
/// Null directory retains the parent directory. Environment is inherited unless
/// clear_environment is one; entries then set or remove individual variables.
/// Output mode 3/4 requires a file path; other modes require null file paths.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CigletRsPlotProcessOptions {
    pub program: CigletRsPath,
    pub arguments: *const CigletRsPath,
    pub argument_count: usize,
    pub environment: *const CigletRsPlotEnvironment,
    pub environment_count: usize,
    pub clear_environment: u32,
    pub directory: *const CigletRsPath,
    pub stdout_mode: u32,
    pub stdout_file: *const CigletRsPath,
    pub stderr_mode: u32,
    pub stderr_file: *const CigletRsPath,
}

pub(super) unsafe fn os_string(value: CigletRsPath) -> Result<OsString, u32> {
    range(value.bytes, value.byte_count)?;
    // SAFETY: Caller retains the validated initialized counted byte storage.
    let bytes = if value.byte_count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(value.bytes, value.byte_count) }
    };
    path(bytes, value.encoding)
        .map(|v| v.into_os_string())
        .map_err(|_| 3)
}
unsafe fn optional(value: *const CigletRsPath) -> Result<Option<OsString>, u32> {
    if value.is_null() {
        return Ok(None);
    }
    range(value, 1)?;
    // SAFETY: Caller retains a readable descriptor and its bytes until conversion ends.
    unsafe { os_string(*value).map(Some) }
}
enum Output {
    Inherit,
    Null,
    Capture,
    File(OsString, bool),
}
unsafe fn output(mode: u32, file: *const CigletRsPath) -> Result<Output, u32> {
    match (mode, file.is_null()) {
        (0, true) => Ok(Output::Inherit),
        (1, true) => Ok(Output::Null),
        (2, true) => Ok(Output::Capture),
        (3 | 4, false) => {
            // SAFETY: Forward the stable descriptor and byte storage contracts.
            let file = unsafe { optional(file)? }.ok_or(3_u32)?;
            if file.is_empty() {
                return Err(3);
            }
            Ok(Output::File(file, mode == 4))
        }
        _ => Err(3),
    }
}
impl Output {
    fn open(self) -> Result<Stdio, Error> {
        match self {
            Self::Inherit => Ok(Stdio::inherit()),
            Self::Null => Ok(Stdio::null()),
            Self::Capture => Ok(Stdio::piped()),
            Self::File(path, append) => OpenOptions::new()
                .create(true)
                .write(true)
                .append(append)
                .truncate(!append)
                .open(path)
                .map(Stdio::from)
                .map_err(|_| Error("plot output file opening failed")),
        }
    }
}
pub(super) struct Process {
    program: OsString,
    arguments: Vec<OsString>,
    environment: Vec<(OsString, Option<OsString>)>,
    clear_environment: bool,
    directory: Option<OsString>,
    stdout: Output,
    stderr: Output,
}
pub(super) unsafe fn prepare(options: CigletRsPlotProcessOptions) -> Result<Process, u32> {
    range(options.arguments, options.argument_count)?;
    range(options.environment, options.environment_count)?;
    if options.clear_environment > 1 {
        return Err(3);
    }
    // SAFETY: All descriptors/tables and nested bytes are stable readable caller storage.
    unsafe {
        let program = os_string(options.program)?;
        if program.is_empty() {
            return Err(3);
        }
        let mut arguments = Vec::with_capacity(options.argument_count);
        for i in 0..options.argument_count {
            arguments.push(os_string(*options.arguments.add(i))?);
        }
        let mut environment = Vec::with_capacity(options.environment_count);
        for i in 0..options.environment_count {
            let entry = *options.environment.add(i);
            let key = os_string(entry.key)?;
            // Rust Command rejects empty keys and '=' before creating the child.
            if key.is_empty() || key.as_encoded_bytes().contains(&b'=') {
                return Err(3);
            }
            environment.push((key, optional(entry.value)?));
        }
        let directory = optional(options.directory)?;
        if directory.as_ref().is_some_and(|v| v.is_empty()) {
            return Err(3);
        }
        Ok(Process {
            program,
            arguments,
            environment,
            clear_environment: options.clear_environment != 0,
            directory,
            stdout: output(options.stdout_mode, options.stdout_file)?,
            stderr: output(options.stderr_mode, options.stderr_file)?,
        })
    }
}
impl Process {
    pub(super) fn command(self) -> Result<Command, Error> {
        let mut command = Command::new(self.program);
        command.args(self.arguments);
        if self.clear_environment {
            command.env_clear();
        }
        for (key, value) in self.environment {
            if let Some(value) = value {
                command.env(key, value);
            } else {
                command.env_remove(key);
            }
        }
        if let Some(directory) = self.directory {
            command.current_dir(directory);
        }
        command
            .stdout(self.stdout.open()?)
            .stderr(self.stderr.open()?);
        Ok(command)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn literal_arguments_include_empty_and_unicode_values() {
        use super::*;
        let descriptor = |bytes: &[u8]| CigletRsPath {
            bytes: bytes.as_ptr(),
            byte_count: bytes.len(),
            encoding: 0,
        };
        let arguments = [
            descriptor(b""),
            descriptor("literal \"argument\" $value; \u{1f30d}".as_bytes()),
        ];
        let options = CigletRsPlotProcessOptions {
            program: descriptor(b"unused-program"),
            arguments: arguments.as_ptr(),
            argument_count: 2,
            environment: std::ptr::null(),
            environment_count: 0,
            clear_environment: 0,
            directory: std::ptr::null(),
            stdout_mode: 1,
            stdout_file: std::ptr::null(),
            stderr_mode: 1,
            stderr_file: std::ptr::null(),
        };
        // SAFETY: Descriptors retain their initialized static bytes until command construction ends.
        let command = unsafe { prepare(options) }
            .unwrap_or_else(|e| panic!("prepare failed: {e}"))
            .command()
            .unwrap();
        assert_eq!(
            command.get_args().collect::<Vec<_>>(),
            [
                std::ffi::OsStr::new(""),
                std::ffi::OsStr::new("literal \"argument\" $value; \u{1f30d}")
            ]
        );
    }
}

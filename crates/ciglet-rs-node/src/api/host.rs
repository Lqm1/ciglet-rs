use crate::plotting::Figure as NativeFigure;
use napi::Error as BindingError;
use std::{
    io::Write,
    process::{Command, Stdio},
};
fn error(value: impl std::fmt::Display) -> BindingError {
    napi::Error::from_reason(value.to_string())
}
pub struct ProcessOutput {
    pub success: bool,
    pub code: Option<i32>,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}
impl ProcessOutput {
    pub fn stdout(&self) -> Vec<u8> {
        self.stdout.clone()
    }
    pub fn stderr(&self) -> Vec<u8> {
        self.stderr.clone()
    }
}
/// Own a plotting process and its stdin; arguments are passed without a shell.
pub struct Figure {
    inner: NativeFigure,
}
impl Figure {
    pub fn open() -> Result<Self, BindingError> {
        NativeFigure::open()
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn open_with(program: &str) -> Result<Self, BindingError> {
        NativeFigure::open_with(program)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn spawn(program: &str, arguments: Vec<String>) -> Result<Self, BindingError> {
        NativeFigure::spawn(
            Command::new(program)
                .args(arguments)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped()),
        )
        .map(|inner| Self { inner })
        .map_err(error)
    }
    pub fn id(&self) -> u32 {
        self.inner.id()
    }
    pub fn append(&mut self, bytes: &[u8]) -> Result<(), BindingError> {
        self.inner
            .commands_mut()
            .writer_mut()
            .write_all(bytes)
            .map_err(error)
    }
    pub fn flush(&mut self) -> Result<(), BindingError> {
        self.inner
            .commands_mut()
            .writer_mut()
            .flush()
            .map_err(error)
    }
    pub fn close_with_output(self) -> Result<ProcessOutput, BindingError> {
        self.inner
            .close_with_output()
            .map(|output| ProcessOutput {
                success: output.status.success(),
                code: output.status.code(),
                stdout: output.stdout,
                stderr: output.stderr,
            })
            .map_err(error)
    }
}

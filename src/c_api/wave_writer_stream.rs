//! Synchronous borrowed Write adapters, with explicit callback validity.
use std::{
    ffi::c_void,
    io::{self, Write},
};
/// Borrowed writer; callback status zero means success, nonzero means I/O error.
/// Write reports <= count initialized source bytes consumed. Partial writes are
/// supported. PCM encoding never calls flush and never closes or retains context.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CigletRsWriteStream {
    pub context: *mut c_void,
    pub write: Option<unsafe extern "C" fn(*mut c_void, *const u8, usize, *mut usize) -> u32>,
    /// Optional native Write adapter hook; PCM encoding does not invoke it.
    pub flush: Option<unsafe extern "C" fn(*mut c_void) -> u32>,
}
pub(super) struct Writer(pub CigletRsWriteStream);
impl Write for Writer {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.is_empty() {
            return Ok(0);
        }
        let callback = self
            .0
            .write
            .ok_or_else(|| io::Error::other("missing write callback"))?;
        let mut count = 0;
        // SAFETY: Boundary caller retains a valid non-unwinding callback/context;
        // source and count slot are valid only for this synchronous call.
        let status = unsafe { callback(self.0.context, bytes.as_ptr(), bytes.len(), &mut count) };
        if status != 0 {
            return Err(io::Error::other("write callback failed"));
        }
        if count > bytes.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "write count exceeds source",
            ));
        }
        Ok(count)
    }
    fn flush(&mut self) -> io::Result<()> {
        let callback = self.0.flush.ok_or_else(|| {
            io::Error::new(io::ErrorKind::Unsupported, "caller owns stream flushing")
        })?;
        // SAFETY: Same synchronous callback/context contract as write.
        if unsafe { callback(self.0.context) } != 0 {
            return Err(io::Error::other("flush callback failed"));
        }
        Ok(())
    }
}

//! Synchronous borrowed Read + Seek adapters; foreign callbacks are never retained.
use std::{
    ffi::c_void,
    io::{self, Read, Seek, SeekFrom},
};

/// Absolute seek origin; absolute is used and relative is ignored.
pub const CIGLET_RS_SEEK_START: u32 = 0;
/// Relative seek origin from current position; absolute is ignored.
pub const CIGLET_RS_SEEK_CURRENT: u32 = 1;
/// Relative seek origin from stream end; absolute is ignored.
pub const CIGLET_RS_SEEK_END: u32 = 2;

/// Borrowed synchronous reader. Callback status zero means success; otherwise I/O error.
/// Read writes at most capacity bytes, reports count <= capacity, and uses zero count
/// for EOF. Seek reports a new absolute position and must support all three origins.
/// Callbacks and context remain valid for the entire decode, must not unwind and
/// must not modify the decode output slot. They may modify their independent context.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CigletRsReadStream {
    pub context: *mut c_void,
    pub read: Option<unsafe extern "C" fn(*mut c_void, *mut u8, usize, *mut usize) -> u32>,
    pub seek: Option<unsafe extern "C" fn(*mut c_void, u32, u64, i64, *mut u64) -> u32>,
}

pub(super) struct Reader(pub CigletRsReadStream);
impl Read for Reader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }
        let callback = self
            .0
            .read
            .ok_or_else(|| io::Error::other("missing read callback"))?;
        let mut count = 0;
        // SAFETY: Boundary caller guarantees synchronous callback/context validity;
        // temporary initialized buffer and count slot are writable for this call.
        let status = unsafe {
            callback(
                self.0.context,
                buffer.as_mut_ptr(),
                buffer.len(),
                &mut count,
            )
        };
        if status != 0 {
            return Err(io::Error::other("read callback failed"));
        }
        if count > buffer.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "read count exceeds buffer",
            ));
        }
        Ok(count)
    }
}
impl Seek for Reader {
    fn seek(&mut self, from: SeekFrom) -> io::Result<u64> {
        let callback = self
            .0
            .seek
            .ok_or_else(|| io::Error::other("missing seek callback"))?;
        let (origin, absolute, relative) = match from {
            SeekFrom::Start(n) => (CIGLET_RS_SEEK_START, n, 0),
            SeekFrom::Current(n) => (CIGLET_RS_SEEK_CURRENT, 0, n),
            SeekFrom::End(n) => (CIGLET_RS_SEEK_END, 0, n),
        };
        let mut position = 0;
        // SAFETY: Boundary caller guarantees synchronous callback/context validity;
        // returned absolute position uses this temporary initialized writable slot.
        let status = unsafe { callback(self.0.context, origin, absolute, relative, &mut position) };
        if status != 0 {
            return Err(io::Error::other("seek callback failed"));
        }
        Ok(position)
    }
}

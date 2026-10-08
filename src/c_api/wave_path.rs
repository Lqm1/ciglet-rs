//! Explicit UTF-8 and lossless OS-native path representations.
use crate::Error;
use std::path::PathBuf;

/// Path byte encoding is UTF-8, without a terminating NUL.
pub const CIGLET_RS_PATH_UTF8: u32 = 0;
/// Unix OS bytes or Windows UTF-16 little-endian code units, without a terminator.
pub const CIGLET_RS_PATH_NATIVE: u32 = 1;

/// Borrowed counted path bytes. Encoding one preserves native OS filename values.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CigletRsPath {
    pub bytes: *const u8,
    pub byte_count: usize,
    pub encoding: u32,
}

pub(super) fn path(bytes: &[u8], encoding: u32) -> Result<PathBuf, Error> {
    if encoding == CIGLET_RS_PATH_UTF8 {
        if bytes.contains(&0) {
            return Err(Error("path contains NUL"));
        }
        return std::str::from_utf8(bytes)
            .map(PathBuf::from)
            .map_err(|_| Error("invalid UTF-8 path"));
    }
    if encoding != CIGLET_RS_PATH_NATIVE {
        return Err(Error("unknown path encoding"));
    }
    native(bytes)
}
#[cfg(unix)]
fn native(bytes: &[u8]) -> Result<PathBuf, Error> {
    use std::{ffi::OsString, os::unix::ffi::OsStringExt};
    if bytes.contains(&0) {
        return Err(Error("path contains NUL"));
    }
    Ok(OsString::from_vec(bytes.to_vec()).into())
}
#[cfg(windows)]
fn native(bytes: &[u8]) -> Result<PathBuf, Error> {
    use std::{ffi::OsString, os::windows::ffi::OsStringExt};
    if !bytes.len().is_multiple_of(2) {
        return Err(Error("native Windows path length is odd"));
    }
    let wide: Vec<_> = bytes
        .chunks_exact(2)
        .map(|b| u16::from_le_bytes([b[0], b[1]]))
        .collect();
    if wide.contains(&0) {
        return Err(Error("path contains NUL"));
    }
    Ok(OsString::from_wide(&wide).into())
}
#[cfg(not(any(unix, windows)))]
fn native(_bytes: &[u8]) -> Result<PathBuf, Error> {
    Err(Error(
        "native path representation is unavailable on this target",
    ))
}

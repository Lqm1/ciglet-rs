//! Centered frames and inclusive-range extrema with staged caller outputs.
use super::{boundary, generated_result, range};
use crate::{Error, extrema, statistics::Scalar};
use std::panic::RefUnwindSafe;

fn direction(code: u32) -> Result<extrema::Direction, Error> {
    match code {
        0 => Ok(extrema::Direction::Maximum),
        1 => Ok(extrema::Direction::Minimum),
        _ => Err(Error("unknown extrema direction")),
    }
}
pub(super) unsafe fn frame<T: Scalar + RefUnwindSafe>(
    input: *const T,
    input_length: usize,
    center: isize,
    output: *mut T,
    output_length: usize,
) -> u32 {
    if let Err(status) = range(input, input_length) {
        return status;
    }
    // SAFETY: Caller input is initialized and stable; output is exclusively writable.
    // The generated adapter commits only after the input borrow ends.
    unsafe {
        generated_result(output, output_length, || {
            let input = if input_length == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(input, input_length)
            };
            crate::frame::fetch_frame(input, center, output_length)
        })
    }
}
type Search<T> =
    fn(&[T], std::ops::RangeInclusive<usize>, extrema::Direction) -> Result<usize, Error>;
pub(super) unsafe fn search<T: Scalar + RefUnwindSafe>(
    input: *const T,
    length: usize,
    start: usize,
    end: usize,
    mode: u32,
    output: *mut usize,
    compute: Search<T>,
) -> u32 {
    if let Err(status) = range(input, length).and_then(|()| range(output.cast_const(), 1)) {
        return status;
    }
    let result = boundary(|| {
        let direction = direction(mode)?;
        let input = if length == 0 {
            &[]
        } else {
            // SAFETY: Caller input is an initialized stable single-allocation range.
            unsafe { std::slice::from_raw_parts(input, length) }
        };
        compute(input, start..=end, direction)
    });
    match result {
        Ok(index) => {
            // SAFETY: Input borrow ended. Caller provides an aligned writable usize.
            unsafe { output.write(index) };
            0
        }
        Err(status) => status,
    }
}

//! Centered, zero-padded frames compatible with upstream `fetch_frame`.
use crate::{Error, statistics::Scalar, vector};

pub fn fetch_frame<T: Scalar>(input: &[T], center: isize, length: usize) -> Result<Vec<T>, Error> {
    let half = isize::try_from(length / 2).map_err(|_| Error("frame length is too large"))?;
    let start = center
        .checked_sub(half)
        .ok_or(Error("frame start overflows"))?;
    let mut output = vector::buffer(length)?;
    for offset in 0..length {
        let offset = isize::try_from(offset).map_err(|_| Error("frame length is too large"))?;
        let index = start
            .checked_add(offset)
            .ok_or(Error("frame index overflows"))?;
        output.push(
            usize::try_from(index)
                .ok()
                .and_then(|index| input.get(index))
                .copied()
                .unwrap_or_else(T::zero),
        );
    }
    Ok(output)
}

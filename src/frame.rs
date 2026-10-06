//! Centered, zero-padded frames compatible with upstream `fetch_frame`.
use crate::Error;

pub fn fetch_frame(input: &[f32], center: isize, length: usize) -> Result<Vec<f32>, Error> {
    let half = isize::try_from(length / 2).map_err(|_| Error("frame length is too large"))?;
    let start = center
        .checked_sub(half)
        .ok_or(Error("frame start overflows"))?;
    let mut output = Vec::with_capacity(length);
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
                .unwrap_or(0.0),
        );
    }
    Ok(output)
}

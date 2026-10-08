#![cfg(feature = "c-api")]
use ciglet_rs::c_api::*;
struct Reader<'a>(&'a [u8], usize);
impl Reader<'_> {
    fn take(&mut self, n: usize) -> &[u8] {
        let (a, b) = self.0.split_at(n);
        self.0 = b;
        a
    }
    fn integer(&mut self) -> u32 {
        u32::from_le_bytes(self.take(4).try_into().unwrap())
    }
    fn scalar(&mut self) -> f64 {
        if self.1 == 4 {
            f32::from_le_bytes(self.take(4).try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(self.take(8).try_into().unwrap())
        }
    }
}

#[test]
fn frames_and_extrema_f32_match_original_c() {
    let input = [-2.0_f32, 0.5, 4.0, 1.25, -0.75, 3.0];
    let mut r = Reader(include_bytes!("fixtures/frame-float.bin"), 4);
    assert_eq!(r.take(4), b"FRM1");
    assert_eq!(r.integer(), 4);
    assert_eq!(r.integer(), 25);
    for _ in 0..25 {
        let center = r.integer() as i32 as isize;
        let n = r.integer() as usize;
        let mut output = vec![71.0; n];
        // SAFETY: Initialized input and independent writable output allocation.
        unsafe {
            assert_eq!(
                ciglet_rs_fetch_frame_f32(
                    input.as_ptr(),
                    input.len(),
                    center,
                    output.as_mut_ptr(),
                    n
                ),
                0
            );
        }
        for value in output {
            assert_eq!((value as f64).to_bits(), r.scalar().to_bits());
        }
    }
    assert!(r.0.is_empty());
    let mut r = Reader(include_bytes!("fixtures/extrema-float.bin"), 4);
    assert_eq!(r.take(4), b"EXT2");
    assert_eq!(r.integer(), 4);
    assert_eq!(r.integer(), 180);
    for _ in 0..180 {
        let n = r.integer() as usize;
        let start = r.integer() as usize;
        let end = r.integer() as usize;
        let direction = r.integer();
        let signal: Vec<f32> = (0..n).map(|_| r.scalar() as f32).collect();
        let peak = r.integer() as usize;
        let extreme = r.integer() as usize;
        let mut output = 71;
        // SAFETY: Initialized in-bounds input and independent writable index.
        unsafe {
            assert_eq!(
                ciglet_rs_peak_f32(signal.as_ptr(), n, start, end, direction, &mut output),
                0
            );
            assert_eq!(output, peak);
            assert_eq!(
                ciglet_rs_extremum_f32(signal.as_ptr(), n, start, end, direction, &mut output),
                0
            );
            assert_eq!(output, extreme);
        }
    }
    assert!(r.0.is_empty());
}

#[test]
fn frames_and_extrema_f64_match_original_c() {
    let input = [-2.0_f64, 0.5, 4.0, 1.25, -0.75, 3.0];
    let mut r = Reader(include_bytes!("fixtures/frame-double.bin"), 8);
    assert_eq!(r.take(4), b"FRM1");
    assert_eq!(r.integer(), 8);
    assert_eq!(r.integer(), 25);
    for _ in 0..25 {
        let center = r.integer() as i32 as isize;
        let n = r.integer() as usize;
        let mut output = vec![71.0; n];
        // SAFETY: Initialized input and independent writable output allocation.
        unsafe {
            assert_eq!(
                ciglet_rs_fetch_frame_f64(
                    input.as_ptr(),
                    input.len(),
                    center,
                    output.as_mut_ptr(),
                    n
                ),
                0
            );
        }
        for value in output {
            assert_eq!(value.to_bits(), r.scalar().to_bits());
        }
    }
    assert!(r.0.is_empty());
    let mut r = Reader(include_bytes!("fixtures/extrema-double.bin"), 8);
    assert_eq!(r.take(4), b"EXT2");
    assert_eq!(r.integer(), 8);
    assert_eq!(r.integer(), 180);
    for _ in 0..180 {
        let n = r.integer() as usize;
        let start = r.integer() as usize;
        let end = r.integer() as usize;
        let direction = r.integer();
        let signal: Vec<f64> = (0..n).map(|_| r.scalar()).collect();
        let peak = r.integer() as usize;
        let extreme = r.integer() as usize;
        let mut output = 71;
        // SAFETY: Initialized in-bounds input and independent writable index.
        unsafe {
            assert_eq!(
                ciglet_rs_peak_f64(signal.as_ptr(), n, start, end, direction, &mut output),
                0
            );
            assert_eq!(output, peak);
            assert_eq!(
                ciglet_rs_extremum_f64(signal.as_ptr(), n, start, end, direction, &mut output),
                0
            );
            assert_eq!(output, extreme);
        }
    }
    assert!(r.0.is_empty());
}

#[test]
fn frames_support_overlap_special_bits_and_atomic_errors() {
    let mut input = [1.0_f64, 2.0, 3.0, 4.0, 5.0, 6.0];
    // SAFETY: The frame output occupies a live prefix of the initialized input array.
    unsafe {
        assert_eq!(
            ciglet_rs_fetch_frame_f64(input.as_ptr(), 6, 0, input.as_mut_ptr(), 4),
            0
        );
    }
    assert_eq!(input, [0.0, 0.0, 1.0, 2.0, 5.0, 6.0]);
    let special = [f32::from_bits(0x7fc00042), -0.0, f32::INFINITY];
    let mut output = [71.0_f32; 3];
    // SAFETY: Valid independent buffers; a zero-length null input is not read.
    unsafe {
        assert_eq!(
            ciglet_rs_fetch_frame_f32(special.as_ptr(), 3, 1, output.as_mut_ptr(), 3),
            0
        );
        for (a, b) in output.iter().zip(special) {
            assert_eq!(a.to_bits(), b.to_bits());
        }
        output = [71.0; 3];
        assert_eq!(
            ciglet_rs_fetch_frame_f32(std::ptr::null(), 0, isize::MIN, output.as_mut_ptr(), 3),
            3
        );
        assert_eq!(output, [71.0; 3]);
        assert_eq!(
            ciglet_rs_fetch_frame_f32(std::ptr::null(), 0, 0, output.as_mut_ptr(), 3),
            0
        );
        assert_eq!(output, [0.0; 3]);
    }
}
#[test]
fn searches_preserve_fallback_inspected_ranges_and_failed_outputs() {
    let signal = [8.0_f64, 1.0, 5.0, 0.0, 5.0, 1.0, 9.0];
    let mut output = 71;
    // SAFETY: Initialized seven-element input and writable independent usize.
    unsafe {
        assert_eq!(
            ciglet_rs_peak_f64(signal.as_ptr(), 7, 0, 6, 0, &mut output),
            0
        );
        assert_eq!(output, 0);
        assert_eq!(
            ciglet_rs_peak_f64(signal.as_ptr(), 7, 1, 5, 0, &mut output),
            0
        );
        assert_eq!(output, 2);
        assert_eq!(
            ciglet_rs_extremum_f64(signal.as_ptr(), 7, 0, 6, 0, &mut output),
            0
        );
        assert_eq!(output, 6);
        output = 71;
        assert_eq!(
            ciglet_rs_peak_f64(signal.as_ptr(), 7, 5, 3, 0, &mut output),
            3
        );
        assert_eq!(
            ciglet_rs_extremum_f64(signal.as_ptr(), 7, 0, 7, 0, &mut output),
            3
        );
        assert_eq!(
            ciglet_rs_peak_f64(signal.as_ptr(), 7, 0, 6, 2, &mut output),
            3
        );
        assert_eq!(output, 71);
        let values = [f64::NAN, 1.0, 3.0, 2.0, f64::NAN];
        assert_eq!(
            ciglet_rs_peak_f64(values.as_ptr(), 5, 2, 2, 0, &mut output),
            0
        );
        assert_eq!(output, 2);
        output = 71;
        assert_eq!(
            ciglet_rs_peak_f64(values.as_ptr(), 5, 1, 3, 0, &mut output),
            3
        );
        assert_eq!(output, 71);
        assert_eq!(
            ciglet_rs_extremum_f64(std::ptr::null(), 0, 0, 0, 0, &mut output),
            3
        );
        assert_eq!(output, 71);
    }
}

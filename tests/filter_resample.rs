use ciglet_rs::{
    filtering::{self, Passband},
    resampling::{self, BoundaryPolicy},
    window::Window,
};
use rustfft::FftNum;
struct Reader<'a>(&'a [u8]);
impl Reader<'_> {
    fn bytes(&mut self, n: usize) -> &[u8] {
        let (head, tail) = self.0.split_at(n);
        self.0 = tail;
        head
    }
    fn integer(&mut self) -> usize {
        u32::from_le_bytes(self.bytes(4).try_into().unwrap()) as usize
    }
    fn scalar<T: FftNum>(&mut self, width: usize) -> T {
        T::from_f64(if width == 4 {
            f32::from_le_bytes(self.bytes(4).try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(self.bytes(8).try_into().unwrap())
        })
        .unwrap()
    }
    fn values<T: FftNum>(&mut self, width: usize, length: usize) -> Vec<T> {
        (0..length).map(|_| self.scalar(width)).collect()
    }
}
fn compare<T: FftNum + Into<f64>>(bytes: &[u8], width: usize) {
    let mut r = Reader(bytes);
    assert_eq!(r.bytes(4), b"FRS1");
    assert_eq!(r.integer(), width);
    let counts = [r.integer(), r.integer(), r.integer()];
    assert_eq!(counts, [300, 60, 105]);
    let windows = [
        Window::Boxcar,
        Window::Hann,
        Window::Hamming,
        Window::Blackman,
        Window::BlackmanHarris,
    ];
    let mut maxima = [0.0f64; 3];
    let mut scalars = [0; 3];
    for (group, &count) in counts.iter().enumerate() {
        for record in 0..count {
            let actual = match group {
                0 => {
                    let n = r.integer();
                    let window = windows[r.integer()];
                    let kind = r.integer();
                    let lower = r.scalar::<T>(width);
                    let upper = r.scalar::<T>(width);
                    let band = match kind {
                        0 => Passband::Lowpass(lower),
                        1 => Passband::Highpass(lower),
                        2 => Passband::Bandpass { lower, upper },
                        _ => Passband::Bandstop { lower, upper },
                    };
                    let h = filtering::windowed_fir(n, band, window).unwrap();
                    if kind < 2 {
                        let doubled = lower * T::from_u8(2).unwrap();
                        assert_eq!(h, filtering::fir(n, doubled, kind == 1, window).unwrap());
                    } else if kind == 2 {
                        let two = T::from_u8(2).unwrap();
                        assert_eq!(
                            h,
                            filtering::bandpass_fir(n, lower * two, upper * two, window).unwrap()
                        );
                    }
                    h
                }
                1 => {
                    let nx = r.integer();
                    let nh = r.integer();
                    let x = r.values::<T>(width, nx);
                    let h = r.values::<T>(width, nh);
                    filtering::convolve(&x, &h).unwrap()
                }
                _ => {
                    let nx = r.integer();
                    let ratio = r.scalar(width);
                    let x = r.values::<T>(width, nx);
                    let ny = r.integer();
                    let y = resampling::resample_with(&x, ratio, BoundaryPolicy::LegacySkipFirst)
                        .unwrap();
                    assert_eq!(y.len(), ny);
                    y
                }
            };
            for (index, value) in actual.into_iter().enumerate() {
                let expected: f64 = r.scalar::<T>(width).into();
                let actual: f64 = value.into();
                let error = (actual - expected).abs() / expected.abs().max(1.0);
                maxima[group] = maxima[group].max(error);
                scalars[group] += 1;
                assert!(
                    error < if width == 4 { 2e-7 } else { 2e-15 },
                    "{group}/{record}/{index}: {actual} vs {expected}, error {error:e}"
                );
            }
        }
    }
    assert!(r.0.is_empty());
    eprintln!(
        "Filter/resample {width}-byte: counts {scalars:?}, normalized differences {maxima:?}"
    );
}
#[test]
fn float_filtering_and_resampling_match_c() {
    compare::<f32>(include_bytes!("fixtures/c-filter-resample-f32.bin"), 4);
}
#[test]
fn double_filtering_and_resampling_match_c() {
    compare::<f64>(include_bytes!("fixtures/c-filter-resample-f64.bin"), 8);
}
#[test]
fn independent_convolution_and_boundaries() {
    assert_eq!(
        filtering::convolve(&[1.0f32, 2.0, 3.0], &[2.0, -1.0]).unwrap(),
        [2.0, 3.0, 4.0, -3.0]
    );
    assert_eq!(
        filtering::convolve(&[1.0f64], &[1.0, 2.0, 3.0]).unwrap(),
        [1.0, 2.0, 3.0]
    );
    assert!(filtering::convolve::<f32>(&[], &[1.0]).unwrap().is_empty());
    assert!(filtering::convolve::<f32>(&[1.0], &[]).unwrap().is_empty());
    assert!(filtering::convolve(&[f32::NAN], &[1.0]).is_err());
    let corrected = resampling::resample(&[0.75f32], 2.0).unwrap();
    assert_eq!(corrected.len(), 2);
    assert_eq!(corrected[0], 0.75);
    assert!(corrected[1] > 0.4);
    assert_eq!(
        resampling::resample_with(&[0.75f32], 2.0, BoundaryPolicy::LegacySkipFirst).unwrap(),
        [0.0, 0.0]
    );
    assert_eq!(
        resampling::resample(&[1.0f32, -0.0, 0.5], 1.0)
            .unwrap()
            .iter()
            .map(|x| x.to_bits())
            .collect::<Vec<_>>(),
        [1.0f32, -0.0, 0.5]
            .iter()
            .map(|x| x.to_bits())
            .collect::<Vec<_>>()
    );
    assert!(resampling::resample::<f32>(&[], 0.5).unwrap().is_empty());
    assert!(resampling::resample(&[1.0f64], 0.25).unwrap().is_empty());
    for ratio in [0.0, -1.0, f32::NAN, f32::INFINITY] {
        assert!(resampling::resample(&[1.0], ratio).is_err());
    }
    assert!(resampling::resample(&[f32::NAN], 1.0).is_err());
    assert!(resampling::resample(&[1.0f32], f32::MAX).is_err());
    for n in [0, 1] {
        assert!(filtering::windowed_fir(n, Passband::Lowpass(0.25f32), Window::Hann).is_err());
    }
    for band in [
        Passband::Lowpass(-0.1),
        Passband::Highpass(f64::NAN),
        Passband::Bandpass {
            lower: 0.4,
            upper: 0.3,
        },
        Passband::Bandstop {
            lower: 0.0,
            upper: 0.6,
        },
    ] {
        assert!(filtering::windowed_fir(32, band, Window::Hamming).is_err());
    }
    for n in [2, 3, 8, 31, 32] {
        assert!(
            filtering::windowed_fir(n, Passband::Lowpass(0.5f64), Window::Boxcar)
                .unwrap()
                .iter()
                .all(|x| x.is_finite())
        );
    }
}

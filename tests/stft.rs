use ciglet_rs::{
    stft::{AnalysisOptions, StftPlan},
    window::{Precision, Window},
};
use rustfft::FftNum;

struct Reader<'a>(&'a [u8]);
impl Reader<'_> {
    fn bytes(&mut self, count: usize) -> &[u8] {
        let (head, tail) = self.0.split_at(count);
        self.0 = tail;
        head
    }
    fn integer(&mut self) -> usize {
        u32::from_le_bytes(self.bytes(4).try_into().unwrap()) as usize
    }
    fn scalar(&mut self, width: usize) -> f64 {
        if width == 4 {
            f32::from_le_bytes(self.bytes(4).try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(self.bytes(8).try_into().unwrap())
        }
    }
}

fn compare<T: FftNum + Into<f64>>(bytes: &[u8], width: usize) {
    let mut reader = Reader(bytes);
    assert_eq!(reader.bytes(4), b"STF1");
    assert_eq!(reader.integer(), width);
    assert_eq!(reader.integer(), 480);
    let windows = [
        Window::Boxcar,
        Window::Hann,
        Window::Hamming,
        Window::Blackman,
        Window::BlackmanHarris,
    ];
    let mut maximum_magnitude = 0.0f64;
    let mut maximum_phase = 0.0f64;
    let mut maximum_weighted_phase = 0.0f64;
    let mut skipped_phase = 0;
    for record in 0..480 {
        let window = windows[reader.integer()];
        let precision = if reader.integer() == 0 {
            Precision::Accurate
        } else {
            Precision::Approximate
        };
        let subtract_mean = reader.integer() != 0;
        let variable = reader.integer() != 0;
        let scenario = reader.integer();
        let mask = reader.integer();
        let expected_norm = reader.scalar(width);
        let expected_weight = reader.scalar(width);
        let signal: Vec<T> = (0..27)
            .map(|i| {
                T::from_f64(match scenario {
                    0 => {
                        if i == 11 {
                            0.75
                        } else {
                            0.0
                        }
                    }
                    1 => (i % 9 - 4) as f64 * 0.125,
                    _ => 2.0,
                })
                .unwrap()
            })
            .collect();
        let lengths = if variable { [8, 12, 16, 20] } else { [16; 4] };
        let options = AnalysisOptions {
            subtract_mean,
            magnitude: mask & 1 != 0,
            phase: mask & 2 != 0,
            normalization_hop: (!variable).then_some(8),
        };
        let mut plan = StftPlan::<T>::new(32, window, precision).unwrap();
        let result = plan
            .analyze(&signal, &[-3, 5, 13, 21], &lengths, options)
            .unwrap();
        let full = plan
            .analyze(
                &signal,
                &[-3, 5, 13, 21],
                &lengths,
                AnalysisOptions {
                    magnitude: true,
                    phase: true,
                    ..options
                },
            )
            .unwrap();
        assert_eq!(result.frames, 4);
        assert_eq!(result.bins, 17);
        if !variable {
            assert!((result.window_weight.unwrap().into() - expected_weight).abs() < 1e-14);
            assert!((result.normalization.unwrap().into() - expected_norm).abs() < 1e-14);
        }
        for index in 0..68 {
            if mask & 1 != 0 {
                let expected = reader.scalar(width);
                let actual = result.magnitude.as_ref().unwrap()[index].into();
                let error = (actual - expected).abs() / expected.abs().max(1.0);
                maximum_magnitude = maximum_magnitude.max(error);
                assert!(
                    error < if width == 4 { 2e-6 } else { 2e-14 },
                    "magnitude {record}/{index}: {actual} vs {expected}"
                );
                assert_eq!(
                    result.magnitude.as_ref().unwrap()[index],
                    full.magnitude.as_ref().unwrap()[index]
                );
            }
            if mask & 2 != 0 {
                let expected = reader.scalar(width);
                let actual = result.phase.as_ref().unwrap()[index].into();
                assert_eq!(
                    result.phase.as_ref().unwrap()[index],
                    full.phase.as_ref().unwrap()[index]
                );
                if full.magnitude.as_ref().unwrap()[index].into() < 1e-6 {
                    skipped_phase += 1;
                    continue;
                }
                let difference = actual - expected;
                let error = difference.sin().atan2(difference.cos()).abs();
                maximum_phase = maximum_phase.max(error);
                let frame_begin = index / 17 * 17;
                let frame_scale = full.magnitude.as_ref().unwrap()[frame_begin..frame_begin + 17]
                    .iter()
                    .map(|&v| v.into())
                    .fold(1.0f64, f64::max);
                let weighted = 2.0
                    * (difference * 0.5).sin().abs()
                    * full.magnitude.as_ref().unwrap()[index].into()
                    / frame_scale;
                maximum_weighted_phase = maximum_weighted_phase.max(weighted);
                assert!(
                    weighted < if width == 4 { 2e-6 } else { 2e-14 },
                    "phase {record}/{index}: {actual} vs {expected}"
                );
            }
        }
    }
    assert!(reader.0.is_empty());
    eprintln!(
        "STFT {width}-byte: magnitude normalized {maximum_magnitude:e}, phase circular {maximum_phase:e}, weighted {maximum_weighted_phase:e}, {skipped_phase} near-zero phases excluded"
    );
}
#[test]
fn float_stft_matches_c() {
    compare::<f32>(include_bytes!("fixtures/c-stft-f32.bin"), 4);
}
#[test]
fn double_stft_matches_c() {
    compare::<f64>(include_bytes!("fixtures/c-stft-f64.bin"), 8);
}

#[test]
fn odd_windows_variable_weights_padding_and_validation() {
    let mut plan = StftPlan::<f64>::new(8, Window::Boxcar, Precision::Accurate).unwrap();
    let odd = plan
        .analyze(&[1.0, 2.0, 3.0], &[1], &[3], AnalysisOptions::default())
        .unwrap();
    assert_eq!(odd.magnitude.as_ref().unwrap()[0], 6.0);
    assert_eq!(odd.window_weight, Some(3.0));
    let padded = plan
        .analyze(&[1.0, 2.0, 3.0], &[-1], &[3], AnalysisOptions::default())
        .unwrap();
    assert_eq!(padded.magnitude.as_ref().unwrap()[0], 1.0);
    let mut plan = StftPlan::<f64>::new(8, Window::Hann, Precision::Accurate).unwrap();
    let result = plan
        .analyze(
            &[1.0; 12],
            &[4, 8],
            &[4, 8],
            AnalysisOptions {
                normalization_hop: Some(2),
                ..Default::default()
            },
        )
        .unwrap();
    let expected = Window::Hann
        .generate_with::<f64>(8, Precision::Accurate)
        .unwrap()
        .iter()
        .sum::<f64>();
    assert!((result.magnitude.unwrap()[5] - expected).abs() < 1e-14);
    assert!(
        plan.analyze(&[], &[0], &[], AnalysisOptions::default())
            .is_err()
    );
    assert!(
        plan.analyze(&[], &[0], &[9], AnalysisOptions::default())
            .is_err()
    );
    assert!(
        plan.analyze(&[], &[0], &[0], AnalysisOptions::default())
            .is_err()
    );
    assert!(
        plan.analyze(&[f64::NAN], &[0], &[4], AnalysisOptions::default())
            .is_err()
    );
    assert!(
        plan.analyze(
            &[],
            &[0],
            &[4],
            AnalysisOptions {
                normalization_hop: Some(0),
                ..Default::default()
            }
        )
        .is_err()
    );
    let empty = plan
        .analyze(&[], &[], &[], AnalysisOptions::default())
        .unwrap();
    assert!(empty.magnitude.unwrap().is_empty());
    assert_eq!(empty.window_weight, None);
}

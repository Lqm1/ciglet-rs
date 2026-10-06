use ciglet_rs::{
    approximation,
    stft::{
        AnalysisOptions, FrameGrid, NyquistPolicy, StftPlan, SynthesisOptions, analyze_regular,
        synthesize_regular,
    },
    window::{Precision, Window},
};
use rustfft::FftNum;
struct Reader<'a>(&'a [u8]);
impl Reader<'_> {
    fn bytes(&mut self, n: usize) -> &[u8] {
        let (head, tail) = self.0.split_at(n);
        self.0 = tail;
        head
    }
    fn integer(&mut self) -> u32 {
        u32::from_le_bytes(self.bytes(4).try_into().unwrap())
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
    assert_eq!(reader.bytes(4), b"IST1");
    assert_eq!(reader.integer() as usize, width);
    assert_eq!(reader.integer(), 288);
    let mut maximum = 0.0f64;
    let mut samples = 0;
    for record in 0..288 {
        let length = reader.integer() as usize;
        let frames = reader.integer() as usize;
        let fade = reader.integer() as usize;
        let offset = reader.integer() as i32;
        let _scenario = reader.integer();
        let normalization = T::from_f64(reader.scalar(width)).unwrap();
        let mut magnitude = Vec::new();
        let mut phase = Vec::new();
        for _ in 0..frames * (length / 2 + 1) {
            magnitude.push(T::from_f64(reader.scalar(width)).unwrap());
            phase.push(T::from_f64(reader.scalar(width)).unwrap());
        }
        let mut plan = StftPlan::<T>::new(length, Window::Blackman, Precision::Accurate).unwrap();
        let options = SynthesisOptions {
            hop: 4,
            offset,
            fade_samples: fade,
            normalization,
            nyquist: NyquistPolicy::Legacy,
        };
        let actual = plan.synthesize(&magnitude, &phase, options).unwrap();
        assert_eq!(actual.len(), reader.integer() as usize);
        for (index, value) in actual.iter().enumerate() {
            let expected = reader.scalar(width);
            let value: f64 = (*value).into();
            let error = (value - expected).abs() / expected.abs().max(1.0);
            maximum = maximum.max(error);
            assert!(
                error < if width == 4 { 2e-7 } else { 2e-15 },
                "inverse {record}/{index}: {value} vs {expected}"
            );
            samples += 1;
        }
        assert_eq!(
            actual,
            plan.synthesize(&magnitude, &phase, options).unwrap()
        );
    }
    assert!(reader.0.is_empty());
    eprintln!("ISTFT {width}-byte: {samples} samples, maximum normalized error {maximum:e}");
}
#[test]
fn float_inverse_matches_c() {
    compare::<f32>(include_bytes!("fixtures/c-istft-f32.bin"), 4);
}
#[test]
fn double_inverse_matches_c() {
    compare::<f64>(include_bytes!("fixtures/c-istft-f64.bin"), 8);
}

#[test]
fn nyquist_is_preserved_and_invalid_inputs_are_rejected() {
    let mut plan = StftPlan::<f64>::new(8, Window::Boxcar, Precision::Accurate).unwrap();
    let magnitude = [0.0, 0.0, 0.0, 0.0, 8.0];
    let phase = [0.0; 5];
    let options = SynthesisOptions {
        hop: 8,
        offset: 4,
        fade_samples: 0,
        normalization: 1.0,
        nyquist: NyquistPolicy::Preserve,
    };
    let output = plan.synthesize(&magnitude, &phase, options).unwrap();
    let amplitude = approximation::cosine(0.0).unwrap() as f64;
    for (index, &value) in output[..8].iter().enumerate() {
        assert_eq!(
            value,
            if index % 2 == 0 {
                amplitude
            } else {
                -amplitude
            }
        );
    }
    assert_eq!(&output[8..], &[0.0; 4]);
    assert!(
        plan.synthesize(
            &magnitude,
            &phase,
            SynthesisOptions {
                nyquist: NyquistPolicy::Legacy,
                ..options
            }
        )
        .unwrap()
        .iter()
        .all(|&x| x == 0.0)
    );
    assert!(plan.synthesize(&magnitude, &phase[..4], options).is_err());
    assert!(
        plan.synthesize(
            &magnitude,
            &phase,
            SynthesisOptions {
                normalization: 0.0,
                ..options
            }
        )
        .is_err()
    );
    assert!(
        plan.synthesize(&magnitude, &phase, SynthesisOptions { hop: 0, ..options })
            .is_err()
    );
    assert!(
        plan.synthesize(
            &magnitude,
            &phase,
            SynthesisOptions {
                fade_samples: 9,
                ..options
            }
        )
        .is_err()
    );
    assert!(
        plan.synthesize(
            &magnitude,
            &phase,
            SynthesisOptions {
                offset: -9,
                ..options
            }
        )
        .is_err()
    );
    assert!(
        plan.synthesize(&magnitude, &[f64::NAN; 5], options)
            .is_err()
    );
    assert!(
        plan.synthesize(
            &[],
            &[],
            SynthesisOptions {
                offset: 0,
                ..options
            }
        )
        .unwrap()
        .is_empty()
    );
}

#[test]
fn regular_grid_matches_explicit_analysis_and_reconstructs_audio() {
    let grid = FrameGrid {
        hop: 8,
        frames: 24,
        window_factor: 8,
        zero_padding_factor: 2,
    };
    let signal: Vec<f64> = (0..192)
        .map(|n| (std::f64::consts::TAU * n as f64 / 32.0).sin())
        .collect();
    let result = analyze_regular(
        &signal,
        grid,
        Precision::Accurate,
        AnalysisOptions::default(),
    )
    .unwrap();
    let level_two = analyze_regular(
        &signal,
        grid,
        Precision::Approximate,
        AnalysisOptions::default(),
    )
    .unwrap();
    assert_eq!(result.magnitude, level_two.magnitude);
    assert_eq!(result.phase, level_two.phase);
    let centers: Vec<i32> = (0..24).map(|n| n * 8).collect();
    let explicit = StftPlan::<f64>::new(128, Window::Blackman, Precision::Accurate)
        .unwrap()
        .analyze(
            &signal,
            &centers,
            &[64; 24],
            AnalysisOptions {
                normalization_hop: Some(8),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(result.magnitude, explicit.magnitude);
    assert_eq!(result.normalization, explicit.normalization);
    let output = synthesize_regular(
        result.magnitude.as_ref().unwrap(),
        result.phase.as_ref().unwrap(),
        grid,
        result.normalization.unwrap(),
        NyquistPolicy::Preserve,
    )
    .unwrap();
    let error = output[64..128]
        .iter()
        .zip(&signal[64..128])
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f64, f64::max);
    eprintln!("Regular STFT/ISTFT interior reconstruction error {error:e}");
    assert!(error < 1e-4);
    assert_eq!(output.len(), 192);
    assert!(
        analyze_regular::<f64>(
            &[],
            FrameGrid { hop: 0, ..grid },
            Precision::Accurate,
            AnalysisOptions::default()
        )
        .is_err()
    );
    assert!(synthesize_regular(&[0.0], &[0.0], grid, 1.0, NyquistPolicy::Preserve).is_err());
    let one = analyze_regular(
        &[1.0; 8],
        FrameGrid { frames: 1, ..grid },
        Precision::Accurate,
        AnalysisOptions::default(),
    )
    .unwrap();
    assert!(one.normalization.unwrap() > 0.0);
}

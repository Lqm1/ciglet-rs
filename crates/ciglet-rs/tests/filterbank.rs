use ciglet_rs::filterbank::{Compression, FilterBank, MelOptions};
use rustfft::FftNum;
struct Reader<'a>(&'a [u8]);
impl Reader<'_> {
    fn bytes(&mut self, n: usize) -> &[u8] {
        let (a, b) = self.0.split_at(n);
        self.0 = b;
        a
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
fn value<T: FftNum>(x: f64) -> T {
    T::from_f64(x).unwrap()
}
fn compare<T: FftNum + Into<f64>>(bytes: &[u8], width: usize) {
    let mut reader = Reader(bytes);
    assert_eq!(reader.bytes(4), b"FBK1");
    assert_eq!(reader.integer(), width);
    assert_eq!(reader.integer(), 72);
    let mut maximum_response = 0.0f64;
    let mut maximum_output = 0.0f64;
    let mut responses = 0;
    let mut outputs = 0;
    for record in 0..72 {
        let kind = reader.integer();
        let bins = reader.integer();
        let channels = reader.integer();
        let nyquist = value::<T>(reader.scalar(width));
        let bank = match kind {
            0 => FilterBank::empty(bins, nyquist, channels),
            1 => FilterBank::mel(
                bins,
                nyquist,
                channels,
                MelOptions {
                    minimum_hz: value(50.0),
                    maximum_hz: nyquist,
                    scale: value(0.85),
                    minimum_width_hz: value(120.0),
                },
            ),
            2 => FilterBank::mel_default(bins, nyquist, channels, value(50.0), nyquist),
            _ => FilterBank::plp(bins, nyquist, channels),
        }
        .unwrap();
        assert_eq!(bank.bins(), bins);
        assert_eq!(bank.channels(), channels);
        assert_eq!(bank.nyquist_hz(), nyquist);
        for channel in 0..channels {
            let lower = reader.integer();
            let upper = reader.integer();
            assert_eq!(bank.support(channel), Some(lower..upper));
            for (bin, &actual) in bank.response(channel).unwrap().iter().enumerate() {
                let expected = reader.scalar(width);
                let error = (actual.into() - expected).abs() / expected.abs().max(1.0);
                maximum_response = maximum_response.max(error);
                responses += 1;
                assert!(
                    error < if width == 4 { 2e-7 } else { 2e-14 },
                    "response {record}/{channel}/{bin}: {} vs {expected}",
                    actual.into()
                );
            }
        }
        let spectra: Vec<T> = (0..3)
            .flat_map(|scenario| {
                (0..bins).map(move |bin| {
                    value(match scenario {
                        0 => 0.0,
                        1 => (bin % 17 + 1) as f64 * 0.125,
                        _ => {
                            if bin == bins / 3 {
                                2.0
                            } else {
                                0.0
                            }
                        }
                    })
                })
            })
            .collect();
        for compression in [Compression::Logarithmic, Compression::Plp] {
            let all = bank.apply_spectrogram(&spectra, compression).unwrap();
            for frame in 0..3 {
                let single = bank
                    .apply(&spectra[frame * bins..(frame + 1) * bins], compression)
                    .unwrap();
                assert_eq!(&all[frame * channels..(frame + 1) * channels], single);
                for (channel, &actual) in single.iter().enumerate() {
                    let expected = reader.scalar(width);
                    let error = (actual.into() - expected).abs() / expected.abs().max(1.0);
                    maximum_output = maximum_output.max(error);
                    outputs += 1;
                    assert!(
                        error < if width == 4 { 2e-6 } else { 2e-13 },
                        "output {record}/{frame}/{channel}: {} vs {expected}",
                        actual.into()
                    );
                }
            }
            for &actual in &all {
                let expected = reader.scalar(width);
                let error = (actual.into() - expected).abs() / expected.abs().max(1.0);
                maximum_output = maximum_output.max(error);
                outputs += 1;
                assert!(error < if width == 4 { 2e-6 } else { 2e-13 });
            }
        }
    }
    assert!(reader.0.is_empty());
    eprintln!(
        "Filterbank {width}-byte: {responses} responses error {maximum_response:e}, {outputs} outputs error {maximum_output:e}"
    );
}
#[test]
fn float_filterbanks_match_c() {
    compare::<f32>(
        include_bytes!("../../../tests/fixtures/c-filterbank-f32.bin"),
        4,
    );
}
#[test]
fn double_filterbanks_match_c() {
    compare::<f64>(
        include_bytes!("../../../tests/fixtures/c-filterbank-f64.bin"),
        8,
    );
}
#[test]
fn small_grids_and_custom_channels_are_safe() {
    let bank = FilterBank::<f32>::mel_default(65, 8000.0, 20, 50.0, 8000.0).unwrap();
    assert_eq!(bank.support(0).unwrap().start, 0);
    assert_eq!(bank.response(0).unwrap()[0], 1.0);
    for bins in [1, 2, 4, 65] {
        for scale in [0.01, 1.0, 100.0] {
            let bank = FilterBank::<f64>::mel(
                bins,
                8000.0,
                20,
                MelOptions {
                    minimum_hz: 0.0,
                    maximum_hz: 8000.0,
                    scale,
                    minimum_width_hz: 0.0,
                },
            )
            .unwrap();
            for channel in 0..20 {
                let support = bank.support(channel).unwrap();
                assert!(support.start < support.end && support.end <= bins);
                assert!(
                    bank.response(channel)
                        .unwrap()
                        .iter()
                        .all(|&x| (0.0..=1.0).contains(&x))
                );
            }
            assert!(
                bank.apply(&vec![1.0; bins], Compression::Logarithmic)
                    .unwrap()
                    .iter()
                    .all(|x| x.is_finite())
            );
        }
    }
    let mut custom = FilterBank::<f64>::empty(4, 8000.0, 1).unwrap();
    custom.set_channel(0, &[9.0, 0.5, 1.0, 9.0], 1..3).unwrap();
    let expected =
        ciglet_rs::approximation::logarithm((0.5 * 2.0 + 1.0 * 3.0 + 1e-15) as f32).unwrap() as f64;
    assert_eq!(
        custom
            .apply(&[1.0, 2.0, 3.0, 4.0], Compression::Logarithmic)
            .unwrap(),
        [expected]
    );
    let before = custom.response(0).unwrap().to_vec();
    assert!(custom.set_channel(0, &[f64::NAN; 4], 0..4).is_err());
    assert_eq!(custom.response(0).unwrap(), before);
    assert!(custom.set_channel(0, &[0.0; 4], 0..5).is_err());
    assert!(custom.apply(&[0.0; 3], Compression::Plp).is_err());
    assert!(custom.apply(&[f64::NAN; 4], Compression::Plp).is_err());
    assert!(
        custom
            .apply_spectrogram(&[0.0; 5], Compression::Plp)
            .is_err()
    );
    assert!(
        custom
            .apply_spectrogram(&[], Compression::Plp)
            .unwrap()
            .is_empty()
    );
    assert_eq!(FilterBank::<f64>::empty(0, 8000.0, 1).unwrap().bins(), 0);
    assert_eq!(
        FilterBank::<f64>::empty(4, 0.0, 1).unwrap().nyquist_hz(),
        0.0
    );
    assert_eq!(
        FilterBank::<f64>::empty(4, 8000.0, 0)
            .unwrap()
            .apply(&[1.0; 4], Compression::Plp)
            .unwrap(),
        []
    );
}

fn custom_compare<T: FftNum + Into<f64>>(bytes: &[u8], width: usize) {
    let mut reader = Reader(bytes);
    assert_eq!(reader.bytes(4), b"CFB1");
    assert_eq!(reader.integer(), width);
    assert_eq!(reader.integer(), 33);
    let mut maximum_error = 0.0_f64;
    let mut outputs = 0;
    for record in 0..33 {
        let kind = reader.integer();
        let bins = reader.integer();
        let channels = reader.integer();
        let metadata = value::<T>(reader.scalar(width));
        let mut bank = if kind >= 5 {
            FilterBank::plp(bins, metadata, channels).unwrap()
        } else {
            FilterBank::empty(bins, metadata, channels).unwrap()
        };
        assert_eq!(bank.bins(), bins);
        assert_eq!(bank.channels(), channels);
        let stored: f64 = bank.nyquist_hz().into();
        assert!(stored == metadata.into() || stored.is_nan() && metadata.into().is_nan());
        for channel in 0..channels {
            let lower = reader.integer();
            let upper = reader.integer();
            let response: Vec<T> = (0..bins).map(|_| value(reader.scalar(width))).collect();
            if kind < 5 {
                bank.set_channel(channel, &response, lower..upper).unwrap();
            }
            assert_eq!(bank.support(channel), Some(lower..upper));
            for (&actual, &expected) in bank.response(channel).unwrap().iter().zip(&response) {
                assert!(actual == expected || actual.into().is_nan() && expected.into().is_nan());
            }
        }
        let spectra: Vec<T> = (0..3 * bins).map(|_| value(reader.scalar(width))).collect();
        for compression in [Compression::Logarithmic, Compression::Plp] {
            let all = bank.apply_frames(&spectra, 3, compression).unwrap();
            if bins != 0 {
                assert_eq!(bank.apply_spectrogram(&spectra, compression).unwrap(), all);
            }
            for frame in 0..3 {
                let actual = bank
                    .apply(&spectra[frame * bins..(frame + 1) * bins], compression)
                    .unwrap();
                assert_eq!(actual, all[frame * channels..(frame + 1) * channels]);
                for &actual in &actual {
                    let expected = reader.scalar(width);
                    let error = (actual.into() - expected).abs() / expected.abs().max(1.0);
                    maximum_error = maximum_error.max(error);
                    outputs += 1;
                    assert!(
                        error <= if width == 4 { 2e-7 } else { 2e-15 },
                        "record {record}: {} vs {expected}",
                        actual.into()
                    );
                }
            }
            for &actual in &all {
                let expected = reader.scalar(width);
                let error = (actual.into() - expected).abs() / expected.abs().max(1.0);
                maximum_error = maximum_error.max(error);
                outputs += 1;
                assert!(error <= if width == 4 { 2e-7 } else { 2e-15 });
            }
            bank.set_nyquist_hz(value(f64::NEG_INFINITY));
            assert_eq!(bank.nyquist_hz(), value::<T>(f64::NEG_INFINITY));
            assert_eq!(
                bank.clone().apply_frames(&spectra, 3, compression).unwrap(),
                all
            );
        }
    }
    assert!(reader.0.is_empty());
    assert_eq!(outputs, 672);
    eprintln!("Custom filterbank {width}-byte: {outputs} outputs, maximum error {maximum_error:e}");
}

#[test]
fn custom_float_configurations_match_c() {
    custom_compare::<f32>(
        include_bytes!("../../../tests/fixtures/custom_filterbank-float.bin"),
        4,
    );
}

#[test]
fn custom_double_configurations_match_c() {
    custom_compare::<f64>(
        include_bytes!("../../../tests/fixtures/custom_filterbank-double.bin"),
        8,
    );
}

#[test]
fn active_values_and_explicit_empty_frame_counts_are_checked() {
    let mut bank = FilterBank::<f64>::empty(3, f64::NAN, 1).unwrap();
    bank.set_channel(0, &[f64::NAN, 1.0, f64::INFINITY], 1..2)
        .unwrap();
    assert_eq!(
        bank.apply(&[f64::NAN, 1.0, f64::INFINITY], Compression::Plp)
            .unwrap(),
        [1.0]
    );
    assert!(bank.apply(&[0.0, f64::NAN, 0.0], Compression::Plp).is_err());
    assert!(
        bank.set_channel(0, &[f64::NAN, 1.0, f64::INFINITY], 0..2)
            .is_err()
    );
    assert_eq!(bank.support(0), Some(1..2));
    assert!(bank.apply_frames(&[0.0; 3], 2, Compression::Plp).is_err());
    assert!(
        bank.apply_frames(&[], usize::MAX, Compression::Plp)
            .is_err()
    );
    let empty = FilterBank::<f64>::empty(0, 0.0, 2).unwrap();
    assert_eq!(empty.apply(&[], Compression::Plp).unwrap(), [0.0; 2]);
    assert_eq!(
        empty.apply_frames(&[], 3, Compression::Plp).unwrap(),
        [0.0; 6]
    );
    assert!(
        empty
            .apply_frames(&[], usize::MAX, Compression::Plp)
            .is_err()
    );
    assert!(empty.apply_spectrogram(&[], Compression::Plp).is_err());
    for nyquist in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(FilterBank::<f64>::mel_default(6, nyquist, 2, 0.0, 1.0).is_err());
        if !nyquist.is_finite() {
            assert!(FilterBank::<f64>::plp(6, nyquist, 2).is_err());
        }
    }
    assert!(FilterBank::<f64>::mel_default(0, 1.0, 2, 0.0, 1.0).is_err());
    assert_eq!(
        FilterBank::<f64>::plp(0, 1.0, 2)
            .unwrap()
            .apply(&[], Compression::Plp)
            .unwrap(),
        [0.0; 2]
    );
}

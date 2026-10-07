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
    compare::<f32>(include_bytes!("fixtures/c-filterbank-f32.bin"), 4);
}
#[test]
fn double_filterbanks_match_c() {
    compare::<f64>(include_bytes!("fixtures/c-filterbank-f64.bin"), 8);
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
    assert!(FilterBank::<f64>::empty(0, 8000.0, 1).is_err());
    assert!(FilterBank::<f64>::empty(4, 0.0, 1).is_err());
    assert_eq!(
        FilterBank::<f64>::empty(4, 8000.0, 0)
            .unwrap()
            .apply(&[1.0; 4], Compression::Plp)
            .unwrap(),
        []
    );
}

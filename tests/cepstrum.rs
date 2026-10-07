use ciglet_rs::{
    cepstrum::{
        CepstralOptions, band_spectrogram_to_cepstra, band_to_cepstrum, band_to_cepstrum_legacy,
    },
    filterbank::{Compression, FilterBank},
};
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
fn compare<T: FftNum + Into<f64>>(bytes: &[u8], width: usize) {
    let mut reader = Reader(bytes);
    assert_eq!(reader.bytes(4), b"CEP1");
    assert_eq!(reader.integer(), width);
    assert_eq!(reader.integer(), 48);
    let mut count = 0;
    let mut maximum = 0.0f64;
    for record in 0..48 {
        let bands = reader.integer();
        let order = reader.integer();
        let include_dc = reader.integer() != 0;
        let options = CepstralOptions { order, include_dc };
        let coefficients = options.coefficient_count(bands).unwrap();
        let features: Vec<T> = (0..3 * bands)
            .map(|_| T::from_f64(reader.scalar(width)).unwrap())
            .collect();
        let matrix = band_spectrogram_to_cepstra(&features, bands, options).unwrap();
        let mut expected_matrix = Vec::new();
        for frame in 0..3 {
            let full =
                band_to_cepstrum_legacy(&features[frame * bands..(frame + 1) * bands], options)
                    .unwrap();
            let selected =
                band_to_cepstrum(&features[frame * bands..(frame + 1) * bands], options).unwrap();
            assert_eq!(selected, &full[..coefficients]);
            assert_eq!(
                &matrix[frame * coefficients..(frame + 1) * coefficients],
                selected
            );
            for (index, &actual) in full.iter().enumerate() {
                let expected = reader.scalar(width);
                let error = (actual.into() - expected).abs() / expected.abs().max(1.0);
                maximum = maximum.max(error);
                count += 1;
                assert!(
                    error < if width == 4 { 1e-7 } else { 1e-15 },
                    "cepstrum {record}/{frame}/{index}: {} vs {expected}",
                    actual.into()
                );
                expected_matrix.push(expected);
            }
        }
        for expected in expected_matrix {
            assert_eq!(reader.scalar(width), expected);
            count += 1;
        }
    }
    assert!(reader.0.is_empty());
    eprintln!("Cepstrum {width}-byte: {count} original coefficients, normalized error {maximum:e}");
}
#[test]
fn float_cepstra_match_c() {
    compare::<f32>(include_bytes!("fixtures/c-cepstrum-f32.bin"), 4);
}
#[test]
fn double_cepstra_match_c() {
    compare::<f64>(include_bytes!("fixtures/c-cepstrum-f64.bin"), 8);
}
#[test]
fn dc_order_shapes_and_filterbank_pipeline() {
    let options = CepstralOptions {
        order: 12,
        include_dc: true,
    };
    assert_eq!(
        band_to_cepstrum(&[2.0f64], options).unwrap(),
        [2.0 * ciglet_rs::approximation::cosine(0.0).unwrap() as f64]
    );
    assert!(
        band_to_cepstrum(
            &[2.0f64],
            CepstralOptions {
                include_dc: false,
                ..options
            }
        )
        .unwrap()
        .is_empty()
    );
    assert_eq!(options.coefficient_count(20).unwrap(), 13);
    assert_eq!(
        CepstralOptions {
            order: usize::MAX,
            include_dc: true
        }
        .coefficient_count(20)
        .unwrap(),
        20
    );
    assert!(band_to_cepstrum::<f32>(&[], options).is_err());
    assert!(band_to_cepstrum(&[f64::NAN], options).is_err());
    assert!(band_spectrogram_to_cepstra(&[1.0f64; 3], 2, options).is_err());
    assert!(
        band_spectrogram_to_cepstra::<f64>(&[], 2, options)
            .unwrap()
            .is_empty()
    );
    assert!(band_spectrogram_to_cepstra::<f64>(&[], 0, options).is_err());
    let bank = FilterBank::<f64>::mel_default(257, 8000.0, 20, 50.0, 8000.0).unwrap();
    let bands: Vec<f64> = bank
        .apply_spectrogram(&vec![0.0; 3 * 257], Compression::Logarithmic)
        .unwrap()
        .into_iter()
        .map(|x| x.max(-15.0))
        .collect();
    assert!(bands.iter().all(|&x| x == -15.0));
    let cepstra = band_spectrogram_to_cepstra(&bands, 20, options).unwrap();
    assert_eq!(cepstra.len(), 39);
    let expected_dc =
        -15.0 * 20.0f64.sqrt() * ciglet_rs::approximation::cosine(0.0).unwrap() as f64;
    for frame in cepstra.chunks_exact(13) {
        assert!((frame[12] - expected_dc).abs() < 1e-12);
    }
}

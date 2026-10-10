use super::Model;
use crate::statistics::Scalar;

fn reference<T: Scalar>(bytes: &[u8], width: usize) {
    assert_eq!(&bytes[..4], b"LFP1");
    assert_eq!(
        u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize,
        width
    );
    assert_eq!(u32::from_le_bytes(bytes[8..12].try_into().unwrap()), 208);
    assert_eq!(bytes.len(), 12 + 208 * 20 * width);
    let mut maximum = 0.0_f64;
    for (case, record) in bytes[12..].chunks_exact(20 * width).enumerate() {
        let values: Vec<f64> = record
            .chunks_exact(width)
            .map(|value| {
                if width == 4 {
                    f32::from_le_bytes(value.try_into().unwrap()) as f64
                } else {
                    f64::from_le_bytes(value.try_into().unwrap())
                }
            })
            .collect();
        let scalar = |index: usize| T::from_f64(values[index]).unwrap();
        let model = Model {
            period: scalar(0),
            closure: scalar(1),
            peak: scalar(2),
            return_time: scalar(3),
            excitation: scalar(4),
        };
        let prepared = model
            .prepare()
            .unwrap_or_else(|error| panic!("case={case}: {error}"));
        for (index, actual) in [
            (5, prepared.period),
            (6, prepared.closure),
            (8, prepared.return_time),
            (9, prepared.angular_frequency),
            (10, prepared.sine),
            (11, prepared.cosine),
            (12, prepared.decay),
            (14, prepared.growth),
            (15, prepared.initial_excitation),
            (16, prepared.scale),
        ] {
            let actual: f64 = actual.into();
            let expected = values[index];
            let error = (actual - expected).abs() / (1.0 + expected.abs());
            maximum = maximum.max(error);
            if width == 4 {
                assert_eq!(
                    actual.to_bits(),
                    expected.to_bits(),
                    "case={case}, field={index}"
                );
            } else {
                assert!(
                    error <= 2e-14,
                    "case={case}, field={index}: {actual} != {expected}, error={error:e}"
                );
            }
        }
    }
    eprintln!("LF preparation width={width}, maximum normalized difference={maximum:e}");
}

#[test]
fn binary32_preparation_matches_conditioning_corpus() {
    reference::<f32>(
        include_bytes!("../test_data/glottal-preparation-endpoints-float.bin"),
        4,
    );
}

#[test]
fn binary64_preparation_matches_conditioning_corpus() {
    reference::<f64>(
        include_bytes!("../test_data/glottal-preparation-endpoints-double.bin"),
        8,
    );
}

#[test]
fn source_endpoint_fallback_changes_eight_float_preparations() {
    let original = include_bytes!("../test_data/glottal-preparation-original-float.bin");
    let corrected = include_bytes!("../test_data/glottal-preparation-endpoints-float.bin");
    let changed: Vec<_> = original[12..]
        .chunks_exact(80)
        .zip(corrected[12..].chunks_exact(80))
        .enumerate()
        .filter_map(|(case, (a, b))| (a[..68] != b[..68]).then_some(case))
        .collect();
    assert_eq!(changed, [80, 88, 96, 104, 145, 153, 161, 169]);
    let read = |bytes: &[u8], field: usize| {
        let start = 12 + 80 * 80 + field * 4;
        f32::from_le_bytes(bytes[start..start + 4].try_into().unwrap())
    };
    assert_eq!(read(original, 12), 500_000_000.0);
    assert_eq!(read(corrected, 12), 1_000_000_000.0);
    assert_eq!(read(original, 17), 0.5);
    assert_eq!(read(corrected, 17), 0.0);
}

use ciglet_rs::{
    glottal::{Model, Precision},
    statistics::Scalar,
};
fn reference<T: Scalar>(bytes: &[u8], width: usize, tolerance: [f64; 3]) {
    assert_eq!(&bytes[..4], b"LFM1");
    let mut cursor = 4;
    fn integer(bytes: &[u8], cursor: &mut usize) -> usize {
        let x = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
        *cursor += 4;
        x as usize
    }
    fn values<T: Scalar>(bytes: &[u8], cursor: &mut usize, width: usize, n: usize) -> Vec<T> {
        (0..n)
            .map(|_| {
                let x = if width == 4 {
                    f32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap()) as f64
                } else {
                    f64::from_le_bytes(bytes[*cursor..*cursor + 8].try_into().unwrap())
                };
                *cursor += width;
                T::from_f64(x).unwrap()
            })
            .collect()
    }
    assert_eq!(integer(bytes, &mut cursor), width);
    assert_eq!(integer(bytes, &mut cursor), 53);
    let mut maximum = [0.0_f64; 3];
    for case in 0..53 {
        let input = values::<T>(bytes, &mut cursor, width, 3);
        let fields = values::<T>(bytes, &mut cursor, width, 5);
        let model = if input[0] == T::zero() {
            Model {
                period: fields[0],
                closure: fields[1],
                peak: fields[2],
                return_time: fields[3],
                excitation: fields[4],
            }
        } else {
            Model::from_shape(input[0], input[1], input[2]).unwrap()
        };
        for (a, e) in [
            model.period,
            model.closure,
            model.peak,
            model.return_time,
            model.excitation,
        ]
        .into_iter()
        .zip(&fields)
        {
            assert_eq!(
                a.into().to_bits(),
                (*e).into().to_bits(),
                "model case={case}"
            );
        }
        let rate = integer(bytes, &mut cursor) as i32;
        let count = integer(bytes, &mut cursor);
        let nf = integer(bytes, &mut cursor);
        let frequencies = values::<T>(bytes, &mut cursor, width, nf);
        let expected_magnitude = values::<T>(bytes, &mut cursor, width, nf);
        let expected_phase = values::<T>(bytes, &mut cursor, width, nf);
        let expected_wave = values::<T>(bytes, &mut cursor, width, count);
        let prepared = model.prepare().unwrap();
        let response = prepared.spectrum(&frequencies).unwrap();
        let wave = prepared.sampled_period(rate, count).unwrap();
        for (field, (actual, expected)) in [
            (response.magnitude.as_slice(), expected_magnitude.as_slice()),
            (response.phase.as_slice(), expected_phase.as_slice()),
            (wave.as_slice(), expected_wave.as_slice()),
        ]
        .into_iter()
        .enumerate()
        {
            for (index, (&a, &e)) in actual.iter().zip(expected).enumerate() {
                let error = (a.into() - e.into()).abs() / (1.0 + e.into().abs());
                maximum[field] = maximum[field].max(error);
                if width == 4 {
                    assert_eq!(
                        a.into().to_bits(),
                        e.into().to_bits(),
                        "case={case} field={field} index={index}"
                    );
                }
                assert!(
                    error <= tolerance[field],
                    "case={case} field={field} index={index} {} != {} error={error:e}",
                    a.into(),
                    e.into()
                );
            }
        }
    }
    assert_eq!(cursor, bytes.len());
    eprintln!("glottal width={width} maximum normalized errors={maximum:?}");
}
#[test]
fn unchanged_c_reference() {
    reference::<f32>(
        include_bytes!("../../../tests/fixtures/glottal-float.bin"),
        4,
        [0.0; 3],
    );
    reference::<f64>(
        include_bytes!("../../../tests/fixtures/glottal-double.bin"),
        8,
        [1e-17, 3e-15, 5e-16],
    );
}
#[test]
fn scaling_symmetry_truncation_and_reuse() {
    let model = Model::from_shape(1.0_f64, 0.005, 0.5).unwrap();
    let prepared = model.prepare().unwrap();
    let spectrum = prepared.spectrum(&[0.0, 200.0, -200.0, 800.0]).unwrap();
    assert_eq!(spectrum.magnitude[0], 0.0);
    assert_eq!(spectrum.phase[0], 0.0);
    // Source approximate trig does not preserve conjugate symmetry exactly.
    assert!((spectrum.magnitude[1] - spectrum.magnitude[2]).abs() < 2e-10);
    assert!((spectrum.phase[1] + spectrum.phase[2]).abs() < 1e-6);
    let exact = prepared
        .spectrum_with(&[200.0, -200.0], Precision::Exact)
        .unwrap();
    assert_eq!(exact.magnitude[0], exact.magnitude[1]);
    assert_eq!(exact.phase[0], -exact.phase[1]);
    let fast = prepared
        .spectrum_with(&[200.0, 800.0], Precision::Fast)
        .unwrap();
    assert!(
        fast.magnitude
            .iter()
            .chain(&fast.phase)
            .all(|x| x.is_finite())
    );
    let wave = prepared.sampled_period(16000, 128).unwrap();
    assert_eq!(&wave[80..], &[0.0; 48]);
    assert_eq!(prepared.sampled_period(16000, 20).unwrap(), wave[..20]);
    assert_eq!(
        prepared.sampled_period(16000, 0).unwrap(),
        Vec::<f64>::new()
    );
    assert_eq!(prepared.model(), model);
    assert_eq!(
        prepared.spectrum(&[200.0]).unwrap().magnitude[0],
        spectrum.magnitude[1]
    );
    let scaled = Model {
        excitation: 1.0,
        ..model
    }
    .prepare()
    .unwrap();
    let scaled_wave = scaled.sampled_period(16000, 128).unwrap();
    for (&x, &y) in wave.iter().zip(&scaled_wave) {
        assert!((y - 2.0 * x).abs() < 1e-14);
    }
    let scaled_spectrum = scaled.spectrum(&[200.0]).unwrap();
    assert!((scaled_spectrum.magnitude[0] - 2.0 * spectrum.magnitude[1]).abs() < 1e-15);
    let inverted = Model::from_shape(1.0_f64, 0.005, -0.5)
        .unwrap()
        .prepare()
        .unwrap();
    for (&x, &y) in wave
        .iter()
        .zip(inverted.sampled_period(16000, 128).unwrap().iter())
    {
        assert_eq!(y, -x);
    }
    assert_eq!(
        inverted.spectrum(&[200.0]).unwrap().magnitude[0],
        spectrum.magnitude[1]
    );
}
#[test]
fn public_configuration_and_invalid_inputs() {
    let model = Model {
        period: 0.005_f32,
        closure: 0.6,
        peak: 0.4,
        return_time: 0.05,
        excitation: 1.0,
    };
    assert!(model.prepare().is_ok());
    assert!(model.sampled_period(0, 16).is_err());
    assert!(model.spectrum(&[f32::NAN]).is_err());
    for bad in [
        Model {
            period: 0.0,
            ..model
        },
        Model {
            closure: 1.0,
            ..model
        },
        Model { peak: 0.7, ..model },
        Model {
            return_time: 0.5,
            ..model
        },
        Model {
            excitation: f32::INFINITY,
            ..model
        },
        Model {
            period: f32::NAN,
            ..model
        },
    ] {
        assert!(bad.prepare().is_err());
    }
    for shape in [0.0, -1.0, f32::NAN, f32::INFINITY] {
        assert!(Model::from_shape(shape, 0.005, 1.0).is_err());
    }
    let long = Model {
        period: 2.0_f32,
        ..model
    };
    assert_eq!(
        long.sampled_period(i32::MAX, 1).unwrap(),
        long.sampled_period(16000, 1).unwrap()
    );
    let long = Model {
        period: 2.0_f64,
        closure: 0.6,
        peak: 0.4,
        return_time: 0.05,
        excitation: 1.0,
    };
    assert_eq!(
        long.sampled_period(i32::MAX, 1).unwrap(),
        long.sampled_period(16000, 1).unwrap()
    );
}

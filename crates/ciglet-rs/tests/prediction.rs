use ciglet_rs::{
    prediction::{self, ArithmeticPolicy},
    statistics::Scalar,
};
struct Reader<'a> {
    bytes: &'a [u8],
    cursor: usize,
    width: usize,
}
impl Reader<'_> {
    fn integer(&mut self) -> u32 {
        let value =
            u32::from_le_bytes(self.bytes[self.cursor..self.cursor + 4].try_into().unwrap());
        self.cursor += 4;
        value
    }
    fn scalar<T: Scalar>(&mut self) -> T {
        let value = if self.width == 4 {
            f32::from_le_bytes(self.bytes[self.cursor..self.cursor + 4].try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(self.bytes[self.cursor..self.cursor + 8].try_into().unwrap())
        };
        self.cursor += self.width;
        T::from_f64(value).unwrap()
    }
    fn vector<T: Scalar>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.scalar()).collect()
    }
}
fn reference<T: Scalar>(bytes: &[u8]) {
    assert_eq!(&bytes[..4], b"LPC1");
    let mut reader = Reader {
        bytes,
        cursor: 4,
        width: 0,
    };
    reader.width = reader.integer() as usize;
    let count = reader.integer();
    for case in 0..count {
        let length = reader.integer() as usize;
        let order = reader.integer() as usize;
        let samples = reader.vector::<T>(length);
        let autocorrelation = reader.vector::<T>(order + 1);
        let coefficients = reader.vector::<T>(order + 1);
        let gain = reader.scalar::<T>();
        let model =
            prediction::from_samples_with(&samples, order, ArithmeticPolicy::Legacy).unwrap();
        for (field, (actual, expected)) in [
            (&model.autocorrelation, &autocorrelation),
            (&model.coefficients, &coefficients),
        ]
        .into_iter()
        .enumerate()
        {
            for (i, (&a, &b)) in actual.iter().zip(expected).enumerate() {
                assert_eq!(
                    a.into().to_bits(),
                    b.into().to_bits(),
                    "case {case} field {field} sample {i}: {a:?} != {b:?}"
                );
            }
        }
        assert_eq!(
            prediction::levinson_with(&autocorrelation, ArithmeticPolicy::Legacy).unwrap(),
            model.coefficients
        );
        assert_eq!(
            prediction::gain_with(
                &model.coefficients,
                &model.autocorrelation,
                ArithmeticPolicy::Legacy
            )
            .unwrap()
            .into()
            .to_bits(),
            gain.into().to_bits()
        );
        let stable = prediction::from_samples(&samples, order).unwrap();
        for (&actual, &expected) in stable.autocorrelation.iter().zip(&autocorrelation) {
            let error = (actual.into() - expected.into()).abs() / (1.0 + expected.into().abs());
            let bound = if reader.width == 4 { 2e-7 } else { 5e-15 };
            assert!(
                error <= bound,
                "stable correlation case={case}, error={error:e}"
            );
        }
        let energy: f64 = stable.autocorrelation[0].into();
        if energy > 0.0 {
            for row in 1..stable.autocorrelation.len() {
                let mut residual = stable.autocorrelation[row].into() / energy;
                let mut scale = 1.0 + residual.abs();
                for column in 1..stable.coefficients.len() {
                    let covariance = if row == column {
                        1.000001
                    } else {
                        stable.autocorrelation[row.abs_diff(column)].into() / energy
                    };
                    let term = covariance * stable.coefficients[column].into();
                    residual += term;
                    scale += term.abs();
                }
                let bound = if reader.width == 4 { 2e-7 } else { 5e-15 };
                assert!(
                    residual.abs() / scale <= bound,
                    "stable waveform case={case}, row={row}, residual={residual:e}"
                );
            }
        }
        assert!(
            prediction::gain(&stable.coefficients, &stable.autocorrelation)
                .unwrap()
                .into()
                .is_finite()
        );
    }
    assert_eq!(reader.cursor, bytes.len());
}
#[test]
fn c_float_reference() {
    reference::<f32>(include_bytes!(
        "../../../tests/fixtures/prediction-float.bin"
    ));
}
#[test]
fn c_double_reference() {
    reference::<f64>(include_bytes!(
        "../../../tests/fixtures/prediction-double.bin"
    ));
}
#[test]
fn regularized_toeplitz_equations_and_safe_boundaries() {
    let correlation = [2.0f64, 0.8, 0.2, -0.1, 0.03];
    let coefficients = prediction::levinson(&correlation).unwrap();
    for row in 1..correlation.len() {
        let mut residual = correlation[row];
        for column in 1..correlation.len() {
            let covariance = if row == column {
                correlation[0] * 1.000001
            } else {
                correlation[row.abs_diff(column)]
            };
            residual += covariance * coefficients[column];
        }
        assert!(
            residual.abs() < 1e-14,
            "Toeplitz equation {row}: {residual}"
        );
    }
    assert_eq!(prediction::levinson(&[3.0f32]).unwrap(), vec![1.0]);
    assert_eq!(
        prediction::from_samples(&[3.0f64], 0)
            .unwrap()
            .autocorrelation,
        vec![9.0]
    );
    let silence = prediction::from_samples(&[0.0f32; 8], 7).unwrap();
    assert_eq!(
        silence.coefficients,
        vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
    );
    assert_eq!(
        prediction::gain(&silence.coefficients, &silence.autocorrelation).unwrap(),
        0.0
    );
    assert!(prediction::levinson::<f64>(&[]).is_err());
    assert!(prediction::levinson(&[-1.0f32, 0.0]).is_err());
    assert!(prediction::levinson(&[0.0f32, 1.0]).is_err());
    assert!(prediction::levinson(&[1.0f64, 2.0]).is_err());
    assert!(prediction::levinson(&[1.0f32, f32::NAN]).is_err());
    assert!(prediction::from_samples(&[1.0f64; 4], 4).is_err());
    assert!(prediction::from_samples(&[1.0f64; 4], usize::MAX).is_err());
    assert!(prediction::gain(&[1.0f32], &[-1.0]).is_err());
    assert!(prediction::gain(&[1.0f64], &[]).is_err());
}

#[test]
fn extreme_energy_and_scaled_gain_remain_representable() {
    let coefficients = prediction::levinson(&[f64::MAX, f64::MAX * 0.5]).unwrap();
    assert!((coefficients[1] + 0.5 / 1.000001).abs() < 2e-15);
    let tiny = f64::from_bits(1);
    let coefficients = prediction::levinson(&[tiny * 4.0, tiny * 2.0]).unwrap();
    assert!((coefficients[1] + 0.5 / 1.000001).abs() < 2e-15);
    let gain = prediction::gain(&[2.0, -1.0], &[f64::MAX, f64::MAX]).unwrap();
    assert!((gain / f64::MAX.sqrt() - 1.0).abs() < 2e-15);
    assert_eq!(prediction::gain(&[tiny], &[tiny]).unwrap(), tiny);
    let tiny = f32::from_bits(1);
    let coefficients = prediction::levinson(&[tiny * 4.0, tiny * 2.0]).unwrap();
    assert!((coefficients[1] as f64 + 0.5 / 1.000001).abs() < 2e-8);
    let gain = prediction::gain(&[2.0_f32, -1.0], &[f32::MAX, f32::MAX]).unwrap();
    assert!((gain as f64 / (f32::MAX as f64).sqrt() - 1.0).abs() < 1e-7);
    assert_eq!(prediction::gain(&[tiny], &[tiny]).unwrap(), tiny);
    assert!(prediction::gain(&[2.0_f64, -3.0], &[f64::MAX, f64::MAX]).is_err());
    assert!(
        prediction::gain_with(
            &[2.0_f64, -1.0],
            &[f64::MAX, f64::MAX],
            ArithmeticPolicy::Legacy
        )
        .is_err()
    );
}

#[test]
fn scaled_waveforms_preserve_large_and_tiny_powers_and_gain() {
    for amplitude in [f64::MAX, f64::from_bits(1)] {
        let model = prediction::from_samples_scaled(&[amplitude, 0.0], 1).unwrap();
        assert_eq!(model.coefficients(), [1.0, -0.0]);
        assert_eq!(model.gain().unwrap(), amplitude);
        assert_eq!(model.autocorrelation_parts().nth(1).unwrap(), (0.0, 0));
        assert!(model.into_prediction().is_err());
    }
    for amplitude in [f32::MAX, f32::from_bits(1)] {
        let model = prediction::from_samples_scaled(&[amplitude, 0.0], 1).unwrap();
        assert_eq!(model.coefficients(), [1.0, -0.0]);
        assert_eq!(model.gain().unwrap(), amplitude);
        assert!(model.into_prediction().is_err());
    }
    let tiny = f64::from_bits(1);
    assert!(prediction::from_samples(&[tiny], 0).is_err());
    let cancelled = prediction::from_samples_scaled(&[f64::MAX, 1.0, -f64::MAX], 2).unwrap();
    assert_eq!(cancelled.autocorrelation_parts().nth(1).unwrap(), (0.0, 0));
    assert!(cancelled.gain().is_err());
    let (fraction, exponent) = cancelled.gain_parts().unwrap();
    assert!((1.0..2.0).contains(&fraction));
    assert_eq!(exponent, 1024);
    let silence = prediction::from_samples_scaled(&[0.0_f64; 4], 3).unwrap();
    assert_eq!(silence.gain_parts().unwrap(), (0.0, 0));
    assert_eq!(
        silence.into_prediction().unwrap().coefficients,
        [1.0, 0.0, 0.0, 0.0]
    );
    assert!(prediction::from_samples_scaled::<f64>(&[], 0).is_err());
    assert!(prediction::from_samples_scaled(&[f64::NAN], 0).is_err());
}

fn stable_reference<T: Scalar>(bytes: &[u8], tolerance: f64) {
    assert_eq!(&bytes[..4], b"SLP1");
    let mut reader = Reader {
        bytes,
        cursor: 4,
        width: 0,
    };
    reader.width = reader.integer() as usize;
    assert_eq!(reader.integer(), 100);
    let mut maximum = 0.0_f64;
    let mut outputs = 0;
    for case in 0..100 {
        let length = reader.integer() as usize;
        let correlation = reader.vector::<T>(length);
        let expected = reader.vector::<T>(length);
        let actual = prediction::levinson(&correlation).unwrap();
        for (index, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
            let error = (actual.into() - expected.into()).abs() / (1.0 + expected.into().abs());
            assert!(error.is_finite(), "case={case} index={index}");
            maximum = maximum.max(error);
            outputs += 1;
        }
    }
    assert_eq!(outputs, 400);
    assert_eq!(reader.cursor, bytes.len());
    println!(
        "Stable prediction {}-byte maximum normalized coefficient error={maximum:e}",
        reader.width
    );
    assert!(
        maximum <= tolerance,
        "maximum error={maximum:e}, bound={tolerance:e}"
    );
}

#[test]
fn exact_toeplitz_float_reference() {
    stable_reference::<f32>(
        include_bytes!("../../../tests/fixtures/stable-prediction-float.bin"),
        2e-15,
    );
}

#[test]
fn exact_toeplitz_double_reference() {
    stable_reference::<f64>(
        include_bytes!("../../../tests/fixtures/stable-prediction-double.bin"),
        6e-11,
    );
}

fn scaled_waveform_reference<T: Scalar>(bytes: &[u8]) {
    assert_eq!(&bytes[..4], b"SWV1");
    let mut reader = Reader {
        bytes,
        cursor: 4,
        width: 0,
    };
    reader.width = reader.integer() as usize;
    assert_eq!(reader.integer(), 64);
    let mut maximum = [0.0_f64; 2];
    let mut outputs = 0;
    for case in 0..64 {
        let length = reader.integer() as usize;
        let order = reader.integer() as usize;
        let samples = reader.vector::<T>(length);
        let model = prediction::from_samples_scaled(&samples, order).unwrap();
        for (actual, exponent) in model.autocorrelation_parts() {
            let expected =
                f64::from_le_bytes(bytes[reader.cursor..reader.cursor + 8].try_into().unwrap());
            reader.cursor += 8;
            let expected_exponent =
                i32::from_le_bytes(bytes[reader.cursor..reader.cursor + 4].try_into().unwrap());
            reader.cursor += 4;
            let error = if expected == 0.0 {
                actual.abs()
            } else {
                (actual * 2.0_f64.powi(exponent - expected_exponent) / expected - 1.0).abs()
            };
            assert!(
                error <= 2e-15 && (expected != 0.0 || actual == 0.0),
                "case={case}, scaled correlation error={error:e}"
            );
            maximum[0] = maximum[0].max(error);
            outputs += 1;
        }
        let expected = reader.vector::<T>(order + 1);
        for (&actual, expected) in model.coefficients().iter().zip(expected) {
            let error = (actual.into() - expected.into()).abs() / (1.0 + expected.into().abs());
            let bound = if reader.width == 4 { 2e-15 } else { 5e-15 };
            assert!(error <= bound, "case={case}, coefficient error={error:e}");
            maximum[1] = maximum[1].max(error);
        }
    }
    assert_eq!(outputs, 176);
    assert_eq!(reader.cursor, bytes.len());
    println!(
        "Scaled waveform {}-byte maxima (correlations, coefficients)={maximum:?}",
        reader.width
    );
}

#[test]
fn exact_scaled_waveform_float_reference() {
    scaled_waveform_reference::<f32>(include_bytes!(
        "../../../tests/fixtures/scaled-waveform-float.bin"
    ));
}

#[test]
fn exact_scaled_waveform_double_reference() {
    scaled_waveform_reference::<f64>(include_bytes!(
        "../../../tests/fixtures/scaled-waveform-double.bin"
    ));
}

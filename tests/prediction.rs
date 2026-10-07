use ciglet_rs::{prediction, statistics::Scalar};
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
        let model = prediction::from_samples(&samples, order).unwrap();
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
            prediction::levinson(&autocorrelation).unwrap(),
            model.coefficients
        );
        assert_eq!(
            prediction::gain(&model.coefficients, &model.autocorrelation)
                .unwrap()
                .into()
                .to_bits(),
            gain.into().to_bits()
        );
    }
    assert_eq!(reader.cursor, bytes.len());
}
#[test]
fn c_float_reference() {
    reference::<f32>(include_bytes!("fixtures/prediction-float.bin"));
}
#[test]
fn c_double_reference() {
    reference::<f64>(include_bytes!("fixtures/prediction-double.bin"));
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

use ciglet_rs::{
    fourier::Complex,
    interpolation,
    phase::{self, NyquistPolicy, Symmetry},
    statistics::Scalar,
    vector,
};
use rustfft::FftNum;
struct Reader<'a> {
    bytes: &'a [u8],
    cursor: usize,
    width: usize,
}
impl Reader<'_> {
    fn integer(&mut self) -> u32 {
        let x = u32::from_le_bytes(self.bytes[self.cursor..self.cursor + 4].try_into().unwrap());
        self.cursor += 4;
        x
    }
    fn scalar<T: FftNum>(&mut self) -> T {
        let x = if self.width == 4 {
            f32::from_le_bytes(self.bytes[self.cursor..self.cursor + 4].try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(self.bytes[self.cursor..self.cursor + 8].try_into().unwrap())
        };
        self.cursor += self.width;
        T::from_f64(x).unwrap()
    }
    fn vector<T: FftNum>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.scalar()).collect()
    }
}
fn check<T: Scalar>(bytes: &[u8]) {
    assert_eq!(&bytes[..4], b"VPH1");
    let mut r = Reader {
        bytes,
        cursor: 4,
        width: 0,
    };
    r.width = r.integer() as usize;
    let count = r.integer();
    let mut maximum_argument_error = 0.0f64;
    for case in 0..count {
        let n = r.integer() as usize;
        let real = r.vector::<T>(n);
        let imaginary = r.vector::<T>(n);
        let angles = r.vector::<T>(n);
        let complex: Vec<_> = real
            .iter()
            .zip(&imaginary)
            .map(|(&re, &im)| Complex::new(re, im))
            .collect();
        let magnitudes = phase::magnitude(&complex).unwrap();
        let mut symmetric = real.clone();
        phase::complete_spectrum_with(&mut symmetric, Symmetry::Symmetric, NyquistPolicy::Legacy)
            .unwrap();
        let mut antisymmetric = real.clone();
        phase::complete_spectrum_with(
            &mut antisymmetric,
            Symmetry::Antisymmetric,
            NyquistPolicy::Legacy,
        )
        .unwrap();
        let actual = [
            vector::difference(&real).unwrap(),
            vector::cumulative_sum(&real).unwrap(),
            vector::reverse(&real),
            phase::unwrap(&angles).unwrap(),
            angles.iter().map(|&x| phase::wrap(x).unwrap()).collect(),
            angles
                .iter()
                .enumerate()
                .map(|(i, &x)| phase::difference(x, angles[(i + 3) % n]).unwrap())
                .collect(),
            magnitudes.clone(),
            phase::argument(&complex).unwrap(),
            phase::polar_real(&magnitudes, &angles).unwrap(),
            phase::polar_imaginary(&magnitudes, &angles).unwrap(),
            symmetric,
            antisymmetric,
        ];
        for (field, observed) in actual.iter().enumerate() {
            let expected = r.vector::<T>(n);
            for (i, (&a, &b)) in observed.iter().zip(&expected).enumerate() {
                if field == 7 && a != T::zero() && b != T::zero() {
                    let error = (a.into() - b.into()).abs() / (1.0 + b.into().abs());
                    maximum_argument_error = maximum_argument_error.max(error);
                    assert_eq!(
                        a.into().to_bits(),
                        b.into().to_bits(),
                        "atan2 case {case} sample {i}: {error}"
                    );
                } else {
                    assert_eq!(
                        a.into().to_bits(),
                        b.into().to_bits(),
                        "case {case} field {field} sample {i}: {a:?} {b:?}"
                    );
                }
            }
        }
    }
    let grids = r.integer();
    for case in 0..grids {
        let n = r.integer() as usize;
        let a = r.scalar::<T>();
        let b = r.scalar::<T>();
        let expected = r.vector::<T>(n);
        assert_eq!(
            vector::linear_grid(a, b, n).unwrap(),
            expected,
            "grid {case}"
        );
        let ratio = r.scalar::<T>();
        let expected = r.scalar::<T>();
        assert_eq!(interpolation::between(a, b, ratio).unwrap(), expected);
    }
    let sequences = r.integer();
    for _ in 0..sequences {
        let n = r.integer() as usize;
        let a = r.integer() as i32;
        let step = r.integer() as i32;
        let expected: Vec<_> = (0..n).map(|_| r.integer() as i32).collect();
        assert_eq!(vector::integer_sequence(a, step, n).unwrap(), expected);
    }
    assert_eq!(r.cursor, bytes.len());
    println!(
        "{}-byte atan2 maximum normalized difference: {maximum_argument_error}",
        r.width
    );
}
#[test]
fn c_float() {
    check::<f32>(include_bytes!("fixtures/vector-phase-float.bin"));
}
#[test]
fn c_double() {
    check::<f64>(include_bytes!("fixtures/vector-phase-double.bin"));
}
#[test]
fn empty_singleton_range_loss_and_completion_boundaries() {
    assert!(vector::difference::<f32>(&[]).unwrap().is_empty());
    assert!(vector::cumulative_sum::<f64>(&[]).unwrap().is_empty());
    assert!(phase::unwrap::<f64>(&[]).unwrap().is_empty());
    assert_eq!(
        vector::difference(&[-0.0f32]).unwrap()[0].to_bits(),
        (-0.0f32).to_bits()
    );
    assert_eq!(
        vector::cumulative_sum(&[-0.0f64]).unwrap()[0].to_bits(),
        (-0.0f64).to_bits()
    );
    assert_eq!(vector::linear_grid(2.0f64, 3.0, 1).unwrap(), vec![2.0]);
    assert!(vector::linear_grid(2.0f32, 3.0, 0).unwrap().is_empty());
    assert_eq!(
        vector::linear_grid(-f64::MAX, f64::MAX, 3).unwrap(),
        vec![-f64::MAX, 0.0, f64::MAX]
    );
    assert_eq!(
        interpolation::between(-f32::MAX, f32::MAX, 0.5).unwrap(),
        0.0
    );
    assert_eq!(
        phase::magnitude_of(Complex::new(f64::MAX, 0.0)).unwrap(),
        f64::MAX
    );
    assert_eq!(
        phase::magnitude_of(Complex::new(1e-30f32, 0.0)).unwrap(),
        1e-30
    );
    assert!(phase::magnitude_of(Complex::new(f64::MAX, f64::MAX)).is_err());
    let mut spectrum = [1.0f64, 2.0, 3.0, 4.0, 5.0, f64::NAN, f64::NAN, f64::NAN];
    phase::complete_spectrum(&mut spectrum, Symmetry::Antisymmetric).unwrap();
    assert_eq!(spectrum, [1.0, 2.0, 3.0, 4.0, 5.0, -4.0, -3.0, -2.0]);
    let mut legacy = [2.0f32, f32::NAN];
    phase::complete_spectrum_with(&mut legacy, Symmetry::Antisymmetric, NyquistPolicy::Legacy)
        .unwrap();
    assert_eq!(legacy, [2.0, 2.0]);
    assert!(phase::complete_spectrum::<f32>(&mut [], Symmetry::Symmetric).is_ok());
    assert_eq!(
        vector::integer_sequence(-2_000_000_000, 2_000_000_000, 3).unwrap(),
        vec![-2_000_000_000, 0, 2_000_000_000]
    );
    assert!(vector::integer_sequence(i32::MAX, 1, 2).is_err());
    assert!(vector::linear_grid(0.0f64, 1.0, usize::MAX).is_err());
    assert!(vector::difference(&[f32::NAN]).is_err());
    assert!(phase::wrap(f64::INFINITY).is_err());
    assert!(phase::difference(-1e20f64, 1e20).is_err());
    assert!(phase::polar_real(&[1.0f32], &[]).is_err());
    assert_eq!(
        phase::wrap(-std::f64::consts::PI).unwrap(),
        std::f64::consts::PI
    );
    assert_eq!(
        phase::difference(-std::f64::consts::PI, 0.0).unwrap(),
        -std::f64::consts::PI
    );
}

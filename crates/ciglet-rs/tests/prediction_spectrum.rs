use ciglet_rs::{
    prediction::{self, ArithmeticPolicy, NyquistPolicy, SpectrumPlan},
    statistics::Scalar,
};
struct Reader<'a> {
    bytes: &'a [u8],
    cursor: usize,
    width: usize,
}
impl Reader<'_> {
    fn integer(&mut self) -> usize {
        let value =
            u32::from_le_bytes(self.bytes[self.cursor..self.cursor + 4].try_into().unwrap());
        self.cursor += 4;
        value as usize
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
fn measure<T: Scalar>(actual: T, expected: T, maximum: &mut f64) {
    let error = (actual.into() - expected.into()).abs() / (1.0 + expected.into().abs());
    assert!(error.is_finite());
    *maximum = maximum.max(error);
}
fn reference<T: Scalar>(bytes: &[u8]) {
    assert_eq!(&bytes[..4], b"LPS1");
    let mut reader = Reader {
        bytes,
        cursor: 4,
        width: 0,
    };
    reader.width = reader.integer();
    let cases = reader.integer();
    let mut maximum = [0.0; 5];
    for _ in 0..cases {
        let bins = reader.integer();
        let length = reader.integer();
        let order = reader.integer();
        let input = reader.vector::<T>(bins);
        let correlation = reader.vector::<T>(order + 1);
        let coefficients = reader.vector::<T>(order + 1);
        let gain = reader.scalar::<T>();
        let spectrum = reader.vector::<T>(length);
        let mut plan = SpectrumPlan::new(length).unwrap();
        let actual = plan
            .estimate_with_arithmetic(
                &input,
                order,
                NyquistPolicy::Legacy,
                ArithmeticPolicy::Legacy,
            )
            .unwrap();
        assert_eq!(
            actual,
            prediction::from_magnitude_with_arithmetic(
                &input,
                order,
                NyquistPolicy::Legacy,
                ArithmeticPolicy::Legacy
            )
            .unwrap()
        );
        assert_eq!(
            actual,
            plan.estimate_with_arithmetic(
                &input,
                order,
                NyquistPolicy::Legacy,
                ArithmeticPolicy::Legacy
            )
            .unwrap()
        );
        for (&a, &b) in actual.autocorrelation.iter().zip(&correlation) {
            measure(a, b, &mut maximum[0]);
        }
        for (&a, &b) in actual.coefficients.iter().zip(&coefficients) {
            measure(a, b, &mut maximum[1]);
        }
        let actual_gain = prediction::gain_with(
            &actual.coefficients,
            &actual.autocorrelation,
            ArithmeticPolicy::Legacy,
        )
        .unwrap();
        measure(actual_gain, gain, &mut maximum[2]);
        // Use C coefficients/gain to isolate forward FFT/spectrum arithmetic.
        for (&a, &b) in plan
            .spectrum(&coefficients, gain)
            .unwrap()
            .iter()
            .zip(&spectrum)
        {
            measure(a, b, &mut maximum[3]);
        }
        for (&a, &b) in plan
            .spectrum(&actual.coefficients, actual_gain)
            .unwrap()
            .iter()
            .zip(&spectrum)
        {
            measure(a, b, &mut maximum[4]);
        }
        assert_eq!(
            prediction::levinson_with(&correlation, ArithmeticPolicy::Legacy).unwrap(),
            coefficients
        );
        let stable = plan.estimate(&input, order, NyquistPolicy::Legacy).unwrap();
        assert_eq!(stable.autocorrelation, actual.autocorrelation);
        assert_eq!(
            stable.coefficients,
            prediction::levinson(&stable.autocorrelation).unwrap()
        );
        let stable_gain = prediction::gain(&stable.coefficients, &stable.autocorrelation).unwrap();
        assert!(
            plan.spectrum(&stable.coefficients, stable_gain)
                .unwrap()
                .iter()
                .all(|&value| value.into().is_finite())
        );
    }
    let cases = reader.integer();
    for _ in 0..cases {
        let length = reader.integer();
        let coefficients = reader.vector::<T>(length);
        let count = reader.integer();
        let frequencies = reader.vector::<T>(count);
        let poles: Vec<_> = (0..count)
            .map(|_| (reader.scalar::<T>(), reader.scalar::<T>()))
            .collect();
        let result = prediction::resonances(&coefficients).unwrap();
        assert_eq!(result.resonances.len(), count);
        for ((actual, &frequency), &(real, imaginary)) in
            result.resonances.iter().zip(&frequencies).zip(&poles)
        {
            assert_eq!(
                actual.frequency_nyquist.into().to_bits(),
                frequency.into().to_bits()
            );
            assert_eq!(actual.pole.re.into().to_bits(), real.into().to_bits());
            assert_eq!(actual.pole.im.into().to_bits(), imaginary.into().to_bits());
        }
    }
    assert_eq!(reader.cursor, bytes.len());
    let bounds = if reader.width == 4 {
        [2e-8, 3e-8, 0.0, 7e-8, 8e-8]
    } else {
        [5e-17, 2e-15, 4e-16, 2e-15, 3e-15]
    };
    for (field, (&error, &bound)) in maximum.iter().zip(&bounds).enumerate() {
        assert!(
            error <= bound,
            "prediction field {field}: {error} exceeds measured bound {bound}"
        );
    }
    println!(
        "{}-byte spectrum prediction maxima (correlation, coefficients, gain, isolated spectrum, complete spectrum): {maximum:?}",
        reader.width
    );
}
#[test]
fn c_float_reference() {
    reference::<f32>(include_bytes!(
        "../../../tests/fixtures/prediction-spectrum-float.bin"
    ));
}
#[test]
fn c_double_reference() {
    reference::<f64>(include_bytes!(
        "../../../tests/fixtures/prediction-spectrum-double.bin"
    ));
}
#[test]
fn nyquist_silence_sizing_and_analytic_spectrum() {
    let corrected = prediction::from_magnitude(&[0.0f64, 0.0, 2.0], 0).unwrap();
    assert_eq!(corrected.autocorrelation, vec![1.0]);
    let legacy =
        prediction::from_magnitude_with(&[0.0f64, 0.0, 2.0], 0, NyquistPolicy::Legacy).unwrap();
    assert_eq!(legacy.autocorrelation, vec![0.0]);
    let discarded =
        prediction::from_magnitude_with(&[1.0f64, 1.0, f64::NAN], 1, NyquistPolicy::Legacy)
            .unwrap();
    assert_eq!(discarded.autocorrelation, vec![1.0, 0.0]);
    assert_eq!(
        discarded,
        prediction::from_magnitude_with(&[1.0f64, 1.0, f64::MAX], 1, NyquistPolicy::Legacy)
            .unwrap()
    );
    assert!(prediction::from_magnitude(&[1.0f64, 1.0, f64::NAN], 1).is_err());
    assert!(
        SpectrumPlan::<f64>::new(16)
            .unwrap()
            .estimate(&[1.0, 1.0, f64::NAN], 1, NyquistPolicy::Legacy)
            .is_err()
    );
    let mut plan = SpectrumPlan::<f64>::new(16).unwrap();
    let silence = plan
        .estimate(&[0.0; 9], 7, NyquistPolicy::Preserve)
        .unwrap();
    assert_eq!(silence.coefficients[0], 1.0);
    assert!(silence.coefficients[1..].iter().all(|&x| x == 0.0));
    let response = plan.spectrum(&[1.0, -0.5], 2.0).unwrap();
    let negative = plan.spectrum(&[1.0, -0.5], -2.0).unwrap();
    assert_eq!(
        negative,
        response.iter().map(|&value| -value).collect::<Vec<_>>()
    );
    for (i, &value) in response.iter().enumerate() {
        let angle = std::f64::consts::TAU * i as f64 / 16.0;
        let expected = 2.0 / (1.25 - angle.cos()).sqrt();
        assert!((value - expected).abs() < 3e-15);
    }
    assert_eq!(plan.spectrum(&[1.0, -1.0], 0.0).unwrap(), vec![0.0; 16]);
    assert!(plan.spectrum(&[1.0, -1.0], 1.0).is_err());
    assert_eq!(plan.spectrum(&[f64::MAX], f64::MAX).unwrap(), vec![1.0; 16]);
    assert_eq!(
        plan.spectrum(&[f64::from_bits(1)], f64::from_bits(1))
            .unwrap(),
        vec![1.0; 16]
    );
    assert_eq!(
        SpectrumPlan::<f32>::new(1)
            .unwrap()
            .spectrum(&[1.0], 2.0)
            .unwrap(),
        vec![2.0]
    );
    assert!(plan.spectrum(&[0.0], 1.0).is_err());
    assert!(plan.spectrum(&[1.0; 17], 1.0).is_err());
    assert!(prediction::from_magnitude(&[1.0f32], 0).is_err());
    assert!(
        plan.estimate(&[1.0; 9], 16, NyquistPolicy::Preserve)
            .is_err()
    );
    assert!(
        plan.estimate(&[f64::MAX; 9], 1, NyquistPolicy::Preserve)
            .is_err()
    );
    assert!(
        plan.estimate(&[f64::NAN; 9], 1, NyquistPolicy::Preserve)
            .is_err()
    );
    assert_eq!(
        plan.estimate(&[1.0; 9], 1, NyquistPolicy::Preserve)
            .unwrap()
            .coefficients,
        vec![1.0, -0.0]
    );
    let resonances = prediction::resonances(&[1.0f64, 0.0, 0.0, 0.0, 0.25]).unwrap();
    assert!(resonances.converged);
    assert_eq!(resonances.resonances.len(), 2);
    for (pole, expected) in resonances.resonances.iter().zip([0.25, 0.75]) {
        assert!((pole.frequency_nyquist - expected).abs() < 1e-6);
    }
    assert!(
        prediction::resonances(&[1.0f64])
            .unwrap()
            .resonances
            .is_empty()
    );
    assert!(prediction::resonances::<f64>(&[]).is_err());
}

#[test]
fn overflowing_fft_preserves_representable_prediction_ratios() {
    fn check<T: Scalar>(large: T, tolerance: f64) {
        let mut plan = SpectrumPlan::<T>::new(16).unwrap();
        let half = T::from_f64(large.into() * 0.5).unwrap();
        for sign in [1.0, -1.0] {
            let gain = T::from_f64(sign * large.into()).unwrap();
            let spectrum = plan.spectrum(&[large, half], gain).unwrap();
            for (bin, value) in spectrum.into_iter().enumerate() {
                let angle = std::f64::consts::TAU * bin as f64 / 16.0;
                let expected = sign / (1.25 + angle.cos()).sqrt();
                assert!((value.into() / expected - 1.0).abs() <= tolerance);
            }
        }
        let one = T::from_f64(1.0).unwrap();
        let minus = T::from_f64(-large.into()).unwrap();
        let cancelled = plan.spectrum(&[large, one, minus], one).unwrap();
        assert_eq!(cancelled[0], one);
        assert_eq!(cancelled[8], one);
        assert!(plan.spectrum(&[large, minus], one).is_err());
        assert_eq!(
            plan.spectrum(&[large, minus], T::zero()).unwrap(),
            vec![T::zero(); 16]
        );
        assert_eq!(plan.spectrum(&[one], one).unwrap(), vec![one; 16]);
    }
    check(f32::MAX, 2e-7);
    check(f64::MAX, 4e-15);
}

#[test]
fn scaled_magnitudes_and_overflowing_inverse_fft_preserve_prediction() {
    fn check<T: Scalar>(amplitude: T) {
        let mut plan = SpectrumPlan::<T>::new(4).unwrap();
        let model = plan
            .estimate_scaled(&[amplitude; 3], 3, NyquistPolicy::Preserve)
            .unwrap();
        assert_eq!(
            model.coefficients(),
            [T::one(), T::zero(), T::zero(), T::zero()]
        );
        assert!(
            model
                .autocorrelation_parts()
                .skip(1)
                .all(|parts| parts == (0.0, 0))
        );
        assert_eq!(model.gain().unwrap(), amplitude);
        assert!(model.into_prediction().is_err());
        let discarded = plan
            .estimate_scaled(
                &[amplitude, amplitude, T::from_f64(f64::NAN).unwrap()],
                3,
                NyquistPolicy::Legacy,
            )
            .unwrap();
        assert_eq!(discarded.gain().unwrap(), amplitude);
        assert!(
            plan.estimate_scaled(&[amplitude; 3], 4, NyquistPolicy::Preserve)
                .is_err()
        );
        assert!(
            plan.estimate_scaled(
                &[T::from_f64(f64::NAN).unwrap(); 3],
                1,
                NyquistPolicy::Preserve
            )
            .is_err()
        );
        assert_eq!(
            plan.estimate(&[T::one(); 3], 1, NyquistPolicy::Preserve)
                .unwrap()
                .autocorrelation,
            [T::one(), T::zero()]
        );
    }
    check(f32::MAX);
    check(f32::from_bits(1));
    check(f64::MAX);
    check(f64::from_bits(1));
    let amplitude = f64::MAX.sqrt() * 0.5;
    let model = SpectrumPlan::<f64>::new(16)
        .unwrap()
        .estimate(&[amplitude; 9], 3, NyquistPolicy::Preserve)
        .unwrap();
    for &coefficient in &model.coefficients[1..] {
        assert!(coefficient.abs() < 2e-15);
    }
    assert!(
        (prediction::gain(&model.coefficients, &model.autocorrelation).unwrap() / amplitude - 1.0)
            .abs()
            < 2e-15
    );
    let model =
        prediction::from_magnitude_scaled(&[f64::from_bits(1); 3], 3, NyquistPolicy::Preserve)
            .unwrap();
    assert_eq!(model.gain().unwrap(), f64::from_bits(1));
    for amplitude in [f64::MAX, f64::from_bits(1)] {
        let model = prediction::from_magnitude_scaled(
            &[0.0, amplitude, 0.0, 0.0, 0.0],
            3,
            NyquistPolicy::Preserve,
        )
        .unwrap();
        let correlations: Vec<_> = model.autocorrelation_parts().collect();
        for (lag, expected) in [
            1.0,
            std::f64::consts::FRAC_1_SQRT_2,
            0.0,
            -std::f64::consts::FRAC_1_SQRT_2,
        ]
        .into_iter()
        .enumerate()
        {
            let ratio = if correlations[lag].0 == 0.0 {
                0.0
            } else {
                correlations[lag].0 / correlations[0].0
                    * 2.0_f64.powi(correlations[lag].1 - correlations[0].1)
            };
            assert!(
                (ratio - expected).abs() < 4e-15,
                "amplitude={amplitude:e}, lag={lag}, ratio={ratio:e}, expected={expected:e}"
            );
        }
        let (fraction, exponent) = model.gain_parts().unwrap();
        assert!((1.0..2.0).contains(&fraction));
        if amplitude == f64::MAX {
            assert!(model.gain().unwrap() > 0.0);
        } else {
            assert!(exponent < -1074);
            assert!(model.gain().is_err());
        }
    }
}

#[test]
fn subnormal_fft_energy_uses_scaled_coefficient_estimation() {
    let mut magnitudes = [0.0_f32; 33];
    magnitudes[8] = f32::MIN_POSITIVE.sqrt();
    let mut plan = SpectrumPlan::<f32>::new(64).unwrap();
    let expected = -std::f64::consts::FRAC_1_SQRT_2 / 1.000001;
    let legacy = plan
        .estimate_with_arithmetic(
            &magnitudes,
            1,
            NyquistPolicy::Preserve,
            ArithmeticPolicy::Legacy,
        )
        .unwrap();
    assert!((legacy.coefficients[1] as f64 - expected).abs() > 2e-7);
    let stable = plan
        .estimate_scaled(&magnitudes, 1, NyquistPolicy::Preserve)
        .unwrap();
    assert!((stable.coefficients()[1] as f64 - expected).abs() < 3e-8);
}

use ciglet_rs::{
    real_cepstrum::{self, Lifter, NyquistPolicy, Plan},
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
    fn vector<T: Scalar>(&mut self, n: usize) -> Vec<T> {
        (0..n)
            .map(|_| {
                let bytes = &self.bytes[self.cursor..self.cursor + self.width];
                let value = if self.width == 4 {
                    f32::from_le_bytes(bytes.try_into().unwrap()) as f64
                } else {
                    f64::from_le_bytes(bytes.try_into().unwrap())
                };
                self.cursor += self.width;
                T::from_f64(value).unwrap()
            })
            .collect()
    }
}
fn measure<T: Scalar>(actual: &[T], expected: &[T], maximum: &mut f64) {
    assert_eq!(actual.len(), expected.len());
    for (&a, &b) in actual.iter().zip(expected) {
        let error = (a.into() - b.into()).abs() / (1.0 + b.into().abs());
        assert!(error.is_finite());
        *maximum = maximum.max(error);
    }
}
fn reference<T: Scalar>(bytes: &[u8]) {
    assert_eq!(&bytes[..4], b"RCP1");
    let mut reader = Reader {
        bytes,
        cursor: 4,
        width: 0,
    };
    reader.width = reader.integer();
    let cases = reader.integer();
    let mut maximum = [0.0; 7];
    for _ in 0..cases {
        let n = reader.integer();
        let count = reader.integer();
        let bins = n / 2 + 1;
        let logs = reader.vector::<T>(bins);
        let coefficients = reader.vector::<T>(n);
        let inverse = reader.vector::<T>(n);
        let phase = reader.vector::<T>(n);
        let magnitudes = reader.vector::<T>(3 * bins);
        let matrix_coefficients = reader.vector::<T>(3 * count);
        let matrix_spectra = reader.vector::<T>(3 * bins);
        let mut plan = Plan::new(n).unwrap();
        let actual = plan.analyze(&logs, NyquistPolicy::Legacy).unwrap();
        measure(&actual, &coefficients, &mut maximum[0]);
        measure(
            &plan
                .reconstruct(&coefficients[..count], NyquistPolicy::Legacy)
                .unwrap(),
            &inverse,
            &mut maximum[1],
        );
        measure(
            &plan
                .reconstruct(&actual[..count], NyquistPolicy::Legacy)
                .unwrap(),
            &inverse,
            &mut maximum[2],
        );
        measure(
            &plan
                .minimum_phase_with(&logs, NyquistPolicy::Legacy, Lifter::Legacy)
                .unwrap(),
            &phase,
            &mut maximum[3],
        );
        let actual_matrix = plan
            .spectrogram_to_cepstra(&magnitudes, count, NyquistPolicy::Legacy)
            .unwrap();
        measure(&actual_matrix, &matrix_coefficients, &mut maximum[4]);
        measure(
            &plan
                .cepstra_to_spectrogram(&matrix_coefficients, 3, count, NyquistPolicy::Legacy)
                .unwrap(),
            &matrix_spectra,
            &mut maximum[5],
        );
        measure(
            &plan
                .cepstra_to_spectrogram(&actual_matrix, 3, count, NyquistPolicy::Legacy)
                .unwrap(),
            &matrix_spectra,
            &mut maximum[6],
        );
    }
    assert_eq!(reader.cursor, bytes.len());
    let bounds = if reader.width == 4 {
        [3e-8, 1.2e-7, 2e-7, 1.6e-7, 7e-9, 0.0, 2.5e-6]
    } else {
        [5e-17, 3e-16, 3e-16, 3e-16, 3e-17, 0.0, 0.0]
    };
    for (field, (&actual, &bound)) in maximum.iter().zip(&bounds).enumerate() {
        assert!(actual <= bound, "field {field}: {actual} exceeds {bound}");
    }
    println!(
        "{}-byte real cepstrum maximum errors {maximum:?}",
        reader.width
    );
}
#[test]
fn unchanged_c_reference() {
    reference::<f32>(include_bytes!("fixtures/real-cepstrum-float.bin"));
    reference::<f64>(include_bytes!("fixtures/real-cepstrum-double.bin"));
}
#[test]
fn analytic_round_trip_and_causal_phase() {
    let n = 16;
    let logs: Vec<f64> = (0..=n / 2)
        .map(|i| (2.0 * std::f64::consts::PI * i as f64 / n as f64).cos())
        .collect();
    let coefficients = real_cepstrum::from_log_spectrum(&logs).unwrap();
    for (i, &value) in coefficients.iter().enumerate() {
        let expected = if i == 1 || i == n - 1 { 0.5 } else { 0.0 };
        assert!((value - expected).abs() < 1e-15);
    }
    let reconstructed = real_cepstrum::to_log_spectrum(&coefficients, n).unwrap();
    for (&a, &b) in reconstructed.iter().zip(&logs) {
        assert!((a - b).abs() < 1e-15);
    }
    let phase = real_cepstrum::minimum_phase(&logs).unwrap();
    for (i, &value) in phase.iter().enumerate() {
        assert!((value + (2.0 * std::f64::consts::PI * i as f64 / n as f64).sin()).abs() < 1e-15);
    }
    // The source leaves the first negative-quefrency sample uncleared.
    let logs8 = [0.0_f64, 1.0, 0.0, 0.0, 0.0];
    let mut plan = Plan::new(8).unwrap();
    let cepstrum = plan.analyze(&logs8, NyquistPolicy::Preserve).unwrap();
    let causal = plan
        .minimum_phase_with(&logs8, NyquistPolicy::Preserve, Lifter::Causal)
        .unwrap();
    let legacy = plan
        .minimum_phase_with(&logs8, NyquistPolicy::Preserve, Lifter::Legacy)
        .unwrap();
    for i in 0..8 {
        let difference = cepstrum[5] * (-2.0 * std::f64::consts::PI * i as f64 * 5.0 / 8.0).sin();
        assert!((legacy[i] - causal[i] - difference).abs() < 1e-15);
    }
}
#[test]
fn shape_policy_silence_and_plan_reuse() {
    assert!(real_cepstrum::from_log_spectrum::<f64>(&[]).is_err());
    assert!(real_cepstrum::minimum_phase(&[1.0_f64]).is_err());
    assert!(Plan::<f64>::new(0).is_err());
    assert!(Plan::<f64>::new(3).is_err());
    let mut plan = Plan::<f64>::new(4).unwrap();
    assert_eq!(plan.len(), 4);
    assert_eq!(plan.bins(), 3);
    assert!(!plan.is_empty());
    assert!(plan.analyze(&[1.0; 2], NyquistPolicy::Preserve).is_err());
    assert!(
        plan.reconstruct(&[0.0; 5], NyquistPolicy::Preserve)
            .is_err()
    );
    assert!(
        plan.analyze(&[0.0, 0.0, f64::NAN], NyquistPolicy::Preserve)
            .is_err()
    );
    assert_eq!(
        plan.analyze(&[0.0, 0.0, f64::NAN], NyquistPolicy::Legacy)
            .unwrap(),
        vec![0.0; 4]
    );
    assert_eq!(
        plan.reconstruct(&[], NyquistPolicy::Preserve).unwrap(),
        vec![0.0; 4]
    );
    assert_eq!(
        plan.reconstruct(&[0.0, 0.0, 0.0, f64::NAN], NyquistPolicy::Preserve)
            .unwrap(),
        vec![0.0; 4]
    );
    let preserved = plan
        .analyze(&[0.0, 0.0, 4.0], NyquistPolicy::Preserve)
        .unwrap();
    assert_eq!(preserved, [1.0, -1.0, 1.0, -1.0]);
    let ignored = plan
        .spectrogram_to_cepstra(&[1.0, 1.0, f64::NAN], 4, NyquistPolicy::Legacy)
        .unwrap();
    assert_eq!(
        ignored,
        plan.spectrogram_to_cepstra(&[1.0; 3], 4, NyquistPolicy::Legacy)
            .unwrap()
    );
    assert!(
        plan.spectrogram_to_cepstra(&[1.0, 1.0, f64::NAN], 4, NyquistPolicy::Preserve)
            .is_err()
    );
    assert!(
        plan.spectrogram_to_cepstra(&[1.0; 2], 1, NyquistPolicy::Preserve)
            .is_err()
    );
    assert!(
        plan.spectrogram_to_cepstra(&[1.0; 3], 5, NyquistPolicy::Preserve)
            .is_err()
    );
    assert!(
        plan.spectrogram_to_cepstra(&[-1.0; 3], 1, NyquistPolicy::Preserve)
            .is_err()
    );
    assert!(
        plan.cepstra_to_spectrogram(&[1.0], 2, 1, NyquistPolicy::Preserve)
            .is_err()
    );
    assert!(
        plan.cepstra_to_spectrogram(&[], usize::MAX, 0, NyquistPolicy::Preserve)
            .is_err()
    );
    assert_eq!(
        plan.spectrogram_to_cepstra(&[], 2, NyquistPolicy::Preserve)
            .unwrap(),
        Vec::<f64>::new()
    );
    assert_eq!(
        plan.cepstra_to_spectrogram(&[], 0, 2, NyquistPolicy::Preserve)
            .unwrap(),
        Vec::<f64>::new()
    );
    let empty = plan
        .cepstra_to_spectrogram(&[], 2, 0, NyquistPolicy::Preserve)
        .unwrap();
    assert_eq!(empty.len(), 6);
    assert!(
        empty
            .iter()
            .all(|&value| value == ciglet_rs::approximation::exponential(0.0).unwrap() as f64)
    );
    assert!(
        plan.reconstruct(&[f64::MAX; 3], NyquistPolicy::Preserve)
            .is_err()
    );
    assert_eq!(
        plan.minimum_phase_with(&[1.0; 3], NyquistPolicy::Preserve, Lifter::Causal)
            .unwrap(),
        vec![0.0; 4]
    );
    let mut singleton = Plan::new(1).unwrap();
    assert_eq!(
        singleton
            .analyze(&[2.0_f32], NyquistPolicy::Legacy)
            .unwrap(),
        [2.0]
    );
    assert_eq!(
        singleton
            .minimum_phase_with(&[2.0_f32], NyquistPolicy::Legacy, Lifter::Legacy)
            .unwrap(),
        [0.0]
    );
}

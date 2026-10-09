use ciglet_rs::{frame::fetch_frame, statistics::*, window::Window};

#[test]
fn matches_independent_c_reference() {
    let x = [-2.0, 0.5, 4.0, 1.25, -0.75, 3.0];
    let y = [1.0, -2.0, 0.25, 4.0, 0.5, -1.0];
    let cases = [
        (
            "statistics",
            vec![
                sum(&x),
                sum_squares(&x),
                mean_with(&x, Policy::Legacy).unwrap(),
                variance_with(&x, Policy::Legacy).unwrap(),
                covariance_with(&x, &y, Policy::Legacy).unwrap(),
                correlation_with(&x, &y, Policy::Legacy).unwrap(),
                dot(&x, &y).unwrap(),
            ],
        ),
        ("hann", Window::Hann.generate(8)),
        ("hamming", Window::Hamming.generate(8)),
        ("blackman", Window::Blackman.generate(8)),
        ("blackman_harris", Window::BlackmanHarris.generate(8)),
        ("nuttall98", Window::Nuttall98.generate(8)),
        ("mlt_sine", Window::MltSine.generate(8)),
        ("frame_left", fetch_frame(&x, 0, 8).unwrap()),
        ("frame_right", fetch_frame(&x, 5, 7).unwrap()),
    ];
    let reference = include_str!("../../../tests/fixtures/dsp-reference.txt");
    for ((name, actual), line) in cases.iter().zip(reference.lines()) {
        let mut fields = line.split_whitespace();
        assert_eq!(fields.next(), Some(*name));
        let expected: Vec<f32> = fields.map(|field| field.parse().unwrap()).collect();
        assert_eq!(actual.len(), expected.len(), "{name}");
        for (index, (actual, expected)) in actual.iter().zip(&expected).enumerate() {
            assert!(
                (actual - expected).abs() <= 1.0e-7,
                "{name}[{index}]: {actual} != {expected}"
            );
        }
    }
    assert_eq!(cases.len(), reference.lines().count());
}

#[test]
fn selection_preserves_values_and_sort_indices() {
    let values = [3.0, 1.0, 3.0, -2.0];
    assert_eq!(median(&values).unwrap(), 2.0);
    assert_eq!(sort(&values), (vec![-2.0, 1.0, 3.0, 3.0], vec![3, 1, 0, 2]));
    assert_eq!(values, [3.0, 1.0, 3.0, -2.0]);
    assert_eq!(
        cross_correlation(&[1.0, 2.0], &[3.0, 4.0], 4).unwrap(),
        [11.0, 6.0, 0.0, 0.0]
    );
}

#[test]
fn invalid_and_degenerate_inputs_have_defined_behavior() {
    assert!(mean::<f32>(&[]).is_err());
    assert!(median::<f32>(&[]).is_err());
    assert!(dot(&[1.0], &[]).is_err());
    assert!(select_nth(&mut [1.0], 1).is_err());
    assert_eq!(Window::Hann.generate(1), [1.0]);
    assert!(Window::Hann.generate(0).is_empty());
    assert!(fetch_frame::<f32>(&[], isize::MIN, 4).is_err());
}

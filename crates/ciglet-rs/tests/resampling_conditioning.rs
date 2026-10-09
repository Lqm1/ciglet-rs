use ciglet_rs::{interpolation, resampling};
use resampling::{BoundaryPolicy, KernelPolicy};

#[test]
fn near_integer_queries_retain_the_impulse_limit() {
    for distance in [
        0.0,
        2.0f64.powi(-50),
        -2.0f64.powi(-50),
        1e-8,
        -1e-8,
        1e-4,
        -1e-4,
    ] {
        let query = 1.0 + distance;
        let actual = interpolation::sinc_uniform(0.0, 3.0, &[0.0, 0.75, 0.0], &[query]).unwrap()[0];
        let angle = std::f64::consts::PI * (query - 1.0);
        let expected = if angle == 0.0 {
            0.75
        } else {
            0.75 * (angle.sin() / angle) * ((angle / 4.0).sin() / (angle / 4.0))
        };
        assert!(
            (actual - expected).abs() <= 3e-16,
            "distance={distance}: {actual} / {expected}"
        );
    }
    let query = 1.0 + 2.0f64.powi(-50);
    let legacy = interpolation::sinc_uniform_with_kernel(
        0.0,
        3.0,
        &[0.0, 0.75, 0.0],
        &[query],
        BoundaryPolicy::IncludeFirst,
        KernelPolicy::Legacy,
    )
    .unwrap()[0];
    assert!(legacy > 1e16);
    let float =
        interpolation::sinc_uniform(0.0f32, 3.0, &[0.0, 0.75, 0.0], &[1.0 + f32::EPSILON]).unwrap();
    assert_eq!(float, [0.75]);
}

#[test]
fn fractional_rate_resampling_is_bounded_and_policies_are_independent() {
    for count in [2, 5, 32, 65] {
        let signal: Vec<_> = (0..count)
            .map(|index| (index % 7) as f64 * 0.125 - 0.375)
            .collect();
        for ratio in [0.9, 1.1, 1.5, 2.0] {
            let actual = resampling::resample(&signal, ratio).unwrap();
            assert!(
                actual
                    .iter()
                    .all(|value| value.is_finite() && value.abs() < 1.0)
            );
            assert_eq!(
                actual,
                resampling::resample_with_kernel(
                    &signal,
                    ratio,
                    BoundaryPolicy::IncludeFirst,
                    KernelPolicy::Stable,
                )
                .unwrap()
            );
            assert_eq!(
                resampling::resample_with(&signal, ratio, BoundaryPolicy::LegacySkipFirst).unwrap(),
                resampling::resample_with_kernel(
                    &signal,
                    ratio,
                    BoundaryPolicy::LegacySkipFirst,
                    KernelPolicy::Legacy
                )
                .unwrap()
            );
        }
    }
    for boundary in [
        BoundaryPolicy::IncludeFirst,
        BoundaryPolicy::LegacySkipFirst,
    ] {
        for kernel in [KernelPolicy::Stable, KernelPolicy::Legacy] {
            let actual = interpolation::sinc_uniform_with_kernel(
                0.0,
                2.0,
                &[0.75, 0.0],
                &[0.0],
                boundary,
                kernel,
            )
            .unwrap();
            assert_eq!(
                actual,
                [if boundary == BoundaryPolicy::IncludeFirst {
                    0.75
                } else {
                    0.0
                }]
            );
        }
    }
}

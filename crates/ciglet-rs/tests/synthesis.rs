use ciglet_rs::synthesis::{Sinusoid, sinusoid, sinusoids};
use rustfft::FftNum;

struct Reader<'a>(&'a [u8]);
impl Reader<'_> {
    fn take<const N: usize>(&mut self) -> [u8; N] {
        let (value, remaining) = self.0.split_at(N);
        self.0 = remaining;
        value.try_into().unwrap()
    }
    fn integer(&mut self) -> usize {
        u32::from_le_bytes(self.take()) as usize
    }
    fn scalar(&mut self, precision: usize) -> f64 {
        match precision {
            4 => f32::from_le_bytes(self.take()) as f64,
            8 => f64::from_le_bytes(self.take()),
            _ => panic!("invalid precision"),
        }
    }
}

fn compare_c<T: FftNum + Into<f64>>(fixture: &[u8], precision: usize, tolerance: f64) {
    let mut reader = Reader(fixture);
    assert_eq!(&reader.take::<4>(), b"SYN1");
    assert_eq!(reader.integer(), precision);
    assert_eq!(reader.integer(), 48);
    let mut maximum_error = 0.0f64;
    let mut count = 0;
    for _ in 0..48 {
        let single = reader.integer();
        let length = reader.integer();
        let rate = reader.integer() as u32;
        let tone_count = reader.integer();
        let mut tones = Vec::new();
        for _ in 0..tone_count {
            tones.push(Sinusoid {
                frequency_hz: T::from_f64(reader.scalar(precision)).unwrap(),
                amplitude: T::from_f64(reader.scalar(precision)).unwrap(),
                phase_radians: T::from_f64(reader.scalar(precision)).unwrap(),
            });
        }
        let scale = tones
            .iter()
            .map(|tone| tone.amplitude.into().abs())
            .sum::<f64>()
            .max(1.0);
        let samples = if single == 1 {
            assert_eq!(tone_count, 1);
            sinusoid(tones[0], rate, length).unwrap()
        } else {
            sinusoids(&tones, rate, length).unwrap()
        };
        for (index, actual) in samples.into_iter().enumerate() {
            let expected = reader.scalar(precision);
            let error = (actual.into() - expected).abs() / scale;
            maximum_error = maximum_error.max(error);
            assert!(
                error <= tolerance,
                "synthesis mismatch {single}/{length}/{rate}/{index}: normalized error {error}"
            );
            count += 1;
        }
    }
    assert!(reader.0.is_empty());
    eprintln!(
        "C synthesis {precision}-byte precision: {count} samples, maximum normalized error {maximum_error:e}"
    );
}

#[test]
fn float_synthesis_matches_c() {
    compare_c::<f32>(
        include_bytes!("../../../tests/fixtures/c-synthesis-f32.bin"),
        4,
        2e-7,
    );
}
#[test]
fn double_synthesis_matches_c() {
    compare_c::<f64>(
        include_bytes!("../../../tests/fixtures/c-synthesis-f64.bin"),
        8,
        2e-14,
    );
}

#[test]
fn amplitude_and_centered_phase_match_independent_formula() {
    for length in [1, 2, 5, 16, 127] {
        for amplitude in [0.0, 0.25, -0.5, 2.0] {
            let tone = Sinusoid {
                frequency_hz: 375.0f64,
                amplitude,
                phase_radians: 0.25,
            };
            let samples = sinusoid(tone, 16000, length).unwrap();
            for (index, &sample) in samples.iter().enumerate() {
                let angle =
                    std::f64::consts::TAU / 16000.0 * 375.0 * (index as f64 - (length / 2) as f64)
                        + 0.25;
                assert!((sample - amplitude * angle.cos()).abs() < 1e-12);
            }
        }
    }
    let tones = [
        Sinusoid {
            frequency_hz: 220.0f64,
            amplitude: 0.5,
            phase_radians: 0.0,
        },
        Sinusoid {
            frequency_hz: 440.0,
            amplitude: 0.25,
            phase_radians: 0.125,
        },
    ];
    let combined = sinusoids(&tones, 16000, 31).unwrap();
    let first = sinusoid(tones[0], 16000, 31).unwrap();
    let second = sinusoid(tones[1], 16000, 31).unwrap();
    for ((a, b), c) in combined.iter().zip(first).zip(second) {
        assert_eq!(*a, b + c);
    }
}

#[test]
fn empty_short_and_invalid_synthesis_is_defined() {
    let tone = Sinusoid {
        frequency_hz: 375.0f32,
        amplitude: 0.25,
        phase_radians: 0.125,
    };
    assert!(sinusoid(tone, 16000, 0).unwrap().is_empty());
    assert_eq!(
        sinusoid(tone, 16000, 1).unwrap(),
        vec![(0.125f64.cos() * 0.25) as f32]
    );
    assert_eq!(sinusoids::<f32>(&[], 16000, 5).unwrap(), vec![0.0; 5]);
    for rate in [0, u32::MAX] {
        assert!(sinusoid(tone, rate, 3).is_err());
    }
    assert!(sinusoid(tone, 16000, usize::MAX).is_err());
    for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert!(
            sinusoid(
                Sinusoid {
                    frequency_hz: invalid,
                    ..tone
                },
                16000,
                3
            )
            .is_err()
        );
        assert!(
            sinusoid(
                Sinusoid {
                    amplitude: invalid,
                    ..tone
                },
                16000,
                3
            )
            .is_err()
        );
        assert!(
            sinusoid(
                Sinusoid {
                    phase_radians: invalid,
                    ..tone
                },
                16000,
                3
            )
            .is_err()
        );
    }
    assert!(
        sinusoid(
            Sinusoid {
                frequency_hz: f32::MAX,
                ..tone
            },
            1,
            3
        )
        .is_err()
    );
}

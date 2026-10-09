use ciglet_rs::{statistics::Scalar, synthesis::Sinusoid};
use ciglet_rs_capi::*;
use std::ptr;
type Single<T> = unsafe extern "C" fn(T, T, T, u32, usize, *mut T, usize) -> u32;
type Multiple<T> =
    unsafe extern "C" fn(*const T, *const T, *const T, usize, u32, usize, *mut T, usize) -> u32;
struct Api<T> {
    single: Single<T>,
    multiple: Multiple<T>,
}
fn single<T: Scalar>(api: &Api<T>, tone: Sinusoid<T>, rate: u32, length: usize) -> Vec<T> {
    let mut output = vec![T::zero(); length];
    // SAFETY: Matching typed entry point and exclusive initialized output.
    unsafe {
        assert_eq!(
            (api.single)(
                tone.frequency_hz,
                tone.amplitude,
                tone.phase_radians,
                rate,
                length,
                output.as_mut_ptr(),
                length
            ),
            0
        );
    }
    output
}
fn multiple<T: Scalar>(api: &Api<T>, tones: &[Sinusoid<T>], rate: u32, length: usize) -> Vec<T> {
    let frequencies: Vec<_> = tones.iter().map(|v| v.frequency_hz).collect();
    let amplitudes: Vec<_> = tones.iter().map(|v| v.amplitude).collect();
    let phases: Vec<_> = tones.iter().map(|v| v.phase_radians).collect();
    let mut output = vec![T::zero(); length];
    // SAFETY: Matching typed entry point, stable initialized independent inputs
    // and exclusive allocated output. Empty Vec pointers have zero counts.
    unsafe {
        assert_eq!(
            (api.multiple)(
                frequencies.as_ptr(),
                amplitudes.as_ptr(),
                phases.as_ptr(),
                tones.len(),
                rate,
                length,
                output.as_mut_ptr(),
                length
            ),
            0
        );
    }
    output
}
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

fn compare_c<T: Scalar>(fixture: &[u8], precision: usize, tolerance: f64, api: &Api<T>) {
    let mut reader = Reader(fixture);
    assert_eq!(&reader.take::<4>(), b"SYN1");
    assert_eq!(reader.integer(), precision);
    assert_eq!(reader.integer(), 48);
    let mut maximum_error = 0.0f64;
    let mut count = 0;
    for _ in 0..48 {
        let single_flag = reader.integer();
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
        let samples = if single_flag == 1 {
            assert_eq!(tone_count, 1);
            single(api, tones[0], rate, length)
        } else {
            multiple(api, &tones, rate, length)
        };
        for (index, actual) in samples.into_iter().enumerate() {
            let expected = reader.scalar(precision);
            let error = (actual.into() - expected).abs() / scale;
            maximum_error = maximum_error.max(error);
            assert!(
                error <= tolerance,
                "synthesis mismatch {single_flag}/{length}/{rate}/{index}: normalized error {error}"
            );
            count += 1;
        }
    }
    assert!(reader.0.is_empty());
    eprintln!(
        "C synthesis {precision}-byte precision: {count} samples, maximum normalized error {maximum_error:e}"
    );
}

fn analytic<T: Scalar>(api: &Api<T>, tolerance: f64) {
    for length in [0, 1, 2, 5, 16] {
        for amplitude in [0., 0.25, -0.5, 2.] {
            let tone = Sinusoid {
                frequency_hz: T::from_f64(375.).unwrap(),
                amplitude: T::from_f64(amplitude).unwrap(),
                phase_radians: T::from_f64(0.25).unwrap(),
            };
            let values = single(api, tone, 16000, length);
            assert_eq!(values, multiple(api, &[tone], 16000, length));
            for (index, value) in values.iter().enumerate() {
                let angle =
                    std::f64::consts::TAU / 16000. * 375. * (index as f64 - (length / 2) as f64)
                        + 0.25;
                assert!(
                    ((*value).into() - amplitude * angle.cos()).abs() <= tolerance,
                    "{length}/{index}"
                );
            }
        }
    }
    let tones = [
        Sinusoid {
            frequency_hz: T::from_f64(-250.).unwrap(),
            amplitude: T::from_f64(-0.5).unwrap(),
            phase_radians: T::from_f64(-0.25).unwrap(),
        },
        Sinusoid {
            frequency_hz: T::from_f64(880.).unwrap(),
            amplitude: T::from_f64(0.25).unwrap(),
            phase_radians: T::from_f64(0.125).unwrap(),
        },
    ];
    let combined = multiple(api, &tones, 16000, 31);
    let a = single(api, tones[0], 16000, 31);
    let b = single(api, tones[1], 16000, 31);
    for ((value, a), b) in combined.into_iter().zip(a).zip(b) {
        assert_eq!(value, a + b);
    }
    assert_eq!(multiple(api, &[], 16000, 7), vec![T::zero(); 7]);
}
fn errors<T: Scalar>(api: &Api<T>) {
    let zero = T::zero();
    let one = T::one();
    let sentinel = T::from_f64(71.).unwrap();
    let nan = T::from_f64(f64::NAN).unwrap();
    let infinity = T::from_f64(f64::INFINITY).unwrap();
    let mut output = [sentinel; 3];
    // SAFETY: Exclusive allocated outputs and initialized inputs; intentionally
    // invalid counts/pointers are rejected without dereference or allocation.
    unsafe {
        for rate in [0, u32::MAX] {
            assert_eq!(
                (api.single)(zero, one, zero, rate, 3, output.as_mut_ptr(), 3),
                3
            );
            assert_eq!(
                (api.multiple)(&zero, &one, &zero, 1, rate, 3, output.as_mut_ptr(), 3),
                3
            );
        }
        for value in [nan, infinity] {
            for tone in [(value, one, zero), (zero, value, zero), (zero, one, value)] {
                assert_eq!(
                    (api.single)(tone.0, tone.1, tone.2, 16000, 3, output.as_mut_ptr(), 3),
                    3
                );
                assert_eq!(
                    (api.multiple)(
                        &tone.0,
                        &tone.1,
                        &tone.2,
                        1,
                        16000,
                        3,
                        output.as_mut_ptr(),
                        3
                    ),
                    3
                );
            }
        }
        assert_eq!(output, [sentinel; 3]);
        assert_eq!(
            (api.single)(zero, one, zero, 16000, 3, output.as_mut_ptr(), 2),
            2
        );
        assert_eq!(
            (api.single)(
                zero,
                one,
                zero,
                16000,
                usize::MAX,
                output.as_mut_ptr(),
                usize::MAX
            ),
            2
        );
        assert_eq!(
            (api.single)(zero, one, zero, 16000, 1, ptr::null_mut(), 1),
            1
        );
        assert_eq!(
            (api.multiple)(
                ptr::null(),
                &one,
                &zero,
                1,
                16000,
                3,
                output.as_mut_ptr(),
                3
            ),
            1
        );
        assert_eq!(
            (api.multiple)(
                &zero,
                ptr::null(),
                &zero,
                1,
                16000,
                3,
                output.as_mut_ptr(),
                3
            ),
            1
        );
        assert_eq!(
            (api.multiple)(
                &zero,
                &one,
                ptr::null(),
                1,
                16000,
                3,
                output.as_mut_ptr(),
                3
            ),
            1
        );
        assert_eq!(
            (api.multiple)(
                &zero,
                &one,
                &zero,
                usize::MAX,
                16000,
                3,
                output.as_mut_ptr(),
                3
            ),
            2
        );
        assert_eq!(output, [sentinel; 3]);
        assert_eq!(
            (api.single)(zero, one, zero, 16000, 0, ptr::null_mut(), 0),
            0
        );
        assert_eq!(
            (api.single)(nan, one, zero, 16000, 0, ptr::null_mut(), 0),
            3
        );
        assert_eq!(
            (api.multiple)(
                ptr::null(),
                ptr::null(),
                ptr::null(),
                0,
                16000,
                0,
                ptr::null_mut(),
                0
            ),
            0
        );
        assert_eq!(
            (api.multiple)(
                ptr::null(),
                ptr::null(),
                ptr::null(),
                0,
                0,
                0,
                ptr::null_mut(),
                0
            ),
            3
        );
        let mut shared = [zero, zero, zero];
        assert_eq!(
            (api.multiple)(
                shared.as_ptr(),
                &one,
                &zero,
                1,
                16000,
                3,
                shared.as_mut_ptr(),
                3
            ),
            0
        );
        assert_eq!(shared, [one; 3]);
        let mut silence = [sentinel; 3];
        assert_eq!(
            (api.multiple)(
                ptr::null(),
                ptr::null(),
                ptr::null(),
                0,
                16000,
                3,
                silence.as_mut_ptr(),
                3
            ),
            0
        );
        assert_eq!(silence, [zero; 3]);
        let mut words = [0x47474747u32; 8];
        assert_eq!(
            (api.single)(
                zero,
                one,
                zero,
                16000,
                1,
                words.as_mut_ptr().cast::<u8>().add(1).cast(),
                1
            ),
            1
        );
        assert_eq!(words, [0x47474747; 8]);
    }
}
#[test]
fn float_original_reference() {
    compare_c::<f32>(
        include_bytes!("../../../tests/fixtures/c-synthesis-f32.bin"),
        4,
        2e-7,
        &Api {
            single: ciglet_rs_sinusoid_f32,
            multiple: ciglet_rs_sinusoids_f32,
        },
    );
}
#[test]
fn float_independent_amplitude_phase_and_sum() {
    analytic(
        &Api {
            single: ciglet_rs_sinusoid_f32,
            multiple: ciglet_rs_sinusoids_f32,
        },
        2e-6,
    );
}
#[test]
fn float_atomic_errors_empty_and_overlap() {
    errors(&Api {
        single: ciglet_rs_sinusoid_f32,
        multiple: ciglet_rs_sinusoids_f32,
    });
}
#[test]
fn double_original_reference() {
    compare_c::<f64>(
        include_bytes!("../../../tests/fixtures/c-synthesis-f64.bin"),
        8,
        2e-14,
        &Api {
            single: ciglet_rs_sinusoid_f64,
            multiple: ciglet_rs_sinusoids_f64,
        },
    );
}
#[test]
fn double_independent_amplitude_phase_and_sum() {
    analytic(
        &Api {
            single: ciglet_rs_sinusoid_f64,
            multiple: ciglet_rs_sinusoids_f64,
        },
        1e-12,
    );
}
#[test]
fn double_atomic_errors_empty_and_overlap() {
    errors(&Api {
        single: ciglet_rs_sinusoid_f64,
        multiple: ciglet_rs_sinusoids_f64,
    });
}

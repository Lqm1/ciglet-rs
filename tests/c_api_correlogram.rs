#![cfg(feature = "c-api")]
use ciglet_rs::{Error, c_api::*};
use std::ptr;
type Compute<T> = unsafe extern "C" fn(
    *const T,
    usize,
    *const i32,
    *const usize,
    usize,
    usize,
    i32,
    *mut T,
    usize,
) -> u32;
type Selected<T> = unsafe extern "C" fn(
    *const T,
    usize,
    *const i32,
    *const usize,
    usize,
    usize,
    i32,
    u32,
    *mut T,
    usize,
) -> u32;
type Frequency<T> =
    unsafe extern "C" fn(*const T, usize, usize, usize, i32, *const T, usize, *mut T, usize) -> u32;
struct Api<T> {
    compute: Compute<T>,
    selected: Selected<T>,
    frequency: Frequency<T>,
}
trait Ffi: Scalar {
    fn api() -> Api<Self>;
}
impl Ffi for f32 {
    fn api() -> Api<Self> {
        Api {
            compute: ciglet_rs_correlogram_compute_f32,
            selected: ciglet_rs_correlogram_compute_with_f32,
            frequency: ciglet_rs_correlogram_to_frequency_f32,
        }
    }
}
impl Ffi for f64 {
    fn api() -> Api<Self> {
        Api {
            compute: ciglet_rs_correlogram_compute_f64,
            selected: ciglet_rs_correlogram_compute_with_f64,
            frequency: ciglet_rs_correlogram_to_frequency_f64,
        }
    }
}
mod correlogram {
    use super::*;
    pub fn compute<T: Ffi>(
        samples: &[T],
        frames: &[Frame],
        periods: usize,
        method: Method,
    ) -> Result<Array2<T>, Error> {
        invoke(samples, frames, periods, method, None)
    }
    pub fn compute_with<T: Ffi>(
        samples: &[T],
        frames: &[Frame],
        periods: usize,
        method: Method,
        policy: Policy,
    ) -> Result<Array2<T>, Error> {
        invoke(samples, frames, periods, method, Some(policy))
    }
    fn invoke<T: Ffi>(
        samples: &[T],
        frames: &[Frame],
        periods: usize,
        method: Method,
        policy: Option<Policy>,
    ) -> Result<Array2<T>, Error> {
        let count = frames
            .len()
            .checked_mul(periods)
            .ok_or(Error("test dimensions overflow"))?;
        let centers: Vec<_> = frames.iter().map(|f| f.center).collect();
        let lengths: Vec<_> = frames.iter().map(|f| f.length).collect();
        let mut output = vec![T::zero(); count];
        let api = T::api();
        // SAFETY: Matching typed API with initialized independent inputs and
        // exclusively writable allocated output. Empty pointers have zero counts.
        let status = unsafe {
            match policy {
                None => (api.compute)(
                    samples.as_ptr(),
                    samples.len(),
                    centers.as_ptr(),
                    lengths.as_ptr(),
                    frames.len(),
                    periods,
                    method as i32,
                    output.as_mut_ptr(),
                    count,
                ),
                Some(policy) => (api.selected)(
                    samples.as_ptr(),
                    samples.len(),
                    centers.as_ptr(),
                    lengths.as_ptr(),
                    frames.len(),
                    periods,
                    method as i32,
                    match policy {
                        Policy::Stable => 0,
                        Policy::Legacy => 1,
                    },
                    output.as_mut_ptr(),
                    count,
                ),
            }
        };
        if status != 0 {
            return Err(Error("C correlogram failed"));
        }
        Array2::from_row_major(frames.len(), periods, output)
    }
    pub fn to_frequency<T: Ffi>(
        values: &Array2<T>,
        rate: i32,
        frequencies: &[T],
    ) -> Result<Array2<T>, Error> {
        let count = values
            .rows()
            .checked_mul(frequencies.len())
            .ok_or(Error("test dimensions overflow"))?;
        let mut output = vec![T::zero(); count];
        // SAFETY: Matching typed API with initialized independent storage.
        let status = unsafe {
            (T::api().frequency)(
                values.as_slice().as_ptr(),
                values.rows(),
                values.columns(),
                values.len(),
                rate,
                frequencies.as_ptr(),
                frequencies.len(),
                output.as_mut_ptr(),
                count,
            )
        };
        if status != 0 {
            return Err(Error("C frequency mapping failed"));
        }
        Array2::from_row_major(values.rows(), frequencies.len(), output)
    }
}
use ciglet_rs::{
    array::Array2,
    correlogram::{Frame, Method, Policy},
    statistics::Scalar,
};
#[test]
fn unchanged_c_reference() {
    reference::<f32>(include_bytes!("fixtures/correlogram-float.bin"), 4);
    reference::<f64>(include_bytes!("fixtures/correlogram-double.bin"), 8);
}
fn reference<T: Ffi>(bytes: &[u8], width: usize) {
    assert_eq!(&bytes[..4], b"CRG1");
    let mut cursor = 4;
    fn integer(bytes: &[u8], cursor: &mut usize) -> u32 {
        let v = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
        *cursor += 4;
        v
    }
    fn values<T: Scalar>(bytes: &[u8], cursor: &mut usize, width: usize, n: usize) -> Vec<T> {
        (0..n)
            .map(|_| {
                let v = if width == 4 {
                    f32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap()) as f64
                } else {
                    f64::from_le_bytes(bytes[*cursor..*cursor + 8].try_into().unwrap())
                };
                *cursor += width;
                T::from_f64(v).unwrap()
            })
            .collect()
    }
    assert_eq!(integer(bytes, &mut cursor) as usize, width);
    assert_eq!(integer(bytes, &mut cursor), 32);
    let mut maximum_error = 0.0_f64;
    for case in 0..32 {
        let nx = integer(bytes, &mut cursor) as usize;
        let count = integer(bytes, &mut cursor) as usize;
        let periods = integer(bytes, &mut cursor) as usize;
        let method = [
            Method::Autocorrelation,
            Method::AbsoluteDifference,
            Method::SquaredDifference,
            Method::Yin,
        ][integer(bytes, &mut cursor) as usize];
        let samples = values::<T>(bytes, &mut cursor, width, nx);
        let centers: Vec<_> = (0..count)
            .map(|_| integer(bytes, &mut cursor) as i32)
            .collect();
        let frames: Vec<_> = centers
            .into_iter()
            .map(|center| Frame {
                center,
                length: integer(bytes, &mut cursor) as usize,
            })
            .collect();
        let expected = values::<T>(bytes, &mut cursor, width, count * periods);
        let actual =
            correlogram::compute_with(&samples, &frames, periods, method, Policy::Legacy).unwrap();
        let nf = integer(bytes, &mut cursor) as usize;
        let frequencies = values::<T>(bytes, &mut cursor, width, nf);
        let mapped_expected = values::<T>(bytes, &mut cursor, width, count * nf);
        let mapped = correlogram::to_frequency(&actual, 16000, &frequencies).unwrap();
        for (index, (&actual, &expected)) in actual
            .as_slice()
            .iter()
            .chain(mapped.as_slice())
            .zip(expected.iter().chain(&mapped_expected))
            .enumerate()
        {
            let error = (actual.into() - expected.into()).abs() / (1.0 + expected.into().abs());
            maximum_error = maximum_error.max(error);
            if width == 4 {
                assert_eq!(
                    actual.into().to_bits(),
                    expected.into().to_bits(),
                    "case={case} method={method:?} index={index} actual={} expected={}",
                    actual.into(),
                    expected.into()
                );
            } else {
                assert!(error <= 3e-15, "{} != {}", actual.into(), expected.into());
            }
        }
    }
    assert_eq!(cursor, bytes.len());
    eprintln!("width={width}, maximum normalized correlogram error={maximum_error:e}");
}
#[test]
fn analytic_methods_boundaries_and_dc_conditioning() {
    let frame = [Frame {
        center: 2,
        length: 4,
    }];
    let ramp = [0.0_f64, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let absolute = correlogram::compute(&ramp, &frame, 5, Method::AbsoluteDifference).unwrap();
    assert_eq!(absolute.as_slice(), &[0.0, 4.0, 8.0, 12.0, 0.0]);
    let squared = correlogram::compute(&ramp, &frame, 5, Method::SquaredDifference).unwrap();
    assert_eq!(squared.as_slice(), &[0.0, 4.0, 16.0, 36.0, 0.0]);
    let yin = correlogram::compute(&ramp, &frame, 5, Method::Yin).unwrap();
    assert_eq!(yin.as_slice(), &[1.0, 1.0, 1.6, 108.0 / 56.0, 1.0]);
    let acf = correlogram::compute(&ramp, &frame, 5, Method::Autocorrelation).unwrap();
    for &value in &acf.as_slice()[..4] {
        assert!((value - 1.0).abs() < 3e-16);
    }
    let offset: Vec<_> = ramp.iter().map(|&x| 1e12 + x).collect();
    let legacy =
        correlogram::compute_with(&offset, &frame, 3, Method::Autocorrelation, Policy::Legacy)
            .unwrap();
    assert_eq!(*legacy.get(0, 0).unwrap(), 0.0);
    let offset = correlogram::compute(&offset, &frame, 3, Method::Autocorrelation).unwrap();
    for &value in offset.as_slice() {
        assert!((value - 1.0).abs() < 1e-7);
    }
    let alternating = [1.0_f64, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0];
    assert_eq!(
        *correlogram::compute(&alternating, &frame, 3, Method::Autocorrelation)
            .unwrap()
            .get(0, 1)
            .unwrap(),
        0.0
    );
    let huge = alternating.map(|x| x * f64::MAX);
    assert_eq!(
        *correlogram::compute(&huge, &frame, 3, Method::Autocorrelation)
            .unwrap()
            .get(0, 0)
            .unwrap(),
        1.0
    );
    let silence = correlogram::compute(&[0.0_f32; 8], &frame, 3, Method::Yin).unwrap();
    assert_eq!(silence.as_slice(), &[1.0, 1.0, 1.0]);
}
#[test]
fn frequency_mapping_and_invalid_inputs() {
    let periods = Array2::from_row_major(
        2,
        4,
        vec![0.0_f64, 10.0, 20.0, 30.0, 100.0, 110.0, 120.0, 130.0],
    )
    .unwrap();
    let mapped =
        correlogram::to_frequency(&periods, 100, &[100.0, 50.0, 40.0, 0.0, -100.0, -0.0]).unwrap();
    assert_eq!(
        mapped.as_slice(),
        &[
            10.0, 20.0, 25.0, 30.0, 0.0, 0.0, 110.0, 120.0, 125.0, 130.0, 100.0, 100.0
        ]
    );
    let single = Array2::from_row_major(1, 1, vec![3.0_f32]).unwrap();
    assert_eq!(
        correlogram::to_frequency(&single, 100, &[0.0, 1.0])
            .unwrap()
            .as_slice(),
        &[3.0, 3.0]
    );
    assert!(correlogram::to_frequency(&periods, 0, &[1.0]).is_err());
    assert!(correlogram::to_frequency(&periods, 100, &[f64::NAN]).is_err());
    assert!(
        correlogram::compute(
            &[1.0_f32],
            &[Frame {
                center: 0,
                length: 0
            }],
            3,
            Method::Yin
        )
        .is_err()
    );
    assert!(correlogram::compute(&[1.0_f32], &[], 0, Method::Yin).is_err());
    assert!(
        correlogram::compute::<f32>(
            &[],
            &[Frame {
                center: 0,
                length: 1
            }],
            3,
            Method::Yin
        )
        .is_err()
    );
    assert!(correlogram::compute(&[f64::NAN], &[], 3, Method::Yin).is_err());
    assert_eq!(
        correlogram::compute::<f32>(&[], &[], 3, Method::Yin)
            .unwrap()
            .rows(),
        0
    );
}

fn atomic<T: Ffi>() {
    let api = T::api();
    let zero = T::zero();
    let one = T::one();
    let sentinel = T::from_f64(71.).unwrap();
    let nan = T::from_f64(f64::NAN).unwrap();
    let samples = [zero, one, zero, one, zero, one, zero, one];
    let center = 2;
    let length = 4usize;
    let mut output = [sentinel; 5];
    // SAFETY: Initialized valid buffers; intentional invalid selectors/counts/null
    // inputs are rejected before writing. Shared data is supported through staging.
    unsafe {
        assert_eq!(
            (api.compute)(
                samples.as_ptr(),
                8,
                &center,
                &length,
                1,
                5,
                4,
                output.as_mut_ptr(),
                5
            ),
            3
        );
        assert_eq!(
            (api.selected)(
                samples.as_ptr(),
                8,
                &center,
                &length,
                1,
                5,
                0,
                2,
                output.as_mut_ptr(),
                5
            ),
            3
        );
        assert_eq!(
            (api.compute)(
                samples.as_ptr(),
                8,
                &center,
                &length,
                1,
                5,
                0,
                output.as_mut_ptr(),
                4
            ),
            2
        );
        assert_eq!(
            (api.compute)(
                samples.as_ptr(),
                8,
                ptr::null(),
                &length,
                1,
                5,
                0,
                output.as_mut_ptr(),
                5
            ),
            1
        );
        assert_eq!(
            (api.compute)(
                samples.as_ptr(),
                8,
                &center,
                ptr::null(),
                1,
                5,
                0,
                output.as_mut_ptr(),
                5
            ),
            1
        );
        assert_eq!(
            (api.compute)(
                samples.as_ptr(),
                8,
                &center,
                &length,
                usize::MAX,
                2,
                0,
                output.as_mut_ptr(),
                5
            ),
            2
        );
        assert_eq!(
            (api.frequency)(
                samples.as_ptr(),
                1,
                4,
                4,
                100,
                &nan,
                1,
                output.as_mut_ptr(),
                1
            ),
            3
        );
        assert_eq!(
            (api.frequency)(
                samples.as_ptr(),
                1,
                4,
                3,
                100,
                &one,
                1,
                output.as_mut_ptr(),
                1
            ),
            2
        );
        assert_eq!(
            (api.frequency)(
                samples.as_ptr(),
                1,
                4,
                4,
                0,
                &one,
                1,
                output.as_mut_ptr(),
                1
            ),
            3
        );
        assert_eq!(output, [sentinel; 5]);
        assert_eq!(
            (api.compute)(
                ptr::null(),
                0,
                ptr::null(),
                ptr::null(),
                0,
                5,
                0,
                ptr::null_mut(),
                0
            ),
            0
        );
        assert_eq!(
            (api.compute)(
                ptr::null(),
                0,
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
        assert_eq!(
            (api.frequency)(ptr::null(), 0, 4, 0, 100, &one, 1, ptr::null_mut(), 0),
            0
        );
        let maximum = T::from_f64(if std::mem::size_of::<T>() == 4 {
            f32::MAX as f64
        } else {
            f64::MAX
        })
        .unwrap();
        let late_samples = [zero, one, zero, one, maximum, -maximum, zero, zero];
        let centers = [1, 5];
        let lengths = [2usize, 2];
        assert_eq!(
            (api.compute)(
                late_samples.as_ptr(),
                8,
                centers.as_ptr(),
                lengths.as_ptr(),
                2,
                2,
                2,
                output.as_mut_ptr(),
                4
            ),
            3
        );
        assert_eq!(output, [sentinel; 5]);
        let mut words = [0x47474747u32; 16];
        let invalid = words.as_mut_ptr().cast::<u8>().add(1);
        assert_eq!(
            (api.compute)(
                samples.as_ptr(),
                8,
                invalid.cast(),
                &length,
                1,
                5,
                0,
                output.as_mut_ptr(),
                5
            ),
            1
        );
        assert_eq!(
            (api.compute)(
                samples.as_ptr(),
                8,
                &center,
                invalid.cast(),
                1,
                5,
                0,
                output.as_mut_ptr(),
                5
            ),
            1
        );
        assert_eq!(
            (api.compute)(
                samples.as_ptr(),
                8,
                &center,
                &length,
                1,
                5,
                0,
                invalid.cast(),
                5
            ),
            1
        );
        assert_eq!(
            (api.frequency)(samples.as_ptr(), 1, 4, 4, 100, &one, 1, invalid.cast(), 1),
            1
        );
        assert_eq!(words, [0x47474747; 16]);
        assert_eq!(output, [sentinel; 5]);
        let mut shared = samples;
        assert_eq!(
            (api.compute)(
                shared.as_ptr(),
                8,
                &center,
                &length,
                1,
                5,
                1,
                shared.as_mut_ptr(),
                5
            ),
            0
        );
        assert_eq!(
            &shared[..5],
            &[
                zero,
                T::from_f64(4.).unwrap(),
                zero,
                T::from_f64(4.).unwrap(),
                zero
            ]
        );
        let mut row = [
            zero,
            one,
            T::from_f64(2.).unwrap(),
            T::from_f64(3.).unwrap(),
        ];
        let query = [T::from_f64(100.).unwrap(), T::from_f64(50.).unwrap()];
        assert_eq!(
            (api.frequency)(
                row.as_ptr(),
                1,
                4,
                4,
                100,
                query.as_ptr(),
                2,
                row.as_mut_ptr(),
                2
            ),
            0
        );
        assert_eq!(&row[..2], &[one, T::from_f64(2.).unwrap()]);
    }
}
#[test]
fn float_atomic_errors_and_overlap() {
    atomic::<f32>();
}
#[test]
fn double_atomic_errors_and_overlap() {
    atomic::<f64>();
}
#[test]
fn default_matches_explicit_stable_all_methods() {
    assert_eq!(
        CIGLET_RS_CORRELOGRAM_AUTOCORRELATION,
        Method::Autocorrelation as i32
    );
    assert_eq!(
        CIGLET_RS_CORRELOGRAM_ABSOLUTE_DIFFERENCE,
        Method::AbsoluteDifference as i32
    );
    assert_eq!(
        CIGLET_RS_CORRELOGRAM_SQUARED_DIFFERENCE,
        Method::SquaredDifference as i32
    );
    assert_eq!(CIGLET_RS_CORRELOGRAM_YIN, Method::Yin as i32);

    fn compare<T: Ffi>() {
        let samples: Vec<_> = (0..8).map(|i| T::from_f64(i as f64).unwrap()).collect();
        let frames = [Frame {
            center: 2,
            length: 4,
        }];
        for method in [
            Method::Autocorrelation,
            Method::AbsoluteDifference,
            Method::SquaredDifference,
            Method::Yin,
        ] {
            assert_eq!(
                correlogram::compute(&samples, &frames, 5, method).unwrap(),
                correlogram::compute_with(&samples, &frames, 5, method, Policy::Stable).unwrap()
            );
        }
    }
    compare::<f32>();
    compare::<f64>();
}

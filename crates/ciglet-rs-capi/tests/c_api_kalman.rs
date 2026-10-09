use ciglet_rs::{kalman::Estimate, statistics::Scalar};
use ciglet_rs_capi::*;
use rustfft::FftNum;
use std::ptr;
type Filter<T> =
    unsafe extern "C" fn(*const T, *const T, *const T, usize, *mut T, *mut T, usize, *mut T) -> u32;
type Initial<T> = unsafe extern "C" fn(
    *const T,
    *const T,
    *const T,
    usize,
    T,
    *mut T,
    *mut T,
    usize,
    *mut T,
) -> u32;
type Smooth<T> = unsafe extern "C" fn(*const T, *const T, *const T, usize, *mut T, usize) -> u32;
struct Api<T> {
    filter: Filter<T>,
    initial: Initial<T>,
    smooth: Smooth<T>,
}
fn estimate<T: Scalar>(
    api: &Api<T>,
    values: &[T],
    process: &[T],
    noise: &[T],
    initial: Option<T>,
) -> Estimate<T> {
    assert_eq!(values.len(), process.len());
    assert_eq!(values.len(), noise.len());
    let count = values.len();
    let mut means = vec![T::zero(); count];
    let mut variances = vec![T::zero(); count];
    let mut likelihood = T::zero();
    // SAFETY: Precision-matched API, initialized equal-length readable inputs and
    // three distinct exclusively writable outputs with valid lifetimes.
    unsafe {
        assert_eq!(
            match initial {
                Some(value) => (api.initial)(
                    values.as_ptr(),
                    process.as_ptr(),
                    noise.as_ptr(),
                    count,
                    value,
                    means.as_mut_ptr(),
                    variances.as_mut_ptr(),
                    count,
                    &mut likelihood
                ),
                None => (api.filter)(
                    values.as_ptr(),
                    process.as_ptr(),
                    noise.as_ptr(),
                    count,
                    means.as_mut_ptr(),
                    variances.as_mut_ptr(),
                    count,
                    &mut likelihood
                ),
            },
            0
        );
    }
    Estimate {
        means,
        variances,
        log_likelihood: likelihood,
    }
}
fn smoothing<T: Scalar>(api: &Api<T>, means: &[T], posterior: &[T], process: &[T]) -> Vec<T> {
    assert_eq!(means.len(), posterior.len());
    assert_eq!(means.len(), process.len());
    let mut output = vec![T::zero(); means.len()];
    // SAFETY: Precision-matched entry point and initialized independent allocations.
    unsafe {
        assert_eq!(
            (api.smooth)(
                means.as_ptr(),
                posterior.as_ptr(),
                process.as_ptr(),
                means.len(),
                output.as_mut_ptr(),
                means.len()
            ),
            0
        );
    }
    output
}
struct Reader<'a>(&'a [u8]);
impl Reader<'_> {
    fn bytes(&mut self, count: usize) -> &[u8] {
        let (head, tail) = self.0.split_at(count);
        self.0 = tail;
        head
    }
    fn integer(&mut self) -> usize {
        u32::from_le_bytes(self.bytes(4).try_into().unwrap()) as usize
    }
    fn scalar<T: FftNum>(&mut self, width: usize) -> T {
        T::from_f64(if width == 4 {
            f32::from_le_bytes(self.bytes(4).try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(self.bytes(8).try_into().unwrap())
        })
        .unwrap()
    }
    fn values<T: FftNum>(&mut self, count: usize, width: usize) -> Vec<T> {
        (0..count).map(|_| self.scalar(width)).collect()
    }
}

fn compare<T: Scalar>(bytes: &[u8], width: usize, api: &Api<T>) {
    let mut reader = Reader(bytes);
    assert_eq!(reader.bytes(4), b"KLM1");
    assert_eq!(reader.integer(), width);
    assert_eq!(reader.integer(), 108);
    for case in 0..108 {
        let count = reader.integer();
        let initial = reader.scalar::<T>(width);
        let measurements = reader.values::<T>(count, width);
        let process = reader.values::<T>(count, width);
        let noise = reader.values::<T>(count, width);
        let means = reader.values::<T>(count, width);
        let variances = reader.values::<T>(count, width);
        let smoothed = reader.values::<T>(count, width);
        let likelihood = reader.scalar::<T>(width);
        let actual = estimate(api, &measurements, &process, &noise, Some(initial));
        assert_eq!(actual.means, means, "means case={case}");
        assert_eq!(actual.variances, variances, "variance case={case}");
        assert_eq!(actual.log_likelihood, likelihood, "likelihood case={case}");
        assert_eq!(
            smoothing(api, &actual.means, &actual.variances, &process),
            smoothed,
            "smooth case={case}"
        );
        if initial == measurements[0] {
            assert_eq!(estimate(api, &measurements, &process, &noise, None), actual);
        }
    }
    assert!(reader.0.is_empty());
}

fn analytic<T: Scalar>(api: &Api<T>, bound: f64) {
    let v = |x| T::from_f64(x).unwrap();
    let estimate = estimate(
        api,
        &[v(1.), v(0.)],
        &[v(1.), v(1.)],
        &[v(1.), v(1.)],
        Some(v(0.)),
    );
    for (actual, expected) in estimate.means.iter().zip([0.5, 0.2]) {
        assert!(((*actual).into() - expected).abs() < bound);
    }
    for (actual, expected) in estimate.variances.iter().zip([0.5, 0.6]) {
        assert!(((*actual).into() - expected).abs() < bound);
    }
    let smooth = smoothing(api, &estimate.means, &estimate.variances, &[v(1.), v(1.)]);
    assert!((smooth[0].into() - 0.4).abs() < bound);
    assert_eq!(smooth[1], estimate.means[1]);
    let fixed = fixed_estimate(api);
    assert_eq!(
        smoothing(api, &fixed.means, &fixed.variances, &[v(0.), v(0.)]),
        fixed.means
    );
}
fn fixed_estimate<T: Scalar>(api: &Api<T>) -> Estimate<T> {
    let v = |x| T::from_f64(x).unwrap();
    let fixed = estimate(
        api,
        &[v(1.), v(2.)],
        &[v(0.), v(0.)],
        &[v(1.), v(1.)],
        Some(v(0.25)),
    );
    assert_eq!(fixed.means, [v(0.25); 2]);
    assert_eq!(fixed.variances, [v(0.); 2]);
    fixed
}
fn errors<T: Scalar>(api: &Api<T>) {
    let zero = T::zero();
    let one = T::one();
    let sentinel = T::from_f64(71.).unwrap();
    let nan = T::from_f64(f64::NAN).unwrap();
    let negative = T::from_f64(-1.).unwrap();
    let mut means = [sentinel; 2];
    let mut variances = [sentinel; 2];
    let mut likelihood = sentinel;
    // SAFETY: Initialized valid caller allocations; invalid null/count/overlap
    // cases are intentionally rejected before writing. Output buffers are distinct
    // except the specific rejected overlap cases. Inputs remain stable.
    unsafe {
        for (values, process, noise) in [
            (nan, one, one),
            (one, negative, one),
            (one, one, negative),
            (one, zero, zero),
        ] {
            assert_eq!(
                (api.filter)(
                    &values,
                    &process,
                    &noise,
                    1,
                    means.as_mut_ptr(),
                    variances.as_mut_ptr(),
                    1,
                    &mut likelihood
                ),
                3
            );
        }
        assert_eq!(
            (api.initial)(
                &one,
                &one,
                &one,
                1,
                nan,
                means.as_mut_ptr(),
                variances.as_mut_ptr(),
                1,
                &mut likelihood
            ),
            3
        );
        assert_eq!(
            (api.smooth)(&one, &negative, &one, 1, means.as_mut_ptr(), 1),
            3
        );
        assert_eq!(
            (api.filter)(
                &one,
                &one,
                &one,
                1,
                means.as_mut_ptr(),
                variances.as_mut_ptr(),
                0,
                &mut likelihood
            ),
            2
        );
        assert_eq!(
            (api.filter)(
                ptr::null(),
                &one,
                &one,
                1,
                means.as_mut_ptr(),
                variances.as_mut_ptr(),
                1,
                &mut likelihood
            ),
            1
        );
        assert_eq!(
            (api.filter)(
                &one,
                &one,
                &one,
                1,
                means.as_mut_ptr(),
                variances.as_mut_ptr(),
                1,
                ptr::null_mut()
            ),
            1
        );
        assert_eq!(
            (api.filter)(
                &one,
                &one,
                &one,
                1,
                means.as_mut_ptr(),
                means.as_mut_ptr(),
                1,
                &mut likelihood
            ),
            2
        );
        assert_eq!(
            (api.filter)(
                &one,
                &one,
                &one,
                1,
                means.as_mut_ptr(),
                variances.as_mut_ptr(),
                1,
                means.as_mut_ptr()
            ),
            2
        );
        assert_eq!(
            (api.filter)(
                &one,
                &one,
                &one,
                1,
                means.as_mut_ptr(),
                variances.as_mut_ptr(),
                1,
                variances.as_mut_ptr()
            ),
            2
        );
        assert_eq!(
            (api.filter)(
                &one,
                &one,
                &one,
                usize::MAX,
                means.as_mut_ptr(),
                variances.as_mut_ptr(),
                usize::MAX,
                &mut likelihood
            ),
            2
        );
        assert_eq!(means, [sentinel; 2]);
        assert_eq!(variances, [sentinel; 2]);
        assert_eq!(likelihood, sentinel);
        assert_eq!(
            (api.filter)(
                ptr::null(),
                ptr::null(),
                ptr::null(),
                0,
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                &mut likelihood
            ),
            0
        );
        assert_eq!(likelihood, zero);
        assert_eq!(
            (api.initial)(
                ptr::null(),
                ptr::null(),
                ptr::null(),
                0,
                nan,
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                &mut likelihood
            ),
            3
        );
        assert_eq!(
            (api.smooth)(ptr::null(), ptr::null(), ptr::null(), 0, ptr::null_mut(), 0),
            0
        );
        let mut shared = [one, zero];
        let input = shared.as_ptr();
        assert_eq!(
            (api.initial)(
                input,
                &one,
                &one,
                1,
                zero,
                shared.as_mut_ptr(),
                variances.as_mut_ptr(),
                1,
                &mut likelihood
            ),
            0
        );
        assert_eq!(shared[0], T::from_f64(0.5).unwrap());
        assert_eq!(variances[0], T::from_f64(0.5).unwrap());
        let input = shared.as_ptr();
        assert_eq!(
            (api.smooth)(input, variances.as_ptr(), &one, 1, shared.as_mut_ptr(), 1),
            0
        );
        assert_eq!(shared[0], T::from_f64(0.5).unwrap());
    }
}
#[test]
fn float_original_reference() {
    compare::<f32>(
        include_bytes!("../../../tests/fixtures/kalman-float.bin"),
        4,
        &Api {
            filter: ciglet_rs_kalman_filter_f32,
            initial: ciglet_rs_kalman_filter_with_initial_f32,
            smooth: ciglet_rs_kalman_smooth_f32,
        },
    );
}
#[test]
fn float_independent_posterior_and_smoothing() {
    analytic(
        &Api {
            filter: ciglet_rs_kalman_filter_f32,
            initial: ciglet_rs_kalman_filter_with_initial_f32,
            smooth: ciglet_rs_kalman_smooth_f32,
        },
        1e-7,
    );
}
#[test]
fn float_atomic_errors_outputs_and_overlap() {
    errors(&Api {
        filter: ciglet_rs_kalman_filter_f32,
        initial: ciglet_rs_kalman_filter_with_initial_f32,
        smooth: ciglet_rs_kalman_smooth_f32,
    });
}
#[test]
fn double_original_reference() {
    compare::<f64>(
        include_bytes!("../../../tests/fixtures/kalman-double.bin"),
        8,
        &Api {
            filter: ciglet_rs_kalman_filter_f64,
            initial: ciglet_rs_kalman_filter_with_initial_f64,
            smooth: ciglet_rs_kalman_smooth_f64,
        },
    );
}
#[test]
fn double_independent_posterior_and_smoothing() {
    analytic(
        &Api {
            filter: ciglet_rs_kalman_filter_f64,
            initial: ciglet_rs_kalman_filter_with_initial_f64,
            smooth: ciglet_rs_kalman_smooth_f64,
        },
        1e-15,
    );
}
#[test]
fn double_atomic_errors_outputs_and_overlap() {
    errors(&Api {
        filter: ciglet_rs_kalman_filter_f64,
        initial: ciglet_rs_kalman_filter_with_initial_f64,
        smooth: ciglet_rs_kalman_smooth_f64,
    });
}

fn late_failure_and_pointers<T: Scalar>(api: &Api<T>) {
    let zero = T::zero();
    let one = T::one();
    let sentinel = T::from_f64(71.).unwrap();
    let maximum = T::from_f64(if std::mem::size_of::<T>() == 4 {
        f32::MAX as f64
    } else {
        f64::MAX
    })
    .unwrap();
    let mut means = [sentinel; 3];
    let mut variances = [sentinel; 3];
    let mut likelihood = sentinel;
    let values = [zero; 3];
    let process = [one, maximum];
    let noise = [one, maximum];
    // SAFETY: Initialized matching input allocations and distinct output ranges.
    // Deliberate overlaps and misalignment are rejected before any write.
    unsafe {
        assert_eq!(
            (api.filter)(
                values.as_ptr(),
                process.as_ptr(),
                noise.as_ptr(),
                2,
                means.as_mut_ptr(),
                variances.as_mut_ptr(),
                2,
                &mut likelihood
            ),
            3
        );
        assert_eq!(
            (api.initial)(
                values.as_ptr(),
                process.as_ptr(),
                noise.as_ptr(),
                2,
                zero,
                means.as_mut_ptr(),
                variances.as_mut_ptr(),
                2,
                &mut likelihood
            ),
            3
        );
        assert_eq!(means, [sentinel; 3]);
        assert_eq!(variances, [sentinel; 3]);
        assert_eq!(likelihood, sentinel);
        let posterior = [maximum, one, one];
        let process = [one, maximum, one];
        assert_eq!(
            (api.smooth)(
                values.as_ptr(),
                posterior.as_ptr(),
                process.as_ptr(),
                3,
                means.as_mut_ptr(),
                3
            ),
            3
        );
        assert_eq!(means, [sentinel; 3]);
        let ones = [one; 2];
        assert_eq!(
            (api.filter)(
                values.as_ptr(),
                ones.as_ptr(),
                ones.as_ptr(),
                2,
                means.as_mut_ptr(),
                means.as_mut_ptr().add(1),
                2,
                &mut likelihood
            ),
            2
        );
        assert_eq!(means, [sentinel; 3]);
        assert_eq!(likelihood, sentinel);
        let mut words = [0x47474747u32; 8];
        let invalid = words.as_mut_ptr().cast::<u8>().add(1).cast::<T>();
        assert_eq!(
            (api.filter)(
                &one,
                &one,
                &one,
                1,
                invalid,
                variances.as_mut_ptr(),
                1,
                &mut likelihood
            ),
            1
        );
        assert_eq!(
            (api.filter)(
                &one,
                &one,
                &one,
                1,
                means.as_mut_ptr(),
                invalid,
                1,
                &mut likelihood
            ),
            1
        );
        assert_eq!(
            (api.filter)(
                &one,
                &one,
                &one,
                1,
                means.as_mut_ptr(),
                variances.as_mut_ptr(),
                1,
                invalid
            ),
            1
        );
        assert_eq!((api.smooth)(&one, &one, &one, 1, invalid, 1), 1);
        assert_eq!(words, [0x47474747; 8]);
        assert_eq!(means, [sentinel; 3]);
        assert_eq!(variances, [sentinel; 3]);
        assert_eq!(likelihood, sentinel);
        assert_eq!(
            (api.filter)(
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut()
            ),
            1
        );
    }
}
#[test]
fn float_late_failure_partial_overlap_and_alignment() {
    late_failure_and_pointers(&Api {
        filter: ciglet_rs_kalman_filter_f32,
        initial: ciglet_rs_kalman_filter_with_initial_f32,
        smooth: ciglet_rs_kalman_smooth_f32,
    });
}
#[test]
fn double_late_failure_partial_overlap_and_alignment() {
    late_failure_and_pointers(&Api {
        filter: ciglet_rs_kalman_filter_f64,
        initial: ciglet_rs_kalman_filter_with_initial_f64,
        smooth: ciglet_rs_kalman_smooth_f64,
    });
}

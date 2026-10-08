#![cfg(feature = "c-api")]
use ciglet_rs::{c_api::*, statistics::Scalar};
use std::ptr;
type Custom<T, H> =
    unsafe extern "C" fn(T, *const T, *const T, *const T, *const T, usize, *mut *mut H) -> u32;
type Copy<T, H> = unsafe extern "C" fn(*const H, *mut T, *mut T, *mut T, *mut T, usize) -> u32;
type Replace<T, H> =
    unsafe extern "C" fn(*mut H, *const T, *const T, *const T, *const T, usize) -> u32;
struct Api<T, H> {
    create: unsafe extern "C" fn(T, T, *mut *mut H) -> u32,
    custom: Custom<T, H>,
    clone: unsafe extern "C" fn(*const H, *mut *mut H) -> u32,
    release: unsafe extern "C" fn(*mut *mut H) -> u32,
    count: unsafe extern "C" fn(*const H, *mut usize) -> u32,
    center: unsafe extern "C" fn(*const H, *mut T) -> u32,
    set_center: unsafe extern "C" fn(*mut H, T) -> u32,
    copy: Copy<T, H>,
    replace: Replace<T, H>,
    estimate: unsafe extern "C" fn(*const H, *const T, usize, *mut T) -> u32,
    selected: unsafe extern "C" fn(*const H, *const T, usize, u32, *mut T) -> u32,
}
struct Owner<'a, T, H> {
    raw: *mut H,
    api: &'a Api<T, H>,
}
impl<T, H> Drop for Owner<'_, T, H> {
    fn drop(&mut self) {
        // SAFETY: Initialized unique ownership slot; no active operations at drop.
        unsafe {
            (self.api.release)(&mut self.raw);
        }
    }
}
fn owner<T, H>(api: &Api<T, H>) -> Owner<'_, T, H> {
    Owner {
        raw: ptr::null_mut(),
        api,
    }
}
fn planes<T: Scalar, H>(api: &Api<T, H>, handle: *const H) -> [Vec<T>; 4] {
    let mut count = 0;
    // SAFETY: Test caller retains a typed immutable live owner and exclusive size.
    unsafe {
        assert_eq!((api.count)(handle, &mut count), 0);
    }
    let mut values = [
        vec![T::zero(); count],
        vec![T::zero(); count],
        vec![T::zero(); count],
        vec![T::zero(); count],
    ];
    // SAFETY: Four independent exclusively writable vectors matching kernel length.
    unsafe {
        assert_eq!(
            (api.copy)(
                handle,
                values[0].as_mut_ptr(),
                values[1].as_mut_ptr(),
                values[2].as_mut_ptr(),
                values[3].as_mut_ptr(),
                count
            ),
            0
        );
    }
    values
}
fn reference<T: Scalar, H>(
    bytes: &[u8],
    width: usize,
    tolerance: f64,
    estimate_tolerance: f64,
    api: &Api<T, H>,
) {
    assert_eq!(&bytes[..4], b"IFD1");
    assert_eq!(
        u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize,
        width
    );
    assert_eq!(u32::from_le_bytes(bytes[8..12].try_into().unwrap()), 24);
    let mut cursor = 12;
    let mut maximum_estimate_error = 0.0_f64;
    fn integer(bytes: &[u8], cursor: &mut usize) -> usize {
        let result = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap()) as usize;
        *cursor += 4;
        result
    }
    fn scalar<T: Scalar>(bytes: &[u8], cursor: &mut usize, width: usize) -> T {
        let value = if width == 4 {
            f32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(bytes[*cursor..*cursor + 8].try_into().unwrap())
        };
        *cursor += width;
        T::from_f64(value).unwrap()
    }
    for _ in 0..24 {
        let center = scalar::<T>(bytes, &mut cursor, width);
        let resolution = scalar::<T>(bytes, &mut cursor, width);
        let length = integer(bytes, &mut cursor);
        let mut detector = owner(api);
        // SAFETY: Distinct initialized exclusive ownership slot and valid scalars.
        unsafe {
            assert_eq!((api.create)(center, resolution, &mut detector.raw), 0);
        }
        assert!(!detector.raw.is_null());
        let filters = planes(api, detector.raw);
        assert_eq!(filters[0].len(), length);
        let mut center_output = T::zero();
        // SAFETY: Live immutable owner and independent exclusively writable scalar.
        unsafe {
            assert_eq!((api.center)(detector.raw, &mut center_output), 0);
        }
        assert_eq!(center_output, center);
        for filter in &filters {
            for &actual in filter {
                let expected = scalar::<T>(bytes, &mut cursor, width);
                if width == 4 {
                    assert_eq!(actual.into().to_bits(), expected.into().to_bits());
                }
                assert!(
                    (actual.into() - expected.into()).abs()
                        <= tolerance * (1.0 + expected.into().abs()),
                    "kernel {} != {}",
                    actual.into(),
                    expected.into()
                );
            }
        }
        for _ in 0..4 {
            let nx = integer(bytes, &mut cursor);
            let samples: Vec<T> = (0..nx).map(|_| scalar(bytes, &mut cursor, width)).collect();
            let expected = scalar::<T>(bytes, &mut cursor, width);
            let mut actual = T::zero();
            // SAFETY: Live immutable owner, initialized readable samples and exclusive
            // independent scalar output. Empty Vec carries zero count.
            unsafe {
                assert_eq!(
                    (api.selected)(
                        detector.raw,
                        samples.as_ptr(),
                        samples.len(),
                        1,
                        &mut actual
                    ),
                    0
                );
            }
            maximum_estimate_error = maximum_estimate_error
                .max((actual.into() - expected.into()).abs() / (1.0 + expected.into().abs()));
            assert!(
                (actual.into() - expected.into()).abs()
                    <= estimate_tolerance * (1.0 + expected.into().abs()),
                "estimate {} != {}",
                actual.into(),
                expected.into()
            );
            if width == 4 {
                assert_eq!(actual.into().to_bits(), expected.into().to_bits());
            }
        }
    }
    assert_eq!(cursor, bytes.len());
    eprintln!("width={width}, maximum normalized estimate error={maximum_estimate_error:e}");
}
fn configuration<T: Scalar, H>(api: &Api<T, H>, maximum: T, minimum: T, tolerance: f64) {
    let v = |x| T::from_f64(x).unwrap();
    let r = [v(1.), v(0.)];
    let i = [v(0.), v(1.)];
    let dr = [v(0.), v(-2.)];
    let di = [v(2.), v(0.)];
    let mut detector = owner(api);
    let mut cloned = owner(api);
    let mut result = v(71.);
    // SAFETY: Live precision-matched unique owners, independent initialized inputs
    // and exclusive scalar/ownership outputs. Aliased sample result is staged.
    unsafe {
        assert_eq!(
            (api.custom)(
                v(0.125),
                r.as_ptr(),
                i.as_ptr(),
                dr.as_ptr(),
                di.as_ptr(),
                2,
                &mut detector.raw
            ),
            0
        );
        assert_eq!((api.clone)(detector.raw, &mut cloned.raw), 0);
        assert_eq!((api.set_center)(cloned.raw, v(0.25)), 0);
        assert_eq!((api.center)(detector.raw, &mut result), 0);
        assert_eq!(result, v(0.125));
        assert_eq!((api.center)(cloned.raw, &mut result), 0);
        assert_eq!(result, v(0.25));
        for samples in [
            [v(1.), v(2.)],
            [maximum, maximum / v(2.)],
            [minimum, minimum * v(2.)],
        ] {
            assert_eq!(
                (api.estimate)(detector.raw, samples.as_ptr(), 2, &mut result),
                0
            );
            assert!((result.into() - 1. / std::f64::consts::PI).abs() < tolerance);
            let mut selected = v(71.);
            assert_eq!(
                (api.selected)(detector.raw, samples.as_ptr(), 2, 0, &mut selected),
                0
            );
            assert_eq!(result, selected);
        }
        let surrounded = [v(f64::NAN), v(1.), v(2.), v(f64::NAN)];
        assert_eq!(
            (api.estimate)(detector.raw, surrounded.as_ptr(), 4, &mut result),
            0
        );
        assert!((result.into() - 1. / std::f64::consts::PI).abs() < tolerance);
        assert_eq!(
            (api.estimate)(detector.raw, surrounded.as_ptr(), 1, &mut result),
            0
        );
        assert_eq!(result, v(0.));
        assert_eq!((api.estimate)(detector.raw, ptr::null(), 0, &mut result), 0);
        assert_eq!(result, v(0.));
        let mut shared = [v(1.), v(2.)];
        assert_eq!(
            (api.estimate)(detector.raw, shared.as_ptr(), 2, shared.as_mut_ptr()),
            0
        );
        assert!((shared[0].into() - 1. / std::f64::consts::PI).abs() < tolerance);
        let singleton = [v(1.)];
        let zero = [v(0.)];
        let derivative = [v(4.)];
        assert_eq!(
            (api.replace)(
                cloned.raw,
                singleton.as_ptr(),
                zero.as_ptr(),
                zero.as_ptr(),
                derivative.as_ptr(),
                1
            ),
            0
        );
        assert_eq!(
            planes(api, cloned.raw),
            [vec![v(1.)], vec![v(0.)], vec![v(0.)], vec![v(4.)]]
        );
        assert_eq!(
            planes(api, detector.raw),
            [r.to_vec(), i.to_vec(), dr.to_vec(), di.to_vec()]
        );
        assert_eq!((api.center)(cloned.raw, &mut result), 0);
        assert_eq!(result, v(0.25));
        assert_eq!((api.release)(&mut cloned.raw), 0);
        assert!(cloned.raw.is_null());
        assert_eq!((api.release)(&mut cloned.raw), 0);
    }
}
fn errors<T: Scalar, H>(api: &Api<T, H>) {
    let v = |x| T::from_f64(x).unwrap();
    let mut detector = owner(api);
    let mut unchanged = owner(api);
    let mut result = v(71.);
    let r = [v(1.), v(0.)];
    let i = [v(0.), v(1.)];
    let dr = [v(0.), v(-2.)];
    let di = [v(2.), v(0.)];
    // SAFETY: Initialized independent storage and live unique owners. Invalid
    // pointers/counts/overlaps are rejected before accessing or changing output.
    unsafe {
        assert_eq!(
            (api.custom)(
                v(0.125),
                r.as_ptr(),
                i.as_ptr(),
                dr.as_ptr(),
                di.as_ptr(),
                2,
                &mut detector.raw
            ),
            0
        );
        for resolution in [v(0.), v(-1.), v(f64::NAN), v(f64::INFINITY), v(5.)] {
            assert_eq!((api.create)(v(0.125), resolution, &mut unchanged.raw), 3);
            assert!(unchanged.raw.is_null());
        }
        assert_eq!(
            (api.custom)(
                v(0.125),
                ptr::null(),
                ptr::null(),
                ptr::null(),
                ptr::null(),
                0,
                &mut unchanged.raw
            ),
            3
        );
        assert!(unchanged.raw.is_null());
        assert_eq!((api.set_center)(detector.raw, v(f64::NAN)), 3);
        assert_eq!((api.center)(detector.raw, &mut result), 0);
        assert_eq!(result, v(0.125));
        let bad = [v(f64::NAN), v(0.)];
        assert_eq!(
            (api.replace)(
                detector.raw,
                bad.as_ptr(),
                i.as_ptr(),
                dr.as_ptr(),
                di.as_ptr(),
                2
            ),
            3
        );
        assert_eq!(
            (api.replace)(
                detector.raw,
                ptr::null(),
                ptr::null(),
                ptr::null(),
                ptr::null(),
                0
            ),
            3
        );
        assert_eq!(
            planes(api, detector.raw),
            [r.to_vec(), i.to_vec(), dr.to_vec(), di.to_vec()]
        );
        result = v(71.);
        let silence = [v(0.); 2];
        for policy in [0, 1, 2] {
            assert_eq!(
                (api.selected)(detector.raw, silence.as_ptr(), 2, policy, &mut result),
                3
            );
            assert_eq!(result, v(71.));
        }
        assert_eq!(
            (api.estimate)(detector.raw, bad.as_ptr(), 2, &mut result),
            3
        );
        assert_eq!(result, v(71.));
        assert_eq!((api.estimate)(ptr::null(), r.as_ptr(), 2, &mut result), 1);
        assert_eq!((api.estimate)(detector.raw, ptr::null(), 2, &mut result), 1);
        assert_eq!(
            (api.estimate)(detector.raw, r.as_ptr(), 2, ptr::null_mut()),
            1
        );
        let mut count = 71;
        assert_eq!((api.count)(ptr::null(), &mut count), 1);
        assert_eq!(count, 71);
        let mut out = [
            vec![v(71.); 2],
            vec![v(71.); 2],
            vec![v(71.); 2],
            vec![v(71.); 2],
        ];
        let pointers = [
            out[0].as_mut_ptr(),
            out[1].as_mut_ptr(),
            out[2].as_mut_ptr(),
            out[3].as_mut_ptr(),
        ];
        for a in 0..4 {
            for b in a + 1..4 {
                let mut aliased = pointers;
                aliased[b] = aliased[a];
                assert_eq!(
                    (api.copy)(
                        detector.raw,
                        aliased[0],
                        aliased[1],
                        aliased[2],
                        aliased[3],
                        2
                    ),
                    2
                );
            }
        }
        assert_eq!(
            (api.copy)(
                detector.raw,
                pointers[0],
                pointers[1],
                pointers[2],
                pointers[3],
                1
            ),
            2
        );
        assert!(out.iter().all(|o| *o == vec![v(71.); 2]));
        assert_eq!((api.release)(ptr::null_mut()), 1);
    }
}
#[test]
fn float_original_reference() {
    reference::<f32, CigletRsIfDetectorF32>(
        include_bytes!("fixtures/instantaneous-frequency-float.bin"),
        4,
        2e-7,
        0.,
        &Api {
            create: ciglet_rs_if_detector_create_f32,
            custom: ciglet_rs_if_detector_from_kernels_f32,
            clone: ciglet_rs_if_detector_clone_f32,
            release: ciglet_rs_if_detector_release_f32,
            count: ciglet_rs_if_detector_kernel_count_f32,
            center: ciglet_rs_if_detector_center_frequency_f32,
            set_center: ciglet_rs_if_detector_set_center_frequency_f32,
            copy: ciglet_rs_if_detector_copy_kernels_f32,
            replace: ciglet_rs_if_detector_replace_kernels_f32,
            estimate: ciglet_rs_if_detector_estimate_f32,
            selected: ciglet_rs_if_detector_estimate_with_f32,
        },
    );
}
#[test]
fn float_configuration_independence_and_extremes() {
    configuration(
        &Api {
            create: ciglet_rs_if_detector_create_f32,
            custom: ciglet_rs_if_detector_from_kernels_f32,
            clone: ciglet_rs_if_detector_clone_f32,
            release: ciglet_rs_if_detector_release_f32,
            count: ciglet_rs_if_detector_kernel_count_f32,
            center: ciglet_rs_if_detector_center_frequency_f32,
            set_center: ciglet_rs_if_detector_set_center_frequency_f32,
            copy: ciglet_rs_if_detector_copy_kernels_f32,
            replace: ciglet_rs_if_detector_replace_kernels_f32,
            estimate: ciglet_rs_if_detector_estimate_f32,
            selected: ciglet_rs_if_detector_estimate_with_f32,
        },
        f32::MAX,
        f32::from_bits(1),
        1e-7,
    );
}
#[test]
fn float_atomic_errors_and_all_plane_overlaps() {
    errors(&Api {
        create: ciglet_rs_if_detector_create_f32,
        custom: ciglet_rs_if_detector_from_kernels_f32,
        clone: ciglet_rs_if_detector_clone_f32,
        release: ciglet_rs_if_detector_release_f32,
        count: ciglet_rs_if_detector_kernel_count_f32,
        center: ciglet_rs_if_detector_center_frequency_f32,
        set_center: ciglet_rs_if_detector_set_center_frequency_f32,
        copy: ciglet_rs_if_detector_copy_kernels_f32,
        replace: ciglet_rs_if_detector_replace_kernels_f32,
        estimate: ciglet_rs_if_detector_estimate_f32,
        selected: ciglet_rs_if_detector_estimate_with_f32,
    });
}
#[test]
fn double_original_reference() {
    reference::<f64, CigletRsIfDetectorF64>(
        include_bytes!("fixtures/instantaneous-frequency-double.bin"),
        8,
        3e-15,
        1e-12,
        &Api {
            create: ciglet_rs_if_detector_create_f64,
            custom: ciglet_rs_if_detector_from_kernels_f64,
            clone: ciglet_rs_if_detector_clone_f64,
            release: ciglet_rs_if_detector_release_f64,
            count: ciglet_rs_if_detector_kernel_count_f64,
            center: ciglet_rs_if_detector_center_frequency_f64,
            set_center: ciglet_rs_if_detector_set_center_frequency_f64,
            copy: ciglet_rs_if_detector_copy_kernels_f64,
            replace: ciglet_rs_if_detector_replace_kernels_f64,
            estimate: ciglet_rs_if_detector_estimate_f64,
            selected: ciglet_rs_if_detector_estimate_with_f64,
        },
    );
}
#[test]
fn double_configuration_independence_and_extremes() {
    configuration(
        &Api {
            create: ciglet_rs_if_detector_create_f64,
            custom: ciglet_rs_if_detector_from_kernels_f64,
            clone: ciglet_rs_if_detector_clone_f64,
            release: ciglet_rs_if_detector_release_f64,
            count: ciglet_rs_if_detector_kernel_count_f64,
            center: ciglet_rs_if_detector_center_frequency_f64,
            set_center: ciglet_rs_if_detector_set_center_frequency_f64,
            copy: ciglet_rs_if_detector_copy_kernels_f64,
            replace: ciglet_rs_if_detector_replace_kernels_f64,
            estimate: ciglet_rs_if_detector_estimate_f64,
            selected: ciglet_rs_if_detector_estimate_with_f64,
        },
        f64::MAX,
        f64::from_bits(1),
        2e-16,
    );
}
#[test]
fn double_atomic_errors_and_all_plane_overlaps() {
    errors(&Api {
        create: ciglet_rs_if_detector_create_f64,
        custom: ciglet_rs_if_detector_from_kernels_f64,
        clone: ciglet_rs_if_detector_clone_f64,
        release: ciglet_rs_if_detector_release_f64,
        count: ciglet_rs_if_detector_kernel_count_f64,
        center: ciglet_rs_if_detector_center_frequency_f64,
        set_center: ciglet_rs_if_detector_set_center_frequency_f64,
        copy: ciglet_rs_if_detector_copy_kernels_f64,
        replace: ciglet_rs_if_detector_replace_kernels_f64,
        estimate: ciglet_rs_if_detector_estimate_f64,
        selected: ciglet_rs_if_detector_estimate_with_f64,
    });
}

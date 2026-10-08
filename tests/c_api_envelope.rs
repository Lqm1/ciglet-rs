#![cfg(feature = "c-api")]
use ciglet_rs::{c_api::*, statistics::Scalar};
use std::ptr;
type Selected<T, H, O, R> = unsafe extern "C" fn(*const H, *const T, usize, O, u32, R) -> u32;
type Both<T, H, O, R> = unsafe extern "C" fn(*const H, *const T, usize, O, u32, u32, R) -> u32;
struct Api<T, H, O, R> {
    create: unsafe extern "C" fn(usize, *mut *mut H) -> u32,
    release: unsafe extern "C" fn(*mut *mut H) -> u32,
    len: unsafe extern "C" fn(*const H, *mut usize) -> u32,
    bins: unsafe extern "C" fn(*const H, *mut usize) -> u32,
    empty: unsafe extern "C" fn(*const H, *mut usize) -> u32,
    estimate: unsafe extern "C" fn(*const H, *const T, usize, O, R) -> u32,
    selected: Selected<T, H, O, R>,
    both: Both<T, H, O, R>,
    one: unsafe extern "C" fn(usize, *const T, usize, O, R) -> u32,
    options: fn(T, usize) -> O,
    output: fn(*mut T, usize, *mut T, usize) -> R,
}
struct Owner<'a, T, H, O, R> {
    raw: *mut H,
    api: &'a Api<T, H, O, R>,
}
impl<T, H, O, R> Drop for Owner<'_, T, H, O, R> {
    fn drop(&mut self) {
        // SAFETY: Unique initialized slot owns the matching live handle or is null.
        assert_eq!(unsafe { (self.api.release)(&mut self.raw) }, 0);
    }
}
fn create<T, H, O, R>(api: &Api<T, H, O, R>, length: usize) -> Owner<'_, T, H, O, R> {
    let mut owner = Owner {
        raw: ptr::null_mut(),
        api,
    };
    // SAFETY: Independent initialized exclusive owner output slot.
    assert_eq!(unsafe { (api.create)(length, &mut owner.raw) }, 0);
    owner
}
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
    fn values<T: Scalar>(&mut self, count: usize) -> Vec<T> {
        (0..count)
            .map(|_| {
                let end = self.cursor + self.width;
                let value = if self.width == 4 {
                    f64::from(f32::from_le_bytes(
                        self.bytes[self.cursor..end].try_into().unwrap(),
                    ))
                } else {
                    f64::from_le_bytes(self.bytes[self.cursor..end].try_into().unwrap())
                };
                self.cursor = end;
                T::from_f64(value).unwrap()
            })
            .collect()
    }
}
fn reference<T: Scalar, H, O: Copy, R: Copy>(
    bytes: &[u8],
    width: usize,
    tolerance: f64,
    api: &Api<T, H, O, R>,
) {
    assert_eq!(&bytes[..4], b"ENV1");
    let mut reader = Reader {
        bytes,
        cursor: 4,
        width,
    };
    assert_eq!(reader.integer(), width);
    assert_eq!(reader.integer(), 144);
    let mut total = 0;
    for _ in 0..144 {
        let n = reader.integer();
        let harmonics = reader.integer();
        let fundamental = reader.values::<T>(1)[0];
        let spectrum = reader.values::<T>(n / 2 + 1);
        let expected_log = reader.values::<T>(n);
        let expected_cepstrum = reader.values::<T>(n / 2 + 1);
        let owner = create(api, n);
        let mut log = vec![T::zero(); n];
        let mut cepstrum = vec![T::zero(); n / 2 + 1];
        let options = (api.options)(fundamental, harmonics);
        let output = (api.output)(log.as_mut_ptr(), n, cepstrum.as_mut_ptr(), n / 2 + 1);
        // SAFETY: Retained live owner, stable readable spectrum and disjoint outputs.
        assert_eq!(
            unsafe {
                (api.both)(
                    owner.raw,
                    spectrum.as_ptr(),
                    spectrum.len(),
                    options,
                    1,
                    1,
                    output,
                )
            },
            0
        );
        for (actual, expected) in [(&log, &expected_log), (&cepstrum, &expected_cepstrum)] {
            for (&a, &e) in actual.iter().zip(expected) {
                assert!(
                    (a.into() - e.into()).abs() / (1. + e.into().abs()) <= tolerance,
                    "{} != {}",
                    a.into(),
                    e.into()
                );
            }
        }
        total += n + n / 2 + 1;
    }
    assert_eq!(total, 18288);
    assert_eq!(reader.cursor, bytes.len());
}
fn policies<T: Scalar, H, O: Copy, R: Copy>(api: &Api<T, H, O, R>, tolerance: f64) {
    let v = |x| T::from_f64(x).unwrap();
    let owner = create(api, 32);
    let spectrum = [v(0.5); 17];
    let mut log = [v(71.); 32];
    let mut cepstrum = [v(71.); 17];
    let output = (api.output)(log.as_mut_ptr(), 32, cepstrum.as_mut_ptr(), 17);
    let options = (api.options)(v(0.09375), 0);
    let mut value = 71;
    // SAFETY: Live owner, readable initialized input and disjoint exclusive outputs.
    unsafe {
        assert_eq!((api.len)(owner.raw, &mut value), 0);
        assert_eq!(value, 32);
        assert_eq!((api.bins)(owner.raw, &mut value), 0);
        assert_eq!(value, 17);
        assert_eq!((api.empty)(owner.raw, &mut value), 0);
        assert_eq!(value, 0);
        assert_eq!(
            (api.estimate)(owner.raw, spectrum.as_ptr(), 17, options, output),
            0
        );
        let expected = ciglet_rs::approximation::logarithm(0.5 + 1e-15).unwrap() as f64;
        for &x in &log {
            assert!((x.into() - expected).abs() < tolerance);
        }
        assert!((cepstrum[0].into() - expected).abs() < tolerance);
        for &x in &cepstrum[1..] {
            assert!(x.into().abs() < tolerance);
        }
        let baseline = (log, cepstrum);
        assert_eq!(
            (api.selected)(owner.raw, spectrum.as_ptr(), 17, options, 0, output),
            0
        );
        assert_eq!((log, cepstrum), baseline);
        assert_eq!(
            (api.both)(owner.raw, spectrum.as_ptr(), 17, options, 0, 0, output),
            0
        );
        assert_eq!((log, cepstrum), baseline);
        assert_eq!((api.one)(32, spectrum.as_ptr(), 17, options, output), 0);
        assert_eq!((log, cepstrum), baseline);
        let ramp: Vec<_> = (0..17).map(|i| v(0.25 + f64::from(i) * 0.125)).collect();
        let options = (api.options)(v(0.125), 5);
        for nyquist in 0..2 {
            for smoothing in 0..2 {
                assert_eq!(
                    (api.both)(
                        owner.raw,
                        ramp.as_ptr(),
                        17,
                        options,
                        nyquist,
                        smoothing,
                        output
                    ),
                    0
                );
                let first = (log, cepstrum);
                assert_eq!(
                    (api.both)(
                        owner.raw,
                        ramp.as_ptr(),
                        17,
                        options,
                        nyquist,
                        smoothing,
                        output
                    ),
                    0
                );
                assert_eq!((log, cepstrum), first);
                for i in 17..32 {
                    assert!((log[i].into() - log[32 - i].into()).abs() < tolerance);
                }
                if nyquist == 1 {
                    assert_eq!(cepstrum[16], cepstrum[15]);
                }
                if smoothing == 0 {
                    assert_eq!(
                        (api.selected)(owner.raw, ramp.as_ptr(), 17, options, nyquist, output),
                        0
                    );
                    assert_eq!((log, cepstrum), first);
                }
            }
        }
        log[..17].copy_from_slice(&ramp);
        let input = log.as_ptr();
        assert_eq!((api.estimate)(owner.raw, input, 17, options, output), 0);
        let overlapping = (log, cepstrum);
        assert_eq!(
            (api.estimate)(owner.raw, ramp.as_ptr(), 17, options, output),
            0
        );
        assert_eq!((log, cepstrum), overlapping);
        let tiny = create(api, 2);
        let mut tl = [T::zero(); 2];
        let mut tc = [T::zero(); 2];
        let to = (api.output)(tl.as_mut_ptr(), 2, tc.as_mut_ptr(), 2);
        let ts = [v(0.5), v(1.)];
        assert_eq!(
            (api.estimate)(tiny.raw, ts.as_ptr(), 2, (api.options)(v(1e-10), 0), to),
            0
        );
        let implicit = (tl, tc);
        assert_eq!(
            (api.estimate)(tiny.raw, ts.as_ptr(), 2, (api.options)(v(1e-10), 2), to),
            0
        );
        assert_eq!((tl, tc), implicit);
        assert_eq!(tl[0], tl[1]);
        assert_eq!(
            (api.estimate)(
                owner.raw,
                spectrum.as_ptr(),
                17,
                (api.options)(v(0.125), 2),
                output
            ),
            0
        );
        for &x in &log {
            assert!((x.into() - log[0].into()).abs() < tolerance);
        }
    }
}
fn concurrent<T: Scalar, H: Sync, O: Copy, R: Copy>(api: &Api<T, H, O, R>) {
    let owner = create(api, 32);
    // SAFETY: Successful constructor returns a live owner. The scope joins all
    // workers before owner release; mutable FFT scratch remains behind its mutex.
    let handle = unsafe { &*owner.raw };
    std::thread::scope(|scope| {
        for worker in 0..4 {
            scope.spawn(move || {
                let spectrum: Vec<T> = (0..17)
                    .map(|i| T::from_f64(0.25 + f64::from(i + worker) * 0.125).unwrap())
                    .collect();
                let mut log = vec![T::zero(); 32];
                let mut cepstrum = vec![T::zero(); 17];
                let output = (api.output)(log.as_mut_ptr(), 32, cepstrum.as_mut_ptr(), 17);
                let options = (api.options)(T::from_f64(0.125).unwrap(), 5);
                let mut baseline = None;
                for _ in 0..50 {
                    // SAFETY: Shared retained owner serializes scratch; each worker
                    // has independent initialized arrays and exclusive output planes.
                    assert_eq!(
                        unsafe { (api.estimate)(handle, spectrum.as_ptr(), 17, options, output) },
                        0
                    );
                    if let Some((expected_log, expected_cepstrum)) = &baseline {
                        assert_eq!((&log, &cepstrum), (expected_log, expected_cepstrum));
                    } else {
                        baseline = Some((log.clone(), cepstrum.clone()));
                    }
                }
            });
        }
    });
}

fn errors<T: Scalar, H, O: Copy, R: Copy>(api: &Api<T, H, O, R>) {
    let v = |x| T::from_f64(x).unwrap();
    let owner = create(api, 8);
    let mut slot = ptr::null_mut();
    let spectrum = [v(0.5); 5];
    let mut log = [v(71.); 8];
    let mut cepstrum = [v(71.); 5];
    let output = (api.output)(log.as_mut_ptr(), 8, cepstrum.as_mut_ptr(), 5);
    let options = (api.options)(v(0.125), 0);
    // SAFETY: Invalid pointer/count/alignment/overlap inputs are rejected before
    // access; all otherwise used buffers are initialized exclusive outputs/readable inputs.
    unsafe {
        for length in [0, 1, 7, usize::MAX] {
            assert_eq!((api.create)(length, &mut slot), 3);
            assert!(slot.is_null());
        }
        for f0 in [0., -0.1, 0.5001, f64::NAN, f64::INFINITY] {
            assert_eq!(
                (api.estimate)(
                    owner.raw,
                    spectrum.as_ptr(),
                    5,
                    (api.options)(v(f0), 0),
                    output
                ),
                3
            );
        }
        for count in [1, i32::MAX as usize + 1] {
            assert_eq!(
                (api.estimate)(
                    owner.raw,
                    spectrum.as_ptr(),
                    5,
                    (api.options)(v(0.125), count),
                    output
                ),
                3
            );
        }
        for invalid in [[v(-1.); 5], [v(f64::NAN); 5], [v(f64::INFINITY); 5]] {
            assert_eq!(
                (api.estimate)(owner.raw, invalid.as_ptr(), 5, options, output),
                3
            );
        }
        assert_eq!(
            (api.selected)(owner.raw, spectrum.as_ptr(), 5, options, 2, output),
            3
        );
        assert_eq!(
            (api.both)(owner.raw, spectrum.as_ptr(), 5, options, 0, 2, output),
            3
        );
        assert_eq!(
            (api.estimate)(owner.raw, spectrum.as_ptr(), 4, options, output),
            3
        );
        assert_eq!(
            (api.estimate)(owner.raw, ptr::null(), 0, options, output),
            3
        );
        assert_eq!(
            (api.estimate)(owner.raw, ptr::null(), 5, options, output),
            1
        );
        assert_eq!(
            (api.estimate)(ptr::null(), spectrum.as_ptr(), 5, options, output),
            1
        );
        assert_eq!(
            (api.estimate)(owner.raw, spectrum.as_ptr(), usize::MAX, options, output),
            2
        );
        let wrong = (api.output)(log.as_mut_ptr(), 7, cepstrum.as_mut_ptr(), 5);
        assert_eq!(
            (api.estimate)(owner.raw, spectrum.as_ptr(), 5, options, wrong),
            2
        );
        let wrong = (api.output)(log.as_mut_ptr(), 8, cepstrum.as_mut_ptr(), 4);
        assert_eq!(
            (api.estimate)(owner.raw, spectrum.as_ptr(), 5, options, wrong),
            2
        );
        for offset in [0, 1] {
            let overlap = (api.output)(log.as_mut_ptr(), 8, log.as_mut_ptr().add(offset), 5);
            assert_eq!(
                (api.estimate)(owner.raw, spectrum.as_ptr(), 5, options, overlap),
                2
            );
        }
        let bad = log.as_mut_ptr().cast::<u8>().add(1).cast::<T>();
        assert_eq!(
            (api.estimate)(
                owner.raw,
                spectrum.as_ptr(),
                5,
                options,
                (api.output)(bad, 8, cepstrum.as_mut_ptr(), 5)
            ),
            1
        );
        assert_eq!(
            (api.estimate)(
                owner.raw,
                spectrum.as_ptr(),
                5,
                options,
                (api.output)(log.as_mut_ptr(), 8, ptr::null_mut(), 5)
            ),
            1
        );
        let mut query = 71;
        assert_eq!((api.len)(ptr::null(), &mut query), 1);
        assert_eq!(query, 71);
        assert_eq!(
            (api.one)(8, spectrum.as_ptr(), 5, (api.options)(v(0.), 0), output),
            3
        );
        assert_eq!(log, [v(71.); 8]);
        assert_eq!(cepstrum, [v(71.); 5]);
        assert_eq!((api.release)(&mut slot), 0);
        assert_eq!((api.release)(&mut slot), 0);
    }
}

const FLOAT: Api<f32, CigletRsEnvelopeF32, CigletRsEnvelopeOptionsF32, CigletRsEnvelopeOutputF32> =
    Api {
        create: ciglet_rs_envelope_create_f32,
        release: ciglet_rs_envelope_release_f32,
        len: ciglet_rs_envelope_len_f32,
        bins: ciglet_rs_envelope_bins_f32,
        empty: ciglet_rs_envelope_is_empty_f32,
        estimate: ciglet_rs_envelope_estimate_f32,
        selected: ciglet_rs_envelope_estimate_with_f32,
        both: ciglet_rs_envelope_estimate_with_smoothing_f32,
        one: ciglet_rs_envelope_one_shot_f32,
        options: |fundamental, harmonics| CigletRsEnvelopeOptionsF32 {
            fundamental,
            harmonics,
        },
        output: |log_spectrum, log_count, cepstrum, cepstrum_count| CigletRsEnvelopeOutputF32 {
            log_spectrum,
            log_count,
            cepstrum,
            cepstrum_count,
        },
    };
#[test]
fn float_original_reference() {
    reference(
        include_bytes!("fixtures/spectral-envelope-float.bin"),
        4,
        3e-7,
        &FLOAT,
    );
}
#[test]
fn float_policies_metadata_and_overlap() {
    policies(&FLOAT, 2e-6);
}
#[test]
fn float_atomic_errors() {
    errors(&FLOAT);
}
#[test]
fn float_concurrent_scratch_is_serialized() {
    concurrent(&FLOAT);
}

const DOUBLE: Api<f64, CigletRsEnvelopeF64, CigletRsEnvelopeOptionsF64, CigletRsEnvelopeOutputF64> =
    Api {
        create: ciglet_rs_envelope_create_f64,
        release: ciglet_rs_envelope_release_f64,
        len: ciglet_rs_envelope_len_f64,
        bins: ciglet_rs_envelope_bins_f64,
        empty: ciglet_rs_envelope_is_empty_f64,
        estimate: ciglet_rs_envelope_estimate_f64,
        selected: ciglet_rs_envelope_estimate_with_f64,
        both: ciglet_rs_envelope_estimate_with_smoothing_f64,
        one: ciglet_rs_envelope_one_shot_f64,
        options: |fundamental, harmonics| CigletRsEnvelopeOptionsF64 {
            fundamental,
            harmonics,
        },
        output: |log_spectrum, log_count, cepstrum, cepstrum_count| CigletRsEnvelopeOutputF64 {
            log_spectrum,
            log_count,
            cepstrum,
            cepstrum_count,
        },
    };
#[test]
fn double_original_reference() {
    reference(
        include_bytes!("fixtures/spectral-envelope-double.bin"),
        8,
        5e-16,
        &DOUBLE,
    );
}
#[test]
fn double_policies_metadata_and_overlap() {
    policies(&DOUBLE, 2e-13);
}
#[test]
fn double_atomic_errors() {
    errors(&DOUBLE);
}
#[test]
fn double_concurrent_scratch_is_serialized() {
    concurrent(&DOUBLE);
}

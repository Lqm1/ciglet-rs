use ciglet_rs::statistics::Scalar;
use ciglet_rs_capi::*;
use std::ptr;

type ModelSpectrum<T> =
    unsafe extern "C" fn(*const T, usize, *const T, usize, *mut T, *mut T, usize) -> u32;
type SelectedSpectrum<T, H> =
    unsafe extern "C" fn(*const H, *const T, usize, u32, *mut T, *mut T, usize) -> u32;
struct Api<T, H> {
    shape: unsafe extern "C" fn(T, T, T, *mut T, usize) -> u32,
    one_spectrum: ModelSpectrum<T>,
    one_period: unsafe extern "C" fn(*const T, usize, i32, usize, *mut T, usize) -> u32,
    create: unsafe extern "C" fn(*const T, usize, *mut *mut H) -> u32,
    clone: unsafe extern "C" fn(*const H, *mut *mut H) -> u32,
    release: unsafe extern "C" fn(*mut *mut H) -> u32,
    model: unsafe extern "C" fn(*const H, *mut T, usize) -> u32,
    spectrum: unsafe extern "C" fn(*const H, *const T, usize, *mut T, *mut T, usize) -> u32,
    selected: SelectedSpectrum<T, H>,
    period: unsafe extern "C" fn(*const H, i32, usize, *mut T, usize) -> u32,
}
struct Owner<'a, T, H> {
    raw: *mut H,
    api: &'a Api<T, H>,
}
impl<T, H> Drop for Owner<'_, T, H> {
    fn drop(&mut self) {
        // SAFETY: This unique slot owns the matching live handle or is null.
        assert_eq!(unsafe { (self.api.release)(&mut self.raw) }, 0);
    }
}
fn create<'a, T, H>(api: &'a Api<T, H>, fields: &[T]) -> Owner<'a, T, H> {
    let mut owner = Owner {
        raw: ptr::null_mut(),
        api,
    };
    // SAFETY: Stable readable initialized configuration and independent owner slot.
    assert_eq!(
        unsafe { (api.create)(fields.as_ptr(), fields.len(), &mut owner.raw) },
        0
    );
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
    fn values<T: Scalar>(&mut self, n: usize) -> Vec<T> {
        (0..n)
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
fn compare<T: Scalar>(actual: &[T], expected: &[T], width: usize, tolerance: f64) {
    assert_eq!(actual.len(), expected.len());
    for (&a, &e) in actual.iter().zip(expected) {
        let a: f64 = a.into();
        let e: f64 = e.into();
        if width == 4 || e == 0. {
            assert_eq!(a.to_bits(), e.to_bits());
        }
        assert!((a - e).abs() / (1. + e.abs()) <= tolerance, "{a} != {e}");
    }
}
fn reference<T: Scalar, H>(bytes: &[u8], width: usize, tolerance: [f64; 3], api: &Api<T, H>) {
    assert_eq!(&bytes[..4], b"LFM1");
    let mut reader = Reader {
        bytes,
        cursor: 4,
        width,
    };
    assert_eq!(reader.integer(), width);
    assert_eq!(reader.integer(), 53);
    let mut outputs = 0;
    for _ in 0..53 {
        let input = reader.values::<T>(3);
        let expected_fields = reader.values::<T>(5);
        let mut fields = expected_fields.clone();
        if input[0] != T::zero() {
            // SAFETY: Five exclusive initialized output fields.
            assert_eq!(
                unsafe { (api.shape)(input[0], input[1], input[2], fields.as_mut_ptr(), 5) },
                0
            );
        }
        compare(&fields, &expected_fields, 4, 0.);
        let rate = reader.integer() as i32;
        let count = reader.integer();
        let nf = reader.integer();
        let frequencies = reader.values::<T>(nf);
        let em = reader.values::<T>(nf);
        let ep = reader.values::<T>(nf);
        let ew = reader.values::<T>(count);
        let owner = create(api, &fields);
        let mut copied = vec![T::zero(); 5];
        let mut magnitude = vec![T::zero(); nf];
        let mut phase = vec![T::zero(); nf];
        let mut wave = vec![T::zero(); count];
        // SAFETY: Live immutable owner and disjoint initialized output vectors.
        unsafe {
            assert_eq!((api.model)(owner.raw, copied.as_mut_ptr(), 5), 0);
            compare(&copied, &expected_fields, 4, 0.);
            assert_eq!(
                (api.spectrum)(
                    owner.raw,
                    frequencies.as_ptr(),
                    nf,
                    magnitude.as_mut_ptr(),
                    phase.as_mut_ptr(),
                    nf
                ),
                0
            );
            compare(&magnitude, &em, width, tolerance[0]);
            compare(&phase, &ep, width, tolerance[1]);
            assert_eq!(
                (api.selected)(
                    owner.raw,
                    frequencies.as_ptr(),
                    nf,
                    CIGLET_RS_GLOTTAL_APPROXIMATE,
                    magnitude.as_mut_ptr(),
                    phase.as_mut_ptr(),
                    nf
                ),
                0
            );
            compare(&magnitude, &em, width, tolerance[0]);
            compare(&phase, &ep, width, tolerance[1]);
            assert_eq!(
                (api.one_spectrum)(
                    fields.as_ptr(),
                    5,
                    frequencies.as_ptr(),
                    nf,
                    magnitude.as_mut_ptr(),
                    phase.as_mut_ptr(),
                    nf
                ),
                0
            );
            compare(&magnitude, &em, width, tolerance[0]);
            compare(&phase, &ep, width, tolerance[1]);
            assert_eq!(
                (api.period)(owner.raw, rate, count, wave.as_mut_ptr(), count),
                0
            );
            compare(&wave, &ew, width, tolerance[2]);
            assert_eq!(
                (api.one_period)(fields.as_ptr(), 5, rate, count, wave.as_mut_ptr(), count),
                0
            );
            compare(&wave, &ew, width, tolerance[2]);
        }
        outputs += 2 * nf + count;
    }
    assert_eq!(outputs, 8304);
    assert_eq!(reader.cursor, bytes.len());
}
fn configuration<T: Scalar, H>(api: &Api<T, H>, tolerance: f64) {
    let v = |x| T::from_f64(x).unwrap();
    let mut fields = [T::zero(); 5];
    let frequencies = [v(-400.), v(0.), v(400.)];
    let mut magnitude = [T::zero(); 3];
    let mut phase = [T::zero(); 3];
    // SAFETY: Checked initialized independent fields/planes and unique owners.
    unsafe {
        assert_eq!(
            (api.shape)(v(1.), v(0.005), v(0.5), fields.as_mut_ptr(), 5),
            0
        );
        let mut owner = create(api, &fields);
        let mut clone = Owner {
            raw: ptr::null_mut(),
            api,
        };
        assert_eq!((api.clone)(owner.raw, &mut clone.raw), 0);
        assert_ne!(owner.raw, clone.raw);
        assert_eq!((api.release)(&mut owner.raw), 0);
        assert!(owner.raw.is_null());
        assert_eq!((api.release)(&mut owner.raw), 0);
        for precision in [
            CIGLET_RS_GLOTTAL_FAST,
            CIGLET_RS_GLOTTAL_APPROXIMATE,
            CIGLET_RS_GLOTTAL_EXACT,
        ] {
            assert_eq!(
                (api.selected)(
                    clone.raw,
                    frequencies.as_ptr(),
                    3,
                    precision,
                    magnitude.as_mut_ptr(),
                    phase.as_mut_ptr(),
                    3
                ),
                0
            );
            assert_eq!(magnitude[1], T::zero());
            assert_eq!(phase[1], T::zero());
            if precision == CIGLET_RS_GLOTTAL_EXACT {
                assert!((magnitude[0].into() - magnitude[2].into()).abs() < tolerance);
                assert!((phase[0].into() + phase[2].into()).abs() < tolerance);
            }
        }
        let mut copied = [T::zero(); 5];
        assert_eq!((api.model)(clone.raw, copied.as_mut_ptr(), 5), 0);
        assert_eq!(copied, fields);
        let mut shared = frequencies;
        assert_eq!(
            (api.spectrum)(
                clone.raw,
                shared.as_ptr(),
                3,
                shared.as_mut_ptr(),
                phase.as_mut_ptr(),
                3
            ),
            0
        );
        assert_eq!(
            (api.spectrum)(
                clone.raw,
                ptr::null(),
                0,
                ptr::null_mut(),
                ptr::null_mut(),
                0
            ),
            0
        );
        assert_eq!((api.period)(clone.raw, 16000, 0, ptr::null_mut(), 0), 0);
        assert_eq!(
            (api.one_spectrum)(
                fields.as_ptr(),
                5,
                ptr::null(),
                0,
                ptr::null_mut(),
                ptr::null_mut(),
                0
            ),
            0
        );
        assert_eq!(
            (api.one_period)(fields.as_ptr(), 5, 16000, 0, ptr::null_mut(), 0),
            0
        );
        fields[4] = T::zero();
        let zero = create(api, &fields);
        let mut wave = [v(71.); 16];
        assert_eq!((api.period)(zero.raw, 16000, 16, wave.as_mut_ptr(), 16), 0);
        assert!(wave.iter().all(|&x| x == T::zero()));
    }
}
fn errors<T: Scalar, H>(api: &Api<T, H>, maximum: T) {
    let v = |x| T::from_f64(x).unwrap();
    let fields = [v(0.005), v(0.6), v(0.4), v(0.1), v(-0.5)];
    let owner = create(api, &fields);
    let frequencies = [v(100.), v(f64::NAN)];
    let mut magnitude = [v(71.); 5];
    let mut phase = [v(71.); 5];
    let mut slot = ptr::null_mut();
    // SAFETY: Live owner; invalid pointers/ranges/selectors are rejected before
    // access. All otherwise-used caller arrays are initialized and independent.
    unsafe {
        let overflow = [v(0.), maximum];
        assert_eq!(
            (api.spectrum)(
                owner.raw,
                overflow.as_ptr(),
                2,
                magnitude.as_mut_ptr(),
                phase.as_mut_ptr(),
                2
            ),
            3
        );
        assert_eq!(magnitude, [v(71.); 5]);
        assert_eq!(phase, [v(71.); 5]);
        assert_eq!(
            (api.spectrum)(
                owner.raw,
                frequencies.as_ptr(),
                2,
                magnitude.as_mut_ptr(),
                phase.as_mut_ptr(),
                2
            ),
            3
        );
        assert_eq!(
            (api.selected)(
                owner.raw,
                frequencies.as_ptr(),
                0,
                3,
                magnitude.as_mut_ptr(),
                phase.as_mut_ptr(),
                0
            ),
            3
        );
        assert_eq!(
            (api.spectrum)(
                owner.raw,
                frequencies.as_ptr(),
                2,
                magnitude.as_mut_ptr(),
                magnitude.as_mut_ptr(),
                2
            ),
            2
        );
        assert_eq!(
            (api.spectrum)(
                owner.raw,
                frequencies.as_ptr(),
                2,
                magnitude.as_mut_ptr(),
                magnitude.as_mut_ptr().add(1),
                2
            ),
            2
        );
        let bad = magnitude.as_mut_ptr().cast::<u8>().add(1).cast::<T>();
        assert_eq!(
            (api.spectrum)(
                owner.raw,
                frequencies.as_ptr(),
                2,
                bad,
                phase.as_mut_ptr(),
                2
            ),
            1
        );
        assert_eq!(
            (api.spectrum)(
                owner.raw,
                ptr::null(),
                2,
                magnitude.as_mut_ptr(),
                phase.as_mut_ptr(),
                2
            ),
            1
        );
        assert_eq!(
            (api.spectrum)(
                owner.raw,
                frequencies.as_ptr(),
                2,
                magnitude.as_mut_ptr(),
                phase.as_mut_ptr(),
                1
            ),
            2
        );
        assert_eq!(
            (api.spectrum)(
                ptr::null(),
                frequencies.as_ptr(),
                2,
                magnitude.as_mut_ptr(),
                phase.as_mut_ptr(),
                2
            ),
            1
        );
        assert_eq!(
            (api.spectrum)(
                owner.raw,
                frequencies.as_ptr(),
                usize::MAX,
                magnitude.as_mut_ptr(),
                phase.as_mut_ptr(),
                usize::MAX
            ),
            2
        );
        assert_eq!((api.model)(owner.raw, magnitude.as_mut_ptr(), 4), 2);
        assert_eq!((api.period)(owner.raw, 0, 5, magnitude.as_mut_ptr(), 5), 3);
        assert_eq!(
            (api.period)(owner.raw, 16000, 5, magnitude.as_mut_ptr(), 4),
            2
        );
        assert_eq!(
            (api.shape)(v(0.), v(0.005), v(0.5), magnitude.as_mut_ptr(), 5),
            3
        );
        let mut invalid = fields;
        invalid[0] = v(0.);
        assert_eq!((api.create)(invalid.as_ptr(), 5, &mut slot), 3);
        assert!(slot.is_null());
        assert_eq!((api.create)(fields.as_ptr(), 4, &mut slot), 2);
        assert!(slot.is_null());
        assert_eq!(
            (api.one_spectrum)(
                invalid.as_ptr(),
                5,
                frequencies.as_ptr(),
                2,
                magnitude.as_mut_ptr(),
                phase.as_mut_ptr(),
                2
            ),
            3
        );
        assert_eq!(
            (api.one_period)(invalid.as_ptr(), 5, 16000, 5, magnitude.as_mut_ptr(), 5),
            3
        );
        assert_eq!(magnitude, [v(71.); 5]);
        assert_eq!(phase, [v(71.); 5]);
    }
}

const FLOAT: Api<f32, CigletRsGlottalF32> = Api {
    shape: ciglet_rs_glottal_model_from_shape_f32,
    one_spectrum: ciglet_rs_glottal_model_spectrum_f32,
    one_period: ciglet_rs_glottal_model_sampled_period_f32,
    create: ciglet_rs_glottal_prepared_create_f32,
    clone: ciglet_rs_glottal_prepared_clone_f32,
    release: ciglet_rs_glottal_prepared_release_f32,
    model: ciglet_rs_glottal_prepared_model_f32,
    spectrum: ciglet_rs_glottal_prepared_spectrum_f32,
    selected: ciglet_rs_glottal_prepared_spectrum_with_f32,
    period: ciglet_rs_glottal_prepared_sampled_period_f32,
};
#[test]
fn float_original_reference() {
    reference(
        include_bytes!("../../../tests/fixtures/glottal-float.bin"),
        4,
        [0.; 3],
        &FLOAT,
    );
}
#[test]
fn float_configuration_cloning_and_precision() {
    configuration(&FLOAT, 2e-6);
}
#[test]
fn float_atomic_errors() {
    errors(&FLOAT, f32::MAX);
}

const DOUBLE: Api<f64, CigletRsGlottalF64> = Api {
    shape: ciglet_rs_glottal_model_from_shape_f64,
    one_spectrum: ciglet_rs_glottal_model_spectrum_f64,
    one_period: ciglet_rs_glottal_model_sampled_period_f64,
    create: ciglet_rs_glottal_prepared_create_f64,
    clone: ciglet_rs_glottal_prepared_clone_f64,
    release: ciglet_rs_glottal_prepared_release_f64,
    model: ciglet_rs_glottal_prepared_model_f64,
    spectrum: ciglet_rs_glottal_prepared_spectrum_f64,
    selected: ciglet_rs_glottal_prepared_spectrum_with_f64,
    period: ciglet_rs_glottal_prepared_sampled_period_f64,
};
#[test]
fn double_original_reference() {
    reference(
        include_bytes!("../../../tests/fixtures/glottal-double.bin"),
        8,
        [1e-17, 3e-15, 5e-16],
        &DOUBLE,
    );
}
#[test]
fn double_configuration_cloning_and_precision() {
    configuration(&DOUBLE, 1e-12);
}
#[test]
fn double_atomic_errors() {
    errors(&DOUBLE, f64::MAX);
}

use ciglet_rs::statistics::Scalar;
use ciglet_rs_capi::*;
use std::ptr;

type Interpolate<T> = unsafe extern "C" fn(*const T, usize, usize, *mut T, *mut T) -> u32;

fn reference<T: Scalar>(bytes: &[u8], width: usize, call: Interpolate<T>) {
    assert_eq!(&bytes[..4], b"QIF1");
    assert_eq!(
        u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize,
        width
    );
    let count = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    assert_eq!(count, 64);
    assert_eq!(bytes.len(), 12 + count * 5 * width);
    for record in bytes[12..].chunks_exact(5 * width) {
        let values: Vec<T> = record
            .chunks_exact(width)
            .map(|value| {
                T::from_f64(if width == 4 {
                    f64::from(f32::from_le_bytes(value.try_into().unwrap()))
                } else {
                    f64::from_le_bytes(value.try_into().unwrap())
                })
                .unwrap()
            })
            .collect();
        let mut bin = T::zero();
        let mut magnitude = T::zero();
        // SAFETY: Readable input and independent exclusive typed output slots.
        assert_eq!(
            unsafe { call(values.as_ptr(), 3, 1, &mut bin, &mut magnitude) },
            0
        );
        assert_eq!(bin.into().to_bits(), values[3].into().to_bits());
        assert_eq!(magnitude.into().to_bits(), values[4].into().to_bits());
    }
}

fn analytic<T: Scalar>(call: Interpolate<T>, tolerance: f64) {
    let v = |value| T::from_f64(value).unwrap();
    let mut bin = v(71.);
    let mut magnitude = v(71.);
    let values = [v(f64::NAN), v(0.), v(1.), v(0.5), v(f64::NAN)];
    // SAFETY: Each array remains initialized and readable until compute finishes;
    // output slots are exclusively writable, disjoint and aligned.
    unsafe {
        assert_eq!(call(values.as_ptr(), 5, 2, &mut bin, &mut magnitude), 0);
        assert!((bin.into() - 13. / 6.).abs() < tolerance);
        assert!((magnitude.into() - 49. / 48.).abs() < tolerance);
        let capped = [v(0.), v(10.), v(10.)];
        assert_eq!(call(capped.as_ptr(), 3, 1, &mut bin, &mut magnitude), 0);
        assert_eq!(bin, v(1.5));
        assert_eq!(magnitude, v(10.2));
        let flat = [v(1.); 3];
        assert_eq!(call(flat.as_ptr(), 3, 1, &mut bin, &mut magnitude), 0);
        assert_eq!((bin, magnitude), (v(1.), v(1.)));
        let mut shared = [v(0.), v(1.), v(0.5)];
        let output = shared.as_mut_ptr();
        assert_eq!(call(output, 3, 1, output, output.add(1)), 0);
        assert!((shared[0].into() - 7. / 6.).abs() < tolerance);
        assert!((shared[1].into() - 49. / 48.).abs() < tolerance);
    }
}

fn errors<T: Scalar>(call: Interpolate<T>, maximum: T) {
    let v = |value| T::from_f64(value).unwrap();
    let values = [v(0.), v(1.), v(0.)];
    let mut bin = v(71.);
    let mut magnitude = v(71.);
    // SAFETY: Invalid ranges, alignment, nullness and overlap are rejected before
    // access; otherwise readable inputs and disjoint exclusive scalar slots.
    unsafe {
        for index in [0, 2, usize::MAX] {
            assert_eq!(call(values.as_ptr(), 3, index, &mut bin, &mut magnitude), 3);
            assert_eq!((bin, magnitude), (v(71.), v(71.)));
        }
        for invalid in [[v(0.), v(f64::NAN), v(0.)], [maximum; 3]] {
            assert_eq!(call(invalid.as_ptr(), 3, 1, &mut bin, &mut magnitude), 3);
            assert_eq!((bin, magnitude), (v(71.), v(71.)));
        }
        assert_eq!(call(ptr::null(), 0, 0, &mut bin, &mut magnitude), 3);
        assert_eq!(call(ptr::null(), 3, 1, &mut bin, &mut magnitude), 1);
        assert_eq!(
            call(values.as_ptr(), 3, 1, ptr::null_mut(), &mut magnitude),
            1
        );
        assert_eq!(call(values.as_ptr(), 3, 1, &mut bin, ptr::null_mut()), 1);
        assert_eq!(call(values.as_ptr(), 3, 1, &mut bin, &mut bin), 2);
        let bad = (&mut bin as *mut T).cast::<u8>().add(1).cast::<T>();
        assert_eq!(call(values.as_ptr(), 3, 1, &mut bin, bad), 1);
        assert_eq!(
            call(values.as_ptr(), usize::MAX, 1, &mut bin, &mut magnitude),
            2
        );
        assert_eq!((bin, magnitude), (v(71.), v(71.)));
    }
}

#[test]
fn float_original_reference() {
    reference::<f32>(
        include_bytes!("../../../tests/fixtures/spectral-peak-float.bin"),
        4,
        ciglet_rs_spectral_peak_interpolate_f32,
    );
}
#[test]
fn double_original_reference() {
    reference::<f64>(
        include_bytes!("../../../tests/fixtures/spectral-peak-double.bin"),
        8,
        ciglet_rs_spectral_peak_interpolate_f64,
    );
}
#[test]
fn float_analytic_and_input_overlap() {
    analytic(ciglet_rs_spectral_peak_interpolate_f32, 2e-7);
}
#[test]
fn double_analytic_and_input_overlap() {
    analytic(ciglet_rs_spectral_peak_interpolate_f64, 1e-15);
}
#[test]
fn float_atomic_errors() {
    errors(ciglet_rs_spectral_peak_interpolate_f32, f32::MAX);
}
#[test]
fn double_atomic_errors() {
    errors(ciglet_rs_spectral_peak_interpolate_f64, f64::MAX);
}

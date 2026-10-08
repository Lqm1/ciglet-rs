#![cfg(feature = "c-api")]
use ciglet_rs::{c_api::*, polynomial, statistics::Scalar};
use std::ptr;
type Resonances<T> = unsafe extern "C" fn(
    *const T,
    usize,
    *mut T,
    *mut T,
    *mut T,
    usize,
    *mut usize,
    *mut usize,
    *mut u32,
    *mut f64,
) -> u32;
struct Reader<'a>(&'a [u8], usize);
impl Reader<'_> {
    fn take(&mut self, count: usize) -> &[u8] {
        let (a, b) = self.0.split_at(count);
        self.0 = b;
        a
    }
    fn integer(&mut self) -> usize {
        u32::from_le_bytes(self.take(4).try_into().unwrap()) as usize
    }
    fn scalar<T: Scalar>(&mut self) -> T {
        T::from_f64(if self.1 == 4 {
            f32::from_le_bytes(self.take(4).try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(self.take(8).try_into().unwrap())
        })
        .unwrap()
    }
    fn vector<T: Scalar>(&mut self, count: usize) -> Vec<T> {
        (0..count).map(|_| self.scalar()).collect()
    }
}
fn reference<T: Scalar>(bytes: &[u8], width: usize, call: Resonances<T>) {
    let mut reader = Reader(bytes, width);
    assert_eq!(reader.take(4), b"LPS1");
    assert_eq!(reader.integer(), width);
    assert_eq!(reader.integer(), 48);
    for _ in 0..48 {
        let bins = reader.integer();
        let length = reader.integer();
        let order = reader.integer();
        let _ = reader.vector::<T>(bins + 2 * (order + 1) + 1 + length);
    }
    assert_eq!(reader.integer(), 11);
    let sentinel = T::from_f64(71.).unwrap();
    for case in 0..11 {
        let length = reader.integer();
        let coefficients = reader.vector::<T>(length);
        let expected_count = reader.integer();
        let expected_frequencies = reader.vector::<T>(expected_count);
        let expected_poles: Vec<_> = (0..expected_count)
            .map(|_| (reader.scalar::<T>(), reader.scalar::<T>()))
            .collect();
        let capacity = length;
        let mut frequencies = vec![sentinel; capacity];
        let mut real = vec![sentinel; capacity];
        let mut imaginary = vec![sentinel; capacity];
        let mut count = 71;
        let mut iterations = 71;
        let mut converged = 71;
        let mut residual = 71.;
        // SAFETY: Initialized coefficient input and seven disjoint writable outputs.
        assert_eq!(
            unsafe {
                call(
                    coefficients.as_ptr(),
                    length,
                    frequencies.as_mut_ptr(),
                    real.as_mut_ptr(),
                    imaginary.as_mut_ptr(),
                    capacity,
                    &mut count,
                    &mut iterations,
                    &mut converged,
                    &mut residual,
                )
            },
            0
        );
        assert_eq!(count, expected_count, "case={case}");
        for ((&actual, &expected), (&re, &im)) in frequencies[..count]
            .iter()
            .zip(&expected_frequencies)
            .zip(real[..count].iter().zip(&imaginary[..count]))
        {
            assert_eq!(actual.into().to_bits(), expected.into().to_bits());
            assert!(re.into().is_finite() && im.into().is_finite());
        }
        for ((&re, &im), &(expected_re, expected_im)) in real[..count]
            .iter()
            .zip(&imaginary[..count])
            .zip(&expected_poles)
        {
            assert_eq!(re.into().to_bits(), expected_re.into().to_bits());
            assert_eq!(im.into().to_bits(), expected_im.into().to_bits());
        }
        for plane in [&frequencies, &real, &imaginary] {
            assert!(plane[count..].iter().all(|&x| x == sentinel));
        }
        // Original fixtures contain frequencies/poles; native root diagnostics are
        // independently checked at the existing root API, not inferred from count.
        let roots = polynomial::roots_real(&coefficients).unwrap();
        assert_eq!(iterations, roots.iterations);
        assert_eq!(converged, u32::from(roots.converged));
        assert_eq!(residual.to_bits(), roots.maximum_residual.to_bits());
    }
    assert!(reader.0.is_empty());
}
#[test]
fn float_all_original_frequencies_and_poles() {
    reference(
        include_bytes!("fixtures/prediction-spectrum-float.bin"),
        4,
        ciglet_rs_prediction_resonances_f32,
    );
}
#[test]
fn double_all_original_frequencies_and_poles() {
    reference(
        include_bytes!("fixtures/prediction-spectrum-double.bin"),
        8,
        ciglet_rs_prediction_resonances_f64,
    );
}
fn edges<T: Scalar>(call: Resonances<T>) {
    let one = T::one();
    let zero = T::zero();
    let sentinel = T::from_f64(71.).unwrap();
    let input = [one, zero, one];
    let mut frequencies = [sentinel; 3];
    let mut real = [sentinel; 3];
    let mut imaginary = [sentinel; 3];
    let mut count = 71;
    let mut iterations = 71;
    let mut converged = 71;
    let mut residual = 71.;
    // SAFETY: Properly sized disjoint initialized buffers and scalar outputs;
    // input/output overlap is raw and commits only after input borrowing ends.
    unsafe {
        assert_eq!(
            call(
                input.as_ptr(),
                3,
                frequencies.as_mut_ptr(),
                real.as_mut_ptr(),
                imaginary.as_mut_ptr(),
                3,
                &mut count,
                &mut iterations,
                &mut converged,
                &mut residual
            ),
            0
        );
        assert_eq!(count, 1);
        assert_eq!(converged, 1);
        assert!(iterations > 0 && iterations <= 50 && residual < 1e-6);
        assert!((frequencies[0].into() - 0.5).abs() < 1e-6);
        assert!(real[0].into().abs() < 1e-6);
        assert!((imaginary[0].into() - 1.).abs() < 1e-6);
        for plane in [&frequencies, &real, &imaginary] {
            assert_eq!(&plane[1..], &[sentinel; 2]);
        }
        count = 71;
        iterations = 71;
        converged = 71;
        residual = 71.;
        frequencies.fill(sentinel);
        real.fill(sentinel);
        imaginary.fill(sentinel);
        assert_eq!(
            call(
                input.as_ptr(),
                3,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                &mut count,
                &mut iterations,
                &mut converged,
                &mut residual
            ),
            2
        );
        assert_eq!((count, iterations, converged, residual), (71, 71, 71, 71.));
        let invalid = [zero, one];
        assert_eq!(
            call(
                invalid.as_ptr(),
                2,
                frequencies.as_mut_ptr(),
                real.as_mut_ptr(),
                imaginary.as_mut_ptr(),
                3,
                &mut count,
                &mut iterations,
                &mut converged,
                &mut residual
            ),
            3
        );
        assert_eq!(
            call(
                ptr::null(),
                0,
                frequencies.as_mut_ptr(),
                real.as_mut_ptr(),
                imaginary.as_mut_ptr(),
                3,
                &mut count,
                &mut iterations,
                &mut converged,
                &mut residual
            ),
            3
        );
        assert_eq!((count, iterations, converged, residual), (71, 71, 71, 71.));
        for plane in [&frequencies, &real, &imaginary] {
            assert_eq!(plane, &[sentinel; 3]);
        }
        assert_eq!(
            call(
                &one,
                1,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                &mut count,
                &mut iterations,
                &mut converged,
                &mut residual
            ),
            0
        );
        assert_eq!((count, iterations, converged, residual), (0, 0, 1, 0.));
        let real_roots = [one, zero, T::from_f64(-1.).unwrap()];
        assert_eq!(
            call(
                real_roots.as_ptr(),
                3,
                frequencies.as_mut_ptr(),
                real.as_mut_ptr(),
                imaginary.as_mut_ptr(),
                3,
                &mut count,
                &mut iterations,
                &mut converged,
                &mut residual
            ),
            0
        );
        assert_eq!(count, 0);
        assert_eq!(frequencies, [sentinel; 3]);
        let mut overlap = input;
        assert_eq!(
            call(
                overlap.as_ptr(),
                3,
                overlap.as_mut_ptr(),
                real.as_mut_ptr(),
                imaginary.as_mut_ptr(),
                3,
                &mut count,
                &mut iterations,
                &mut converged,
                &mut residual
            ),
            0
        );
        assert_eq!(count, 1);
        assert!((overlap[0].into() - 0.5).abs() < 1e-6);
        assert_eq!(&overlap[1..], &input[1..]);
    }
}
#[test]
fn float_constants_capacity_errors_and_overlap() {
    edges(ciglet_rs_prediction_resonances_f32);
}
#[test]
fn double_constants_capacity_errors_and_overlap() {
    edges(ciglet_rs_prediction_resonances_f64);
}
fn aliasing<T: Scalar>(call: Resonances<T>) {
    let input = [T::one(), T::zero(), T::one()];
    let mut pairs = 0;
    for first in 0..7 {
        for second in first + 1..7 {
            let mut arena = [71_u64; 32];
            let base = arena.as_mut_ptr().cast::<u8>();
            // SAFETY: Entire aligned initialized arena is 256 bytes. Slots are spaced
            // by sixteen bytes, larger than every output type on required targets.
            let mut outputs: [*mut u8; 7] =
                std::array::from_fn(|index| unsafe { base.add(index * 16) });
            outputs[second] = outputs[first];
            // SAFETY: Rejected raw overlap creates no aliased Rust references. Every
            // pointer retains arena provenance and appropriate alignment and extent.
            assert_eq!(
                unsafe {
                    call(
                        input.as_ptr(),
                        3,
                        outputs[0].cast(),
                        outputs[1].cast(),
                        outputs[2].cast(),
                        1,
                        outputs[3].cast(),
                        outputs[4].cast(),
                        outputs[5].cast(),
                        outputs[6].cast(),
                    )
                },
                2
            );
            assert_eq!(arena, [71; 32]);
            pairs += 1;
        }
    }
    assert_eq!(pairs, 21);
    if size_of::<T>() == 8 {
        let mut arena = [71_u64; 32];
        let base = arena.as_mut_ptr().cast::<u8>();
        // SAFETY: Converged u32 at byte four is aligned and partially intersects
        // the first f64 output, while all other raw output slots are independent.
        assert_eq!(
            unsafe {
                call(
                    input.as_ptr(),
                    3,
                    base.cast(),
                    base.add(16).cast(),
                    base.add(32).cast(),
                    1,
                    base.add(48).cast(),
                    base.add(64).cast(),
                    base.add(4).cast(),
                    base.add(96).cast(),
                )
            },
            2
        );
        assert_eq!(arena, [71; 32]);
    }
}
#[test]
fn every_output_pair_retains_all_storage() {
    aliasing(ciglet_rs_prediction_resonances_f32);
    aliasing(ciglet_rs_prediction_resonances_f64);
}
#[test]
fn nonconvergence_preserves_complete_root_diagnostics() {
    let coefficients = [
        1., -36., 546., -4536., 22449., -67284., 118124., -109584., 40320.,
    ];
    let mut root_real = [71.; 8];
    let mut root_imaginary = [71.; 8];
    let mut root_iterations = 71;
    let mut root_converged = 71;
    let mut root_residual = 71.;
    let mut frequencies = [71.; 8];
    let mut real = [71.; 8];
    let mut imaginary = [71.; 8];
    let mut count = 71;
    let mut iterations = 71;
    let mut converged = 71;
    let mut residual = 71.;
    // SAFETY: Initialized coefficients and independent writable complete arrays
    // and scalars in both interfaces; no output aliases any other output.
    unsafe {
        assert_eq!(
            ciglet_rs_polynomial_roots_real_f64(
                coefficients.as_ptr(),
                9,
                root_real.as_mut_ptr(),
                root_imaginary.as_mut_ptr(),
                8,
                &mut root_iterations,
                &mut root_converged,
                &mut root_residual
            ),
            0
        );
        assert_eq!(
            ciglet_rs_prediction_resonances_f64(
                coefficients.as_ptr(),
                9,
                frequencies.as_mut_ptr(),
                real.as_mut_ptr(),
                imaginary.as_mut_ptr(),
                8,
                &mut count,
                &mut iterations,
                &mut converged,
                &mut residual
            ),
            0
        );
    }
    assert_eq!((iterations, converged), (50, 0));
    assert!(residual > 1e-6);
    assert_eq!(
        (iterations, converged, residual),
        (root_iterations, root_converged, root_residual)
    );
    assert!(
        frequencies[..count]
            .windows(2)
            .all(|pair| pair[0] <= pair[1])
    );
    assert!(
        frequencies[..count]
            .iter()
            .all(|&value| value > 0.001 && value < 0.999)
    );
    for (&re, &im) in real[..count].iter().zip(&imaginary[..count]) {
        assert!(
            root_real
                .iter()
                .zip(&root_imaginary)
                .any(|(&r, &i)| r == re && i == im)
        );
    }
    for plane in [&frequencies, &real, &imaginary] {
        assert!(plane[count..].iter().all(|&value| value == 71.));
    }
}

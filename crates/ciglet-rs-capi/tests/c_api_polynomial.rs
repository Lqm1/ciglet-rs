use ciglet_rs::statistics::Scalar;
use ciglet_rs_capi::*;
type Evaluate<T> = unsafe extern "C" fn(*const T, *const T, usize, T, T, *mut T, *mut T) -> u32;
type EvaluateReal<T> = unsafe extern "C" fn(*const T, usize, T, T, *mut T, *mut T) -> u32;
type Roots<T> = unsafe extern "C" fn(
    *const T,
    *const T,
    usize,
    *mut T,
    *mut T,
    usize,
    *mut usize,
    *mut u32,
    *mut f64,
) -> u32;
type RootsReal<T> = unsafe extern "C" fn(
    *const T,
    usize,
    *mut T,
    *mut T,
    usize,
    *mut usize,
    *mut u32,
    *mut f64,
) -> u32;
struct Api<T> {
    evaluate: Evaluate<T>,
    evaluate_real: EvaluateReal<T>,
    roots: Roots<T>,
    roots_real: RootsReal<T>,
}
struct Reader<'a>(&'a [u8], usize);
impl Reader<'_> {
    fn take(&mut self, n: usize) -> &[u8] {
        let (a, b) = self.0.split_at(n);
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
}
fn reference<T: Scalar>(bytes: &[u8], width: usize, api: Api<T>) {
    let mut r = Reader(bytes, width);
    assert_eq!(r.take(4), b"POL1");
    assert_eq!(r.integer(), width);
    let cases = r.integer();
    assert_eq!(cases, if width == 4 { 32 } else { 33 });
    for _ in 0..cases {
        let n = r.integer();
        let mut real = Vec::new();
        let mut imaginary = Vec::new();
        for _ in 0..n {
            real.push(r.scalar::<T>());
            imaginary.push(r.scalar::<T>());
        }
        for _ in 0..3 {
            let x = r.scalar();
            let y = r.scalar();
            let expected_real = r.scalar();
            let expected_imaginary = r.scalar();
            let mut a = T::zero();
            let mut b = T::zero();
            // SAFETY: Initialized independent coefficient planes and writable scalars.
            unsafe {
                assert_eq!(
                    (api.evaluate)(real.as_ptr(), imaginary.as_ptr(), n, x, y, &mut a, &mut b),
                    0
                );
                assert_eq!(a, expected_real);
                assert_eq!(b, expected_imaginary);
                if imaginary.iter().all(|&v| v == T::zero()) {
                    assert_eq!(
                        (api.evaluate_real)(real.as_ptr(), n, x, y, &mut a, &mut b),
                        0
                    );
                    assert_eq!(a, expected_real);
                    assert_eq!(b, expected_imaginary);
                }
            }
        }
        let degree = n - 1;
        let mut expected_real = Vec::new();
        let mut expected_imaginary = Vec::new();
        for _ in 0..degree {
            expected_real.push(r.scalar::<T>());
            expected_imaginary.push(r.scalar::<T>());
        }
        let mut a = vec![T::zero(); degree];
        let mut b = vec![T::zero(); degree];
        let mut iterations = 71;
        let mut converged = 71;
        let mut residual = 71.0;
        // SAFETY: All result planes and diagnostic scalars are independent storage.
        unsafe {
            assert_eq!(
                (api.roots)(
                    real.as_ptr(),
                    imaginary.as_ptr(),
                    n,
                    a.as_mut_ptr(),
                    b.as_mut_ptr(),
                    degree,
                    &mut iterations,
                    &mut converged,
                    &mut residual
                ),
                0
            );
            assert_eq!(a, expected_real);
            assert_eq!(b, expected_imaginary);
            assert!((1..=50).contains(&iterations));
            assert!(residual.is_finite());
            assert_eq!(converged, u32::from(residual < 1e-6));
            if imaginary.iter().all(|&v| v == T::zero()) {
                let expected_report = (iterations, converged, residual.to_bits());
                assert_eq!(
                    (api.roots_real)(
                        real.as_ptr(),
                        n,
                        a.as_mut_ptr(),
                        b.as_mut_ptr(),
                        degree,
                        &mut iterations,
                        &mut converged,
                        &mut residual
                    ),
                    0
                );
                assert_eq!(a, expected_real);
                assert_eq!(b, expected_imaginary);
                assert_eq!((iterations, converged, residual.to_bits()), expected_report);
            }
        }
    }
    let trig = r.integer();
    for _ in 0..trig {
        let x = r.scalar::<T>().into() as f32;
        let sine = r.scalar::<T>().into();
        let cosine = r.scalar::<T>().into();
        assert_eq!(ciglet_rs::approximation::sine_fast(x).unwrap() as f64, sine);
        assert_eq!(
            ciglet_rs::approximation::cosine_fast(x).unwrap() as f64,
            cosine
        );
    }
    assert!(r.0.is_empty());
}

#[test]
fn all_original_f32_polynomials_match() {
    reference(
        include_bytes!("../../../tests/fixtures/polynomial-float.bin"),
        4,
        Api {
            evaluate: ciglet_rs_polynomial_evaluate_f32,
            evaluate_real: ciglet_rs_polynomial_evaluate_real_f32,
            roots: ciglet_rs_polynomial_roots_f32,
            roots_real: ciglet_rs_polynomial_roots_real_f32,
        },
    );
}

#[test]
fn all_original_f64_polynomials_match() {
    reference(
        include_bytes!("../../../tests/fixtures/polynomial-double.bin"),
        8,
        Api {
            evaluate: ciglet_rs_polynomial_evaluate_f64,
            evaluate_real: ciglet_rs_polynomial_evaluate_real_f64,
            roots: ciglet_rs_polynomial_roots_f64,
            roots_real: ciglet_rs_polynomial_roots_real_f64,
        },
    );
}

#[test]
fn nonconvergence_exposes_complete_independently_checked_report() {
    let coefficients = [
        1.0_f64, -36.0, 546.0, -4536.0, 22449.0, -67284.0, 118124.0, -109584.0, 40320.0,
    ];
    let mut real = [71.0; 8];
    let mut imaginary = [71.0; 8];
    let mut iterations = 71;
    let mut converged = 71;
    let mut residual = 71.0;
    // SAFETY: Independent initialized coefficients, output planes and diagnostics.
    unsafe {
        assert_eq!(
            ciglet_rs_polynomial_roots_real_f64(
                coefficients.as_ptr(),
                9,
                real.as_mut_ptr(),
                imaginary.as_mut_ptr(),
                8,
                &mut iterations,
                &mut converged,
                &mut residual
            ),
            0
        );
    }
    assert_eq!(iterations, 50);
    assert_eq!(converged, 0);
    assert!(residual > 1e-6);
    let independent = real
        .iter()
        .zip(&imaginary)
        .map(|(&r, &i)| {
            let root = ciglet_rs::fourier::Complex::new(r, i);
            let product = (1..=8).fold(ciglet_rs::fourier::Complex::new(1.0, 0.0), |p, k| {
                p * (root - ciglet_rs::fourier::Complex::new(k as f64, 0.0))
            });
            product.re.hypot(product.im)
        })
        .fold(0.0_f64, f64::max);
    assert!((independent - residual).abs() / independent < 1e-8);
}

#[test]
fn constants_missing_complex_planes_and_imaginary_roots() {
    let mut iterations = 71;
    let mut converged = 71;
    let mut residual = 71.0;
    let one = [1.0_f32];
    let mut real = [71.0_f32; 2];
    let mut imaginary = [71.0_f32; 2];
    // SAFETY: Independent valid input/output storage; zero-root arrays accept null.
    unsafe {
        assert_eq!(
            ciglet_rs_polynomial_roots_f32(
                std::ptr::null(),
                one.as_ptr(),
                1,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                0,
                &mut iterations,
                &mut converged,
                &mut residual
            ),
            0
        );
        assert_eq!((iterations, converged, residual), (0, 1, 0.0));
        assert_eq!(
            ciglet_rs_polynomial_roots_real_f32(
                one.as_ptr(),
                1,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                0,
                &mut iterations,
                &mut converged,
                &mut residual
            ),
            0
        );
        assert_eq!((iterations, converged, residual), (0, 1, 0.0));
        let polynomial = [1.0_f32, 0.0, 1.0];
        assert_eq!(
            ciglet_rs_polynomial_roots_real_f32(
                polynomial.as_ptr(),
                3,
                real.as_mut_ptr(),
                imaginary.as_mut_ptr(),
                2,
                &mut iterations,
                &mut converged,
                &mut residual
            ),
            0
        );
        assert_eq!(converged, 1);
        assert!(residual < 1e-6);
        for i in 0..2 {
            assert!(real[i].abs() < 1e-6 && (imaginary[i].abs() - 1.0).abs() < 1e-6);
        }
        let mut a = 71.0_f32;
        let mut b = 71.0_f32;
        assert_eq!(
            ciglet_rs_polynomial_evaluate_f32(
                std::ptr::null(),
                std::ptr::null(),
                2,
                1.0,
                0.0,
                &mut a,
                &mut b
            ),
            0
        );
        assert_eq!((a, b), (0.0, 0.0));
        let linear = [1.0_f32, -1.0];
        assert_eq!(
            ciglet_rs_polynomial_roots_f32(
                linear.as_ptr(),
                std::ptr::null(),
                2,
                real.as_mut_ptr(),
                imaginary.as_mut_ptr(),
                1,
                &mut iterations,
                &mut converged,
                &mut residual
            ),
            0
        );
        assert_eq!((real[0], imaginary[0]), (1.0, 0.0));
    }
}

#[test]
fn every_output_pair_is_disjoint_and_errors_retain_all_storage() {
    let input = [1.0_f64, -1.0];
    for first in 0..5 {
        for second in first + 1..5 {
            let mut arena = [0x4046800000000000_u64; 5];
            let before = arena;
            let base = arena.as_mut_ptr();
            let mut offsets = [0, 1, 2, 3, 4];
            offsets[second] = offsets[first];
            // SAFETY: Each raw output is aligned/in-bounds. Intentional overlapping
            // outputs must be rejected before writes or mutable references occur.
            unsafe {
                assert_eq!(
                    ciglet_rs_polynomial_roots_real_f64(
                        input.as_ptr(),
                        2,
                        base.add(offsets[0]).cast(),
                        base.add(offsets[1]).cast(),
                        1,
                        base.add(offsets[2]).cast(),
                        base.add(offsets[3]).cast(),
                        base.add(offsets[4]).cast()
                    ),
                    2
                );
            }
            assert_eq!(arena, before);
        }
    }
    let mut a = 71.0_f64;
    let mut b = 71.0_f64;
    let mut iterations = 71;
    let mut converged = 71;
    let mut residual = 71.0;
    let invalid = [0.0_f64, 1.0];
    // SAFETY: All valid output scalars are independent; rejected calls cannot write.
    unsafe {
        assert_eq!(
            ciglet_rs_polynomial_roots_real_f64(
                invalid.as_ptr(),
                2,
                &mut a,
                &mut b,
                1,
                &mut iterations,
                &mut converged,
                &mut residual
            ),
            3
        );
        assert_eq!(
            (a, b, iterations, converged, residual),
            (71.0, 71.0, 71, 71, 71.0)
        );
        assert_eq!(
            ciglet_rs_polynomial_roots_real_f64(
                input.as_ptr(),
                2,
                &mut a,
                &mut b,
                0,
                &mut iterations,
                &mut converged,
                &mut residual
            ),
            2
        );
        assert_eq!(
            ciglet_rs_polynomial_evaluate_real_f64(
                input.as_ptr(),
                2,
                f64::INFINITY,
                0.0,
                &mut a,
                &mut b
            ),
            3
        );
        assert_eq!(
            ciglet_rs_polynomial_evaluate_real_f64(std::ptr::null(), 0, 1.0, 0.0, &mut a, &mut b),
            3
        );
        let nan = [f64::NAN, 1.0];
        assert_eq!(
            ciglet_rs_polynomial_roots_real_f64(
                nan.as_ptr(),
                2,
                &mut a,
                &mut b,
                1,
                &mut iterations,
                &mut converged,
                &mut residual
            ),
            3
        );
        assert_eq!(
            (a, b, iterations, converged, residual),
            (71.0, 71.0, 71, 71, 71.0)
        );
        let output = std::ptr::addr_of_mut!(a);
        assert_eq!(
            ciglet_rs_polynomial_evaluate_real_f64(input.as_ptr(), 2, 2.0, 0.0, output, output),
            2
        );
        assert_eq!(a, 71.0);
    }
}

#[test]
fn staged_results_allow_input_output_overlap() {
    let mut coefficients = [1.0_f64, -1.0];
    let pointer = coefficients.as_mut_ptr();
    let mut imaginary = 71.0;
    let mut iterations = 71;
    let mut converged = 71;
    let mut residual = 71.0;
    // SAFETY: Input/output overlap is supported after borrowing ends; outputs
    // are otherwise pairwise disjoint independent scalar storage.
    unsafe {
        assert_eq!(
            ciglet_rs_polynomial_roots_real_f64(
                pointer,
                2,
                pointer,
                &mut imaginary,
                1,
                &mut iterations,
                &mut converged,
                &mut residual
            ),
            0
        );
        assert_eq!(
            (coefficients, imaginary, iterations, converged, residual),
            ([1.0, -1.0], 0.0, 1, 1, 0.0)
        );
        assert_eq!(
            ciglet_rs_polynomial_evaluate_real_f64(pointer, 2, 2.0, 0.0, pointer, pointer.add(1)),
            0
        );
        assert_eq!(coefficients, [1.0, 0.0]);
    }
}

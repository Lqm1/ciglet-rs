use ciglet_rs_capi::*;
#[test]
fn fourier_planes_roundtrip_and_preserve_failures() {
    let mut real = [1.0_f64, 2.0, 3.0, 4.0];
    let mut imaginary = [0.0_f64; 4];
    // SAFETY: Valid independent planes; in-place input/output is supported.
    unsafe {
        assert_eq!(
            ciglet_rs_fft_f64(
                4,
                real.as_ptr(),
                std::ptr::null(),
                0,
                real.as_mut_ptr(),
                imaginary.as_mut_ptr()
            ),
            0
        );
        assert_eq!(real, [10.0, -2.0, -2.0, -2.0]);
        assert_eq!(imaginary, [0.0, 2.0, 0.0, -2.0]);
        assert_eq!(
            ciglet_rs_fft_f64(
                4,
                real.as_ptr(),
                imaginary.as_ptr(),
                1,
                real.as_mut_ptr(),
                imaginary.as_mut_ptr()
            ),
            0
        );
        assert_eq!(real, [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(imaginary, [0.0; 4]);
        assert_eq!(
            ciglet_rs_fft_f64(
                4,
                real.as_ptr(),
                std::ptr::null(),
                2,
                real.as_mut_ptr(),
                imaginary.as_mut_ptr()
            ),
            3
        );
        assert_eq!(
            ciglet_rs_fft_f64(
                4,
                real.as_ptr(),
                std::ptr::null(),
                0,
                real.as_mut_ptr(),
                real.as_mut_ptr()
            ),
            2
        );
        assert_eq!(
            ciglet_rs_fft_f64(
                3,
                real.as_ptr(),
                std::ptr::null(),
                0,
                real.as_mut_ptr(),
                imaginary.as_mut_ptr()
            ),
            3
        );
        assert_eq!(real, [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(imaginary, [0.0; 4]);
    }
}

#[test]
fn direct_inverse_missing_components_and_odd_shift_are_supported() {
    let impulse = [3.0_f32, 0.0, 0.0];
    let mut real = [71.0_f32; 3];
    let mut imaginary = [72.0_f32; 3];
    let mut odd = [1.0_f32, 2.0, 3.0, 4.0, 5.0];
    // SAFETY: Inputs and outputs are initialized, correctly sized allocations.
    unsafe {
        assert_eq!(
            ciglet_rs_inverse_dft_f32(
                3,
                impulse.as_ptr(),
                std::ptr::null(),
                real.as_mut_ptr(),
                imaginary.as_mut_ptr()
            ),
            0
        );
        assert_eq!(real, [1.0; 3]);
        assert_eq!(imaginary, [0.0; 3]);
        assert_eq!(
            ciglet_rs_inverse_dft_f32(
                3,
                std::ptr::null(),
                std::ptr::null(),
                real.as_mut_ptr(),
                std::ptr::null_mut()
            ),
            0
        );
        assert_eq!(real, [0.0; 3]);
        assert_eq!(
            ciglet_rs_fft_shift_f32(odd.as_ptr(), 5, odd.as_mut_ptr(), 5),
            0
        );
        assert_eq!(odd, [4.0, 5.0, 1.0, 2.0, 3.0]);
    }
}

#[test]
fn reusable_plan_serializes_scratch_and_clears_ownership() {
    let mut handle = std::ptr::null_mut();
    let mut length = 0_usize;
    // SAFETY: Valid pointer slots; handle stays live through all scoped threads.
    unsafe {
        assert_eq!(ciglet_rs_fft_plan_new_f64(4, &mut handle), 0);
        assert_eq!(ciglet_rs_fft_plan_length_f64(handle, &mut length), 0);
        assert_eq!(length, 4);
        let original = handle;
        assert_eq!(ciglet_rs_fft_plan_new_f64(3, &mut handle), 3);
        assert_eq!(handle, original);
        let plan = &*handle;
        std::thread::scope(|scope| {
            let workers: Vec<_> = (0..4)
                .map(|_| {
                    scope.spawn(move || {
                        let input = [1.0_f64, 0.0, 0.0, 0.0];
                        let mut output = [71.0_f64; 4];
                        assert_eq!(
                            ciglet_rs_fft_plan_transform_f64(
                                plan,
                                input.as_ptr(),
                                std::ptr::null(),
                                4,
                                0,
                                output.as_mut_ptr(),
                                std::ptr::null_mut()
                            ),
                            0
                        );
                        assert_eq!(output, [1.0; 4]);
                    })
                })
                .collect();
            for worker in workers {
                worker.join().unwrap();
            }
        });
        let mut output = [71.0_f64; 4];
        assert_eq!(
            ciglet_rs_fft_plan_transform_f64(
                handle,
                std::ptr::null(),
                std::ptr::null(),
                3,
                0,
                output.as_mut_ptr(),
                std::ptr::null_mut()
            ),
            2
        );
        assert_eq!(output, [71.0; 4]);
        assert_eq!(ciglet_rs_fft_plan_release_f64(&mut handle), 0);
        assert!(handle.is_null());
        assert_eq!(ciglet_rs_fft_plan_release_f64(&mut handle), 0);
    }
}

#![cfg(feature = "c-api")]

use ciglet_rs::c_api::*;
use std::mem::MaybeUninit;

#[test]
fn scalar_outputs_preserve_ties_and_nan_payloads() {
    assert_eq!(ciglet_rs_abi_version(), 1);
    let mut result = MaybeUninit::<f32>::uninit();
    // SAFETY: result provides exclusive, correctly aligned writable storage.
    unsafe {
        assert_eq!(ciglet_rs_minimum_f32(0.0, -0.0, result.as_mut_ptr()), 0);
        assert_eq!(result.assume_init().to_bits(), (-0.0_f32).to_bits());
        let nan = f32::from_bits(0x7fc1_2345);
        assert_eq!(ciglet_rs_maximum_f32(1.0, nan, result.as_mut_ptr()), 0);
        assert_eq!(result.assume_init().to_bits(), nan.to_bits());
    }
    let mut result = MaybeUninit::<f64>::uninit();
    // SAFETY: result provides exclusive, correctly aligned writable storage.
    unsafe {
        let nan = f64::from_bits(0x7ff8_1234_5678_9abc);
        assert_eq!(ciglet_rs_minimum_f64(1.0, nan, result.as_mut_ptr()), 0);
        assert_eq!(result.assume_init().to_bits(), nan.to_bits());
        assert_eq!(ciglet_rs_maximum_f64(-0.0, 0.0, result.as_mut_ptr()), 0);
        assert_eq!(result.assume_init().to_bits(), 0);
    }
}

#[test]
fn invalid_outputs_are_rejected_without_writes() {
    let mut storage = [0xa5_u8; 24];
    let aligned = storage.as_mut_ptr().cast::<f64>();
    let invalid = if aligned.is_aligned() {
        aligned.wrapping_byte_add(1)
    } else {
        aligned
    };
    // SAFETY: Null and misaligned pointers are explicitly accepted for rejection.
    unsafe {
        assert_eq!(ciglet_rs_minimum_f32(1.0, 2.0, std::ptr::null_mut()), 1);
        assert_eq!(ciglet_rs_maximum_f64(1.0, 2.0, invalid), 1);
    }
    assert_eq!(storage, [0xa5; 24]);
}

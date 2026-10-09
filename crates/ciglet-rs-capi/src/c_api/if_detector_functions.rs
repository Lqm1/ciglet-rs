//! Typed instantaneous-frequency ownership and configuration entry points.
use super::{
    fourier,
    if_detector::{self, CigletRsIfDetectorF32, CigletRsIfDetectorF64},
};
use crate::instantaneous_frequency::Detector;
/// Owned detector operation with checked caller storage and atomic failure behavior.
/// Frequencies are sampling-rate ratios. Native kernels and scalar arithmetic are
/// retained. Explicit estimation policy zero Stable, one Legacy; default Stable.
/// Kernel replacement validates four complete vectors before changing the owner.
/// Invalid replacement preserves configuration. Kernel copies require four disjoint
/// output planes of kernel_count elements; count mismatch is status two. Short
/// samples retain native zero-return/participating-sample validation behavior.
/// Constructors transfer ownership only on success. Kernel copying allocates no
/// temporary vectors after complete validation. Scalar output slots are required
/// even for empty samples. Release accepts an already-null initialized owner slot.
/// Allocation/panic limits apply.
///
/// # Safety
/// Handles are live objects from this precision's constructors. Reads exclude all
/// mutation/release; mutations and release require exclusive owner access. Every
/// caller buffer/slot is independent of private owner storage. Nonempty inputs are
/// aligned initialized stable readable allocations; outputs are aligned exclusively
/// writable for their declared type/count. Copy planes are pairwise disjoint.
/// Null arrays are accepted only for zero counts. Ownership slots are independent
/// initialized writable storage; release invalidates aliases and clears the slot.
/// Constructors/clone output slots must not already own an unreleased object.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_if_detector_create_f32(
    frequency: f32,
    resolution: f32,
    output: *mut *mut CigletRsIfDetectorF32,
) -> u32 {
    // SAFETY: Forward the documented typed owner and caller storage contracts.
    unsafe { if_detector::create(output, || Detector::new(frequency, resolution)) }
}
/// Owned detector operation with checked caller storage and atomic failure behavior.
/// Frequencies are sampling-rate ratios. Native kernels and scalar arithmetic are
/// retained. Explicit estimation policy zero Stable, one Legacy; default Stable.
/// Kernel replacement validates four complete vectors before changing the owner.
/// Invalid replacement preserves configuration. Kernel copies require four disjoint
/// output planes of kernel_count elements; count mismatch is status two. Short
/// samples retain native zero-return/participating-sample validation behavior.
/// Constructors transfer ownership only on success. Kernel copying allocates no
/// temporary vectors after complete validation. Scalar output slots are required
/// even for empty samples. Release accepts an already-null initialized owner slot.
/// Allocation/panic limits apply.
///
/// # Safety
/// Handles are live objects from this precision's constructors. Reads exclude all
/// mutation/release; mutations and release require exclusive owner access. Every
/// caller buffer/slot is independent of private owner storage. Nonempty inputs are
/// aligned initialized stable readable allocations; outputs are aligned exclusively
/// writable for their declared type/count. Copy planes are pairwise disjoint.
/// Null arrays are accepted only for zero counts. Ownership slots are independent
/// initialized writable storage; release invalidates aliases and clears the slot.
/// Constructors/clone output slots must not already own an unreleased object.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_if_detector_from_kernels_f32(
    frequency: f32,
    real: *const f32,
    imaginary: *const f32,
    derivative_real: *const f32,
    derivative_imaginary: *const f32,
    count: usize,
    output: *mut *mut CigletRsIfDetectorF32,
) -> u32 {
    // SAFETY: Forward the documented typed owner and caller storage contracts.
    unsafe {
        if_detector::from_kernels(
            frequency,
            [real, imaginary, derivative_real, derivative_imaginary],
            count,
            output,
        )
    }
}
/// Owned detector operation with checked caller storage and atomic failure behavior.
/// Frequencies are sampling-rate ratios. Native kernels and scalar arithmetic are
/// retained. Explicit estimation policy zero Stable, one Legacy; default Stable.
/// Kernel replacement validates four complete vectors before changing the owner.
/// Invalid replacement preserves configuration. Kernel copies require four disjoint
/// output planes of kernel_count elements; count mismatch is status two. Short
/// samples retain native zero-return/participating-sample validation behavior.
/// Constructors transfer ownership only on success. Kernel copying allocates no
/// temporary vectors after complete validation. Scalar output slots are required
/// even for empty samples. Release accepts an already-null initialized owner slot.
/// Allocation/panic limits apply.
///
/// # Safety
/// Handles are live objects from this precision's constructors. Reads exclude all
/// mutation/release; mutations and release require exclusive owner access. Every
/// caller buffer/slot is independent of private owner storage. Nonempty inputs are
/// aligned initialized stable readable allocations; outputs are aligned exclusively
/// writable for their declared type/count. Copy planes are pairwise disjoint.
/// Null arrays are accepted only for zero counts. Ownership slots are independent
/// initialized writable storage; release invalidates aliases and clears the slot.
/// Constructors/clone output slots must not already own an unreleased object.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_if_detector_clone_f32(
    handle: *const CigletRsIfDetectorF32,
    output: *mut *mut CigletRsIfDetectorF32,
) -> u32 {
    // SAFETY: Forward the documented typed owner and caller storage contracts.
    unsafe { if_detector::clone_owner(handle, output) }
}
/// Owned detector operation with checked caller storage and atomic failure behavior.
/// Frequencies are sampling-rate ratios. Native kernels and scalar arithmetic are
/// retained. Explicit estimation policy zero Stable, one Legacy; default Stable.
/// Kernel replacement validates four complete vectors before changing the owner.
/// Invalid replacement preserves configuration. Kernel copies require four disjoint
/// output planes of kernel_count elements; count mismatch is status two. Short
/// samples retain native zero-return/participating-sample validation behavior.
/// Constructors transfer ownership only on success. Kernel copying allocates no
/// temporary vectors after complete validation. Scalar output slots are required
/// even for empty samples. Release accepts an already-null initialized owner slot.
/// Allocation/panic limits apply.
///
/// # Safety
/// Handles are live objects from this precision's constructors. Reads exclude all
/// mutation/release; mutations and release require exclusive owner access. Every
/// caller buffer/slot is independent of private owner storage. Nonempty inputs are
/// aligned initialized stable readable allocations; outputs are aligned exclusively
/// writable for their declared type/count. Copy planes are pairwise disjoint.
/// Null arrays are accepted only for zero counts. Ownership slots are independent
/// initialized writable storage; release invalidates aliases and clears the slot.
/// Constructors/clone output slots must not already own an unreleased object.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_if_detector_release_f32(
    handle: *mut *mut CigletRsIfDetectorF32,
) -> u32 {
    // SAFETY: Forward the documented typed owner and caller storage contracts.
    unsafe { fourier::release(handle) }
}
/// Owned detector operation with checked caller storage and atomic failure behavior.
/// Frequencies are sampling-rate ratios. Native kernels and scalar arithmetic are
/// retained. Explicit estimation policy zero Stable, one Legacy; default Stable.
/// Kernel replacement validates four complete vectors before changing the owner.
/// Invalid replacement preserves configuration. Kernel copies require four disjoint
/// output planes of kernel_count elements; count mismatch is status two. Short
/// samples retain native zero-return/participating-sample validation behavior.
/// Constructors transfer ownership only on success. Kernel copying allocates no
/// temporary vectors after complete validation. Scalar output slots are required
/// even for empty samples. Release accepts an already-null initialized owner slot.
/// Allocation/panic limits apply.
///
/// # Safety
/// Handles are live objects from this precision's constructors. Reads exclude all
/// mutation/release; mutations and release require exclusive owner access. Every
/// caller buffer/slot is independent of private owner storage. Nonempty inputs are
/// aligned initialized stable readable allocations; outputs are aligned exclusively
/// writable for their declared type/count. Copy planes are pairwise disjoint.
/// Null arrays are accepted only for zero counts. Ownership slots are independent
/// initialized writable storage; release invalidates aliases and clears the slot.
/// Constructors/clone output slots must not already own an unreleased object.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_if_detector_kernel_count_f32(
    handle: *const CigletRsIfDetectorF32,
    output: *mut usize,
) -> u32 {
    // SAFETY: Forward the documented typed owner and caller storage contracts.
    unsafe { if_detector::kernel_count(handle, output) }
}
/// Owned detector operation with checked caller storage and atomic failure behavior.
/// Frequencies are sampling-rate ratios. Native kernels and scalar arithmetic are
/// retained. Explicit estimation policy zero Stable, one Legacy; default Stable.
/// Kernel replacement validates four complete vectors before changing the owner.
/// Invalid replacement preserves configuration. Kernel copies require four disjoint
/// output planes of kernel_count elements; count mismatch is status two. Short
/// samples retain native zero-return/participating-sample validation behavior.
/// Constructors transfer ownership only on success. Kernel copying allocates no
/// temporary vectors after complete validation. Scalar output slots are required
/// even for empty samples. Release accepts an already-null initialized owner slot.
/// Allocation/panic limits apply.
///
/// # Safety
/// Handles are live objects from this precision's constructors. Reads exclude all
/// mutation/release; mutations and release require exclusive owner access. Every
/// caller buffer/slot is independent of private owner storage. Nonempty inputs are
/// aligned initialized stable readable allocations; outputs are aligned exclusively
/// writable for their declared type/count. Copy planes are pairwise disjoint.
/// Null arrays are accepted only for zero counts. Ownership slots are independent
/// initialized writable storage; release invalidates aliases and clears the slot.
/// Constructors/clone output slots must not already own an unreleased object.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_if_detector_center_frequency_f32(
    handle: *const CigletRsIfDetectorF32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward the documented typed owner and caller storage contracts.
    unsafe { if_detector::center(handle, output) }
}
/// Owned detector operation with checked caller storage and atomic failure behavior.
/// Frequencies are sampling-rate ratios. Native kernels and scalar arithmetic are
/// retained. Explicit estimation policy zero Stable, one Legacy; default Stable.
/// Kernel replacement validates four complete vectors before changing the owner.
/// Invalid replacement preserves configuration. Kernel copies require four disjoint
/// output planes of kernel_count elements; count mismatch is status two. Short
/// samples retain native zero-return/participating-sample validation behavior.
/// Constructors transfer ownership only on success. Kernel copying allocates no
/// temporary vectors after complete validation. Scalar output slots are required
/// even for empty samples. Release accepts an already-null initialized owner slot.
/// Allocation/panic limits apply.
///
/// # Safety
/// Handles are live objects from this precision's constructors. Reads exclude all
/// mutation/release; mutations and release require exclusive owner access. Every
/// caller buffer/slot is independent of private owner storage. Nonempty inputs are
/// aligned initialized stable readable allocations; outputs are aligned exclusively
/// writable for their declared type/count. Copy planes are pairwise disjoint.
/// Null arrays are accepted only for zero counts. Ownership slots are independent
/// initialized writable storage; release invalidates aliases and clears the slot.
/// Constructors/clone output slots must not already own an unreleased object.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_if_detector_set_center_frequency_f32(
    handle: *mut CigletRsIfDetectorF32,
    value: f32,
) -> u32 {
    // SAFETY: Forward the documented typed owner and caller storage contracts.
    unsafe { if_detector::set_center(handle, value) }
}
/// Owned detector operation with checked caller storage and atomic failure behavior.
/// Frequencies are sampling-rate ratios. Native kernels and scalar arithmetic are
/// retained. Explicit estimation policy zero Stable, one Legacy; default Stable.
/// Kernel replacement validates four complete vectors before changing the owner.
/// Invalid replacement preserves configuration. Kernel copies require four disjoint
/// output planes of kernel_count elements; count mismatch is status two. Short
/// samples retain native zero-return/participating-sample validation behavior.
/// Constructors transfer ownership only on success. Kernel copying allocates no
/// temporary vectors after complete validation. Scalar output slots are required
/// even for empty samples. Release accepts an already-null initialized owner slot.
/// Allocation/panic limits apply.
///
/// # Safety
/// Handles are live objects from this precision's constructors. Reads exclude all
/// mutation/release; mutations and release require exclusive owner access. Every
/// caller buffer/slot is independent of private owner storage. Nonempty inputs are
/// aligned initialized stable readable allocations; outputs are aligned exclusively
/// writable for their declared type/count. Copy planes are pairwise disjoint.
/// Null arrays are accepted only for zero counts. Ownership slots are independent
/// initialized writable storage; release invalidates aliases and clears the slot.
/// Constructors/clone output slots must not already own an unreleased object.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_if_detector_copy_kernels_f32(
    handle: *const CigletRsIfDetectorF32,
    real: *mut f32,
    imaginary: *mut f32,
    derivative_real: *mut f32,
    derivative_imaginary: *mut f32,
    count: usize,
) -> u32 {
    // SAFETY: Forward the documented typed owner and caller storage contracts.
    unsafe {
        if_detector::copy_kernels(
            handle,
            [real, imaginary, derivative_real, derivative_imaginary],
            count,
        )
    }
}
/// Owned detector operation with checked caller storage and atomic failure behavior.
/// Frequencies are sampling-rate ratios. Native kernels and scalar arithmetic are
/// retained. Explicit estimation policy zero Stable, one Legacy; default Stable.
/// Kernel replacement validates four complete vectors before changing the owner.
/// Invalid replacement preserves configuration. Kernel copies require four disjoint
/// output planes of kernel_count elements; count mismatch is status two. Short
/// samples retain native zero-return/participating-sample validation behavior.
/// Constructors transfer ownership only on success. Kernel copying allocates no
/// temporary vectors after complete validation. Scalar output slots are required
/// even for empty samples. Release accepts an already-null initialized owner slot.
/// Allocation/panic limits apply.
///
/// # Safety
/// Handles are live objects from this precision's constructors. Reads exclude all
/// mutation/release; mutations and release require exclusive owner access. Every
/// caller buffer/slot is independent of private owner storage. Nonempty inputs are
/// aligned initialized stable readable allocations; outputs are aligned exclusively
/// writable for their declared type/count. Copy planes are pairwise disjoint.
/// Null arrays are accepted only for zero counts. Ownership slots are independent
/// initialized writable storage; release invalidates aliases and clears the slot.
/// Constructors/clone output slots must not already own an unreleased object.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_if_detector_replace_kernels_f32(
    handle: *mut CigletRsIfDetectorF32,
    real: *const f32,
    imaginary: *const f32,
    derivative_real: *const f32,
    derivative_imaginary: *const f32,
    count: usize,
) -> u32 {
    // SAFETY: Forward the documented typed owner and caller storage contracts.
    unsafe {
        if_detector::replace(
            handle,
            [real, imaginary, derivative_real, derivative_imaginary],
            count,
        )
    }
}
/// Owned detector operation with checked caller storage and atomic failure behavior.
/// Frequencies are sampling-rate ratios. Native kernels and scalar arithmetic are
/// retained. Explicit estimation policy zero Stable, one Legacy; default Stable.
/// Kernel replacement validates four complete vectors before changing the owner.
/// Invalid replacement preserves configuration. Kernel copies require four disjoint
/// output planes of kernel_count elements; count mismatch is status two. Short
/// samples retain native zero-return/participating-sample validation behavior.
/// Constructors transfer ownership only on success. Kernel copying allocates no
/// temporary vectors after complete validation. Scalar output slots are required
/// even for empty samples. Release accepts an already-null initialized owner slot.
/// Allocation/panic limits apply.
///
/// # Safety
/// Handles are live objects from this precision's constructors. Reads exclude all
/// mutation/release; mutations and release require exclusive owner access. Every
/// caller buffer/slot is independent of private owner storage. Nonempty inputs are
/// aligned initialized stable readable allocations; outputs are aligned exclusively
/// writable for their declared type/count. Copy planes are pairwise disjoint.
/// Null arrays are accepted only for zero counts. Ownership slots are independent
/// initialized writable storage; release invalidates aliases and clears the slot.
/// Constructors/clone output slots must not already own an unreleased object.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_if_detector_estimate_f32(
    handle: *const CigletRsIfDetectorF32,
    samples: *const f32,
    count: usize,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward the documented typed owner and caller storage contracts.
    unsafe { if_detector::estimate(handle, samples, count, None, output) }
}
/// Owned detector operation with checked caller storage and atomic failure behavior.
/// Frequencies are sampling-rate ratios. Native kernels and scalar arithmetic are
/// retained. Explicit estimation policy zero Stable, one Legacy; default Stable.
/// Kernel replacement validates four complete vectors before changing the owner.
/// Invalid replacement preserves configuration. Kernel copies require four disjoint
/// output planes of kernel_count elements; count mismatch is status two. Short
/// samples retain native zero-return/participating-sample validation behavior.
/// Constructors transfer ownership only on success. Kernel copying allocates no
/// temporary vectors after complete validation. Scalar output slots are required
/// even for empty samples. Release accepts an already-null initialized owner slot.
/// Allocation/panic limits apply.
///
/// # Safety
/// Handles are live objects from this precision's constructors. Reads exclude all
/// mutation/release; mutations and release require exclusive owner access. Every
/// caller buffer/slot is independent of private owner storage. Nonempty inputs are
/// aligned initialized stable readable allocations; outputs are aligned exclusively
/// writable for their declared type/count. Copy planes are pairwise disjoint.
/// Null arrays are accepted only for zero counts. Ownership slots are independent
/// initialized writable storage; release invalidates aliases and clears the slot.
/// Constructors/clone output slots must not already own an unreleased object.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_if_detector_estimate_with_f32(
    handle: *const CigletRsIfDetectorF32,
    samples: *const f32,
    count: usize,
    policy: u32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward the documented typed owner and caller storage contracts.
    unsafe { if_detector::estimate(handle, samples, count, Some(policy), output) }
}
/// Owned detector operation with checked caller storage and atomic failure behavior.
/// Frequencies are sampling-rate ratios. Native kernels and scalar arithmetic are
/// retained. Explicit estimation policy zero Stable, one Legacy; default Stable.
/// Kernel replacement validates four complete vectors before changing the owner.
/// Invalid replacement preserves configuration. Kernel copies require four disjoint
/// output planes of kernel_count elements; count mismatch is status two. Short
/// samples retain native zero-return/participating-sample validation behavior.
/// Constructors transfer ownership only on success. Kernel copying allocates no
/// temporary vectors after complete validation. Scalar output slots are required
/// even for empty samples. Release accepts an already-null initialized owner slot.
/// Allocation/panic limits apply.
///
/// # Safety
/// Handles are live objects from this precision's constructors. Reads exclude all
/// mutation/release; mutations and release require exclusive owner access. Every
/// caller buffer/slot is independent of private owner storage. Nonempty inputs are
/// aligned initialized stable readable allocations; outputs are aligned exclusively
/// writable for their declared type/count. Copy planes are pairwise disjoint.
/// Null arrays are accepted only for zero counts. Ownership slots are independent
/// initialized writable storage; release invalidates aliases and clears the slot.
/// Constructors/clone output slots must not already own an unreleased object.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_if_detector_create_f64(
    frequency: f64,
    resolution: f64,
    output: *mut *mut CigletRsIfDetectorF64,
) -> u32 {
    // SAFETY: Forward the documented typed owner and caller storage contracts.
    unsafe { if_detector::create(output, || Detector::new(frequency, resolution)) }
}
/// Owned detector operation with checked caller storage and atomic failure behavior.
/// Frequencies are sampling-rate ratios. Native kernels and scalar arithmetic are
/// retained. Explicit estimation policy zero Stable, one Legacy; default Stable.
/// Kernel replacement validates four complete vectors before changing the owner.
/// Invalid replacement preserves configuration. Kernel copies require four disjoint
/// output planes of kernel_count elements; count mismatch is status two. Short
/// samples retain native zero-return/participating-sample validation behavior.
/// Constructors transfer ownership only on success. Kernel copying allocates no
/// temporary vectors after complete validation. Scalar output slots are required
/// even for empty samples. Release accepts an already-null initialized owner slot.
/// Allocation/panic limits apply.
///
/// # Safety
/// Handles are live objects from this precision's constructors. Reads exclude all
/// mutation/release; mutations and release require exclusive owner access. Every
/// caller buffer/slot is independent of private owner storage. Nonempty inputs are
/// aligned initialized stable readable allocations; outputs are aligned exclusively
/// writable for their declared type/count. Copy planes are pairwise disjoint.
/// Null arrays are accepted only for zero counts. Ownership slots are independent
/// initialized writable storage; release invalidates aliases and clears the slot.
/// Constructors/clone output slots must not already own an unreleased object.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_if_detector_from_kernels_f64(
    frequency: f64,
    real: *const f64,
    imaginary: *const f64,
    derivative_real: *const f64,
    derivative_imaginary: *const f64,
    count: usize,
    output: *mut *mut CigletRsIfDetectorF64,
) -> u32 {
    // SAFETY: Forward the documented typed owner and caller storage contracts.
    unsafe {
        if_detector::from_kernels(
            frequency,
            [real, imaginary, derivative_real, derivative_imaginary],
            count,
            output,
        )
    }
}
/// Owned detector operation with checked caller storage and atomic failure behavior.
/// Frequencies are sampling-rate ratios. Native kernels and scalar arithmetic are
/// retained. Explicit estimation policy zero Stable, one Legacy; default Stable.
/// Kernel replacement validates four complete vectors before changing the owner.
/// Invalid replacement preserves configuration. Kernel copies require four disjoint
/// output planes of kernel_count elements; count mismatch is status two. Short
/// samples retain native zero-return/participating-sample validation behavior.
/// Constructors transfer ownership only on success. Kernel copying allocates no
/// temporary vectors after complete validation. Scalar output slots are required
/// even for empty samples. Release accepts an already-null initialized owner slot.
/// Allocation/panic limits apply.
///
/// # Safety
/// Handles are live objects from this precision's constructors. Reads exclude all
/// mutation/release; mutations and release require exclusive owner access. Every
/// caller buffer/slot is independent of private owner storage. Nonempty inputs are
/// aligned initialized stable readable allocations; outputs are aligned exclusively
/// writable for their declared type/count. Copy planes are pairwise disjoint.
/// Null arrays are accepted only for zero counts. Ownership slots are independent
/// initialized writable storage; release invalidates aliases and clears the slot.
/// Constructors/clone output slots must not already own an unreleased object.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_if_detector_clone_f64(
    handle: *const CigletRsIfDetectorF64,
    output: *mut *mut CigletRsIfDetectorF64,
) -> u32 {
    // SAFETY: Forward the documented typed owner and caller storage contracts.
    unsafe { if_detector::clone_owner(handle, output) }
}
/// Owned detector operation with checked caller storage and atomic failure behavior.
/// Frequencies are sampling-rate ratios. Native kernels and scalar arithmetic are
/// retained. Explicit estimation policy zero Stable, one Legacy; default Stable.
/// Kernel replacement validates four complete vectors before changing the owner.
/// Invalid replacement preserves configuration. Kernel copies require four disjoint
/// output planes of kernel_count elements; count mismatch is status two. Short
/// samples retain native zero-return/participating-sample validation behavior.
/// Constructors transfer ownership only on success. Kernel copying allocates no
/// temporary vectors after complete validation. Scalar output slots are required
/// even for empty samples. Release accepts an already-null initialized owner slot.
/// Allocation/panic limits apply.
///
/// # Safety
/// Handles are live objects from this precision's constructors. Reads exclude all
/// mutation/release; mutations and release require exclusive owner access. Every
/// caller buffer/slot is independent of private owner storage. Nonempty inputs are
/// aligned initialized stable readable allocations; outputs are aligned exclusively
/// writable for their declared type/count. Copy planes are pairwise disjoint.
/// Null arrays are accepted only for zero counts. Ownership slots are independent
/// initialized writable storage; release invalidates aliases and clears the slot.
/// Constructors/clone output slots must not already own an unreleased object.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_if_detector_release_f64(
    handle: *mut *mut CigletRsIfDetectorF64,
) -> u32 {
    // SAFETY: Forward the documented typed owner and caller storage contracts.
    unsafe { fourier::release(handle) }
}
/// Owned detector operation with checked caller storage and atomic failure behavior.
/// Frequencies are sampling-rate ratios. Native kernels and scalar arithmetic are
/// retained. Explicit estimation policy zero Stable, one Legacy; default Stable.
/// Kernel replacement validates four complete vectors before changing the owner.
/// Invalid replacement preserves configuration. Kernel copies require four disjoint
/// output planes of kernel_count elements; count mismatch is status two. Short
/// samples retain native zero-return/participating-sample validation behavior.
/// Constructors transfer ownership only on success. Kernel copying allocates no
/// temporary vectors after complete validation. Scalar output slots are required
/// even for empty samples. Release accepts an already-null initialized owner slot.
/// Allocation/panic limits apply.
///
/// # Safety
/// Handles are live objects from this precision's constructors. Reads exclude all
/// mutation/release; mutations and release require exclusive owner access. Every
/// caller buffer/slot is independent of private owner storage. Nonempty inputs are
/// aligned initialized stable readable allocations; outputs are aligned exclusively
/// writable for their declared type/count. Copy planes are pairwise disjoint.
/// Null arrays are accepted only for zero counts. Ownership slots are independent
/// initialized writable storage; release invalidates aliases and clears the slot.
/// Constructors/clone output slots must not already own an unreleased object.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_if_detector_kernel_count_f64(
    handle: *const CigletRsIfDetectorF64,
    output: *mut usize,
) -> u32 {
    // SAFETY: Forward the documented typed owner and caller storage contracts.
    unsafe { if_detector::kernel_count(handle, output) }
}
/// Owned detector operation with checked caller storage and atomic failure behavior.
/// Frequencies are sampling-rate ratios. Native kernels and scalar arithmetic are
/// retained. Explicit estimation policy zero Stable, one Legacy; default Stable.
/// Kernel replacement validates four complete vectors before changing the owner.
/// Invalid replacement preserves configuration. Kernel copies require four disjoint
/// output planes of kernel_count elements; count mismatch is status two. Short
/// samples retain native zero-return/participating-sample validation behavior.
/// Constructors transfer ownership only on success. Kernel copying allocates no
/// temporary vectors after complete validation. Scalar output slots are required
/// even for empty samples. Release accepts an already-null initialized owner slot.
/// Allocation/panic limits apply.
///
/// # Safety
/// Handles are live objects from this precision's constructors. Reads exclude all
/// mutation/release; mutations and release require exclusive owner access. Every
/// caller buffer/slot is independent of private owner storage. Nonempty inputs are
/// aligned initialized stable readable allocations; outputs are aligned exclusively
/// writable for their declared type/count. Copy planes are pairwise disjoint.
/// Null arrays are accepted only for zero counts. Ownership slots are independent
/// initialized writable storage; release invalidates aliases and clears the slot.
/// Constructors/clone output slots must not already own an unreleased object.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_if_detector_center_frequency_f64(
    handle: *const CigletRsIfDetectorF64,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward the documented typed owner and caller storage contracts.
    unsafe { if_detector::center(handle, output) }
}
/// Owned detector operation with checked caller storage and atomic failure behavior.
/// Frequencies are sampling-rate ratios. Native kernels and scalar arithmetic are
/// retained. Explicit estimation policy zero Stable, one Legacy; default Stable.
/// Kernel replacement validates four complete vectors before changing the owner.
/// Invalid replacement preserves configuration. Kernel copies require four disjoint
/// output planes of kernel_count elements; count mismatch is status two. Short
/// samples retain native zero-return/participating-sample validation behavior.
/// Constructors transfer ownership only on success. Kernel copying allocates no
/// temporary vectors after complete validation. Scalar output slots are required
/// even for empty samples. Release accepts an already-null initialized owner slot.
/// Allocation/panic limits apply.
///
/// # Safety
/// Handles are live objects from this precision's constructors. Reads exclude all
/// mutation/release; mutations and release require exclusive owner access. Every
/// caller buffer/slot is independent of private owner storage. Nonempty inputs are
/// aligned initialized stable readable allocations; outputs are aligned exclusively
/// writable for their declared type/count. Copy planes are pairwise disjoint.
/// Null arrays are accepted only for zero counts. Ownership slots are independent
/// initialized writable storage; release invalidates aliases and clears the slot.
/// Constructors/clone output slots must not already own an unreleased object.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_if_detector_set_center_frequency_f64(
    handle: *mut CigletRsIfDetectorF64,
    value: f64,
) -> u32 {
    // SAFETY: Forward the documented typed owner and caller storage contracts.
    unsafe { if_detector::set_center(handle, value) }
}
/// Owned detector operation with checked caller storage and atomic failure behavior.
/// Frequencies are sampling-rate ratios. Native kernels and scalar arithmetic are
/// retained. Explicit estimation policy zero Stable, one Legacy; default Stable.
/// Kernel replacement validates four complete vectors before changing the owner.
/// Invalid replacement preserves configuration. Kernel copies require four disjoint
/// output planes of kernel_count elements; count mismatch is status two. Short
/// samples retain native zero-return/participating-sample validation behavior.
/// Constructors transfer ownership only on success. Kernel copying allocates no
/// temporary vectors after complete validation. Scalar output slots are required
/// even for empty samples. Release accepts an already-null initialized owner slot.
/// Allocation/panic limits apply.
///
/// # Safety
/// Handles are live objects from this precision's constructors. Reads exclude all
/// mutation/release; mutations and release require exclusive owner access. Every
/// caller buffer/slot is independent of private owner storage. Nonempty inputs are
/// aligned initialized stable readable allocations; outputs are aligned exclusively
/// writable for their declared type/count. Copy planes are pairwise disjoint.
/// Null arrays are accepted only for zero counts. Ownership slots are independent
/// initialized writable storage; release invalidates aliases and clears the slot.
/// Constructors/clone output slots must not already own an unreleased object.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_if_detector_copy_kernels_f64(
    handle: *const CigletRsIfDetectorF64,
    real: *mut f64,
    imaginary: *mut f64,
    derivative_real: *mut f64,
    derivative_imaginary: *mut f64,
    count: usize,
) -> u32 {
    // SAFETY: Forward the documented typed owner and caller storage contracts.
    unsafe {
        if_detector::copy_kernels(
            handle,
            [real, imaginary, derivative_real, derivative_imaginary],
            count,
        )
    }
}
/// Owned detector operation with checked caller storage and atomic failure behavior.
/// Frequencies are sampling-rate ratios. Native kernels and scalar arithmetic are
/// retained. Explicit estimation policy zero Stable, one Legacy; default Stable.
/// Kernel replacement validates four complete vectors before changing the owner.
/// Invalid replacement preserves configuration. Kernel copies require four disjoint
/// output planes of kernel_count elements; count mismatch is status two. Short
/// samples retain native zero-return/participating-sample validation behavior.
/// Constructors transfer ownership only on success. Kernel copying allocates no
/// temporary vectors after complete validation. Scalar output slots are required
/// even for empty samples. Release accepts an already-null initialized owner slot.
/// Allocation/panic limits apply.
///
/// # Safety
/// Handles are live objects from this precision's constructors. Reads exclude all
/// mutation/release; mutations and release require exclusive owner access. Every
/// caller buffer/slot is independent of private owner storage. Nonempty inputs are
/// aligned initialized stable readable allocations; outputs are aligned exclusively
/// writable for their declared type/count. Copy planes are pairwise disjoint.
/// Null arrays are accepted only for zero counts. Ownership slots are independent
/// initialized writable storage; release invalidates aliases and clears the slot.
/// Constructors/clone output slots must not already own an unreleased object.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_if_detector_replace_kernels_f64(
    handle: *mut CigletRsIfDetectorF64,
    real: *const f64,
    imaginary: *const f64,
    derivative_real: *const f64,
    derivative_imaginary: *const f64,
    count: usize,
) -> u32 {
    // SAFETY: Forward the documented typed owner and caller storage contracts.
    unsafe {
        if_detector::replace(
            handle,
            [real, imaginary, derivative_real, derivative_imaginary],
            count,
        )
    }
}
/// Owned detector operation with checked caller storage and atomic failure behavior.
/// Frequencies are sampling-rate ratios. Native kernels and scalar arithmetic are
/// retained. Explicit estimation policy zero Stable, one Legacy; default Stable.
/// Kernel replacement validates four complete vectors before changing the owner.
/// Invalid replacement preserves configuration. Kernel copies require four disjoint
/// output planes of kernel_count elements; count mismatch is status two. Short
/// samples retain native zero-return/participating-sample validation behavior.
/// Constructors transfer ownership only on success. Kernel copying allocates no
/// temporary vectors after complete validation. Scalar output slots are required
/// even for empty samples. Release accepts an already-null initialized owner slot.
/// Allocation/panic limits apply.
///
/// # Safety
/// Handles are live objects from this precision's constructors. Reads exclude all
/// mutation/release; mutations and release require exclusive owner access. Every
/// caller buffer/slot is independent of private owner storage. Nonempty inputs are
/// aligned initialized stable readable allocations; outputs are aligned exclusively
/// writable for their declared type/count. Copy planes are pairwise disjoint.
/// Null arrays are accepted only for zero counts. Ownership slots are independent
/// initialized writable storage; release invalidates aliases and clears the slot.
/// Constructors/clone output slots must not already own an unreleased object.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_if_detector_estimate_f64(
    handle: *const CigletRsIfDetectorF64,
    samples: *const f64,
    count: usize,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward the documented typed owner and caller storage contracts.
    unsafe { if_detector::estimate(handle, samples, count, None, output) }
}
/// Owned detector operation with checked caller storage and atomic failure behavior.
/// Frequencies are sampling-rate ratios. Native kernels and scalar arithmetic are
/// retained. Explicit estimation policy zero Stable, one Legacy; default Stable.
/// Kernel replacement validates four complete vectors before changing the owner.
/// Invalid replacement preserves configuration. Kernel copies require four disjoint
/// output planes of kernel_count elements; count mismatch is status two. Short
/// samples retain native zero-return/participating-sample validation behavior.
/// Constructors transfer ownership only on success. Kernel copying allocates no
/// temporary vectors after complete validation. Scalar output slots are required
/// even for empty samples. Release accepts an already-null initialized owner slot.
/// Allocation/panic limits apply.
///
/// # Safety
/// Handles are live objects from this precision's constructors. Reads exclude all
/// mutation/release; mutations and release require exclusive owner access. Every
/// caller buffer/slot is independent of private owner storage. Nonempty inputs are
/// aligned initialized stable readable allocations; outputs are aligned exclusively
/// writable for their declared type/count. Copy planes are pairwise disjoint.
/// Null arrays are accepted only for zero counts. Ownership slots are independent
/// initialized writable storage; release invalidates aliases and clears the slot.
/// Constructors/clone output slots must not already own an unreleased object.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_if_detector_estimate_with_f64(
    handle: *const CigletRsIfDetectorF64,
    samples: *const f64,
    count: usize,
    policy: u32,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward the documented typed owner and caller storage contracts.
    unsafe { if_detector::estimate(handle, samples, count, Some(policy), output) }
}

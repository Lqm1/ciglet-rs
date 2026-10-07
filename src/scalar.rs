//! Scalar comparisons retaining ciglet's second-operand tie behavior.

/// Return the first value only when it is strictly smaller than the second.
/// Ties and unordered comparisons retain the second operand, including NaNs.
pub fn minimum<T: PartialOrd>(first: T, second: T) -> T {
    if first < second { first } else { second }
}
/// Return the first value only when it is strictly greater than the second.
/// Ties and unordered comparisons retain the second operand, including NaNs.
pub fn maximum<T: PartialOrd>(first: T, second: T) -> T {
    if first > second { first } else { second }
}
/// Original M_EPS constant, distinct from IEEE machine epsilon.
pub const LEGACY_EPSILON: f64 = 1e-15;
/// Original M_INF is a finite sentinel, not IEEE infinity.
pub const LEGACY_INFINITY_SENTINEL: f64 = 1e15;

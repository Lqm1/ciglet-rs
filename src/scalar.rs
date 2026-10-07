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

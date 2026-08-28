//! Assertion helpers for contract tests.
//!
//! This module starts deliberately small — one well-tested helper. Additional
//! assertions (gas/budget ceilings, event emission checks, ...) are tracked as
//! individual GitHub issues so each lands in its own file without merge
//! conflicts.

/// Returns `true` when `a` and `b` differ by no more than `tolerance`.
///
/// Useful for liquidity / fee math where rounding makes exact equality
/// impractical. `tolerance` is an absolute value in the same units as the
/// inputs and must be non-negative.
///
/// ```
/// use soroban_test_kit::asserts::approx_eq;
/// assert!(approx_eq(1_000, 1_003, 5));
/// assert!(!approx_eq(1_000, 1_010, 5));
/// ```
pub fn approx_eq(a: i128, b: i128, tolerance: i128) -> bool {
    let diff = if a >= b { a - b } else { b - a };
    diff <= tolerance
}

/// Asserts that two `i128` values are within `tolerance` of each other,
/// panicking with a descriptive message otherwise.
///
/// ```
/// use soroban_test_kit::assert_approx_eq;
/// assert_approx_eq!(1_000, 1_003, 5);
/// ```
#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr, $tol:expr $(,)?) => {{
        let a = $a;
        let b = $b;
        let tol = $tol;
        if !$crate::asserts::approx_eq(a, b, tol) {
            panic!(
                "assertion failed: `{} ~= {}` (tolerance {}); difference is {}",
                a,
                b,
                tol,
                if a >= b { a - b } else { b - a }
            );
        }
    }};
}

#[cfg(test)]
mod test {
    use super::approx_eq;

    #[test]
    fn within_tolerance() {
        assert!(approx_eq(100, 100, 0));
        assert!(approx_eq(100, 95, 5));
        assert!(approx_eq(95, 100, 5));
    }

    #[test]
    fn outside_tolerance() {
        assert!(!approx_eq(100, 94, 5));
        assert!(!approx_eq(94, 100, 5));
    }

    #[test]
    fn macro_passes_when_close() {
        assert_approx_eq!(1_000, 1_002, 5);
    }

    #[test]
    #[should_panic(expected = "tolerance")]
    fn macro_panics_when_far() {
        assert_approx_eq!(1_000, 2_000, 5);
    }
}
pub mod addr_eq;
pub mod address_set;
pub mod approx_i64;
pub mod approx_u128;
pub mod auth;
pub mod balances;
pub mod map_contains;
pub mod panics;
pub mod sorted;
pub mod vec_unordered;
pub mod within_pct;
pub mod zeroness;

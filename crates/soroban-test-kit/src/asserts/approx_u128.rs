//! Unsigned-width approximate equality, mirroring
//! [`approx_eq`](crate::asserts::approx_eq) for `u128`.
//!
//! Avoids casting `u128` test values down to `i128`. `tolerance` is an absolute
//! value in the same units and the subtraction is saturating, so it never
//! underflows.
//!
//! ```
//! use soroban_test_kit::asserts::approx_u128::approx_eq_u128;
//! assert!(approx_eq_u128(1_000, 1_003, 5));
//! assert!(!approx_eq_u128(1_000, 1_010, 5));
//! ```

/// Returns `true` when `a` and `b` differ by no more than `tolerance` (`u128`).
///
/// Uses saturating subtraction so the larger-minus-smaller difference is taken
/// without risk of underflow.
pub fn approx_eq_u128(a: u128, b: u128, tolerance: u128) -> bool {
    a.abs_diff(b) <= tolerance
}

/// Asserts two `u128` values are within `tolerance`, panicking otherwise.
///
/// ```
/// use soroban_test_kit::assert_approx_eq_u128;
/// assert_approx_eq_u128!(1_000u128, 1_003u128, 5u128);
/// ```
#[macro_export]
macro_rules! assert_approx_eq_u128 {
    ($a:expr, $b:expr, $tol:expr $(,)?) => {{
        let a: u128 = $a;
        let b: u128 = $b;
        let tol: u128 = $tol;
        if !$crate::asserts::approx_u128::approx_eq_u128(a, b, tol) {
            panic!(
                "assertion failed: `{} ~= {}` (tolerance {}); difference is {}",
                a,
                b,
                tol,
                a.abs_diff(b)
            );
        }
    }};
}

#[cfg(test)]
mod test {
    use super::approx_eq_u128;

    #[test]
    fn within_tolerance() {
        assert!(approx_eq_u128(100, 100, 0));
        assert!(approx_eq_u128(100, 95, 5));
        assert!(approx_eq_u128(95, 100, 5));
    }

    #[test]
    fn outside_tolerance() {
        assert!(!approx_eq_u128(100, 94, 5));
        assert!(!approx_eq_u128(94, 100, 5));
    }

    #[test]
    fn saturating_subtraction_edge_case() {
        // No underflow when the second arg is larger.
        assert!(approx_eq_u128(0, 5, 5));
        assert!(!approx_eq_u128(0, u128::MAX, 1));
    }

    #[test]
    fn macro_passes_when_close() {
        assert_approx_eq_u128!(1_000u128, 1_002u128, 5u128);
    }

    #[test]
    #[should_panic(expected = "tolerance")]
    fn macro_panics_when_far() {
        assert_approx_eq_u128!(1_000u128, 2_000u128, 5u128);
    }
}

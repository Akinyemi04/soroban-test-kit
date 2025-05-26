//! Narrow-width approximate equality for `i64`, mirroring
//! [`approx_eq`](crate::asserts::approx_eq).
//!
//! Ledger timestamps and sequence numbers are `u64`/`i64`; this avoids noisy
//! casts in timing tests. `tolerance` is an absolute value in the same units
//! and must be non-negative.
//!
//! ```
//! use soroban_test_kit::asserts::approx_i64::approx_eq_i64;
//! assert!(approx_eq_i64(1_000, 1_003, 5));
//! assert!(!approx_eq_i64(1_000, 1_010, 5));
//! ```

/// Returns `true` when `a` and `b` differ by no more than `tolerance` (`i64`).
pub fn approx_eq_i64(a: i64, b: i64, tolerance: i64) -> bool {
    let diff = if a >= b { a - b } else { b - a };
    diff <= tolerance
}

/// Asserts two `i64` values are within `tolerance`, panicking otherwise.
///
/// ```
/// use soroban_test_kit::assert_approx_eq_i64;
/// assert_approx_eq_i64!(1_000i64, 1_003i64, 5i64);
/// ```
#[macro_export]
macro_rules! assert_approx_eq_i64 {
    ($a:expr, $b:expr, $tol:expr $(,)?) => {{
        let a: i64 = $a;
        let b: i64 = $b;
        let tol: i64 = $tol;
        if !$crate::asserts::approx_i64::approx_eq_i64(a, b, tol) {
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
    use super::approx_eq_i64;

    #[test]
    fn within_tolerance() {
        assert!(approx_eq_i64(100, 100, 0));
        assert!(approx_eq_i64(100, 95, 5));
        assert!(approx_eq_i64(95, 100, 5));
    }

    #[test]
    fn outside_tolerance() {
        assert!(!approx_eq_i64(100, 94, 5));
        assert!(!approx_eq_i64(94, 100, 5));
    }

    #[test]
    fn macro_passes_when_close() {
        assert_approx_eq_i64!(1_000i64, 1_002i64, 5i64);
    }

    #[test]
    #[should_panic(expected = "tolerance")]
    fn macro_panics_when_far() {
        assert_approx_eq_i64!(1_000i64, 2_000i64, 5i64);
    }
}

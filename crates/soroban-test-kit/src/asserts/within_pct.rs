//! Percentage (basis-point) tolerance assertion.
//!
//! Relative tolerance ("within 0.5%") is often more meaningful than an absolute
//! tolerance for fee/slippage math. Tolerance is expressed in **basis points**
//! (1 bps = 0.01%) to keep the math integer-only — no floats.
//!
//! ## Integer-only math
//! `actual` is within tolerance of `expected` when
//! `|actual - expected| * 10_000 <= |expected| * bps`. Multiplying out avoids
//! division and float rounding. When `expected == 0`, only `actual == 0` is
//! within tolerance (any non-zero deviation is infinite percent).
//!
//! ```
//! use soroban_test_kit::asserts::within_pct::within_pct;
//! assert!(within_pct(1_000, 1_005, 50));  // within 0.5%
//! assert!(!within_pct(1_000, 1_006, 50)); // outside 0.5%
//! ```

/// Denominator for basis-point math (100% == 10_000 bps).
pub const BPS_DENOMINATOR: i128 = 10_000;

/// Returns `true` when `actual` is within `bps` basis points of `expected`,
/// using integer cross-multiplication (no floats).
pub fn within_pct(actual: i128, expected: i128, bps: i128) -> bool {
    let diff = if actual >= expected {
        actual - expected
    } else {
        expected - actual
    };
    let abs_expected = if expected >= 0 { expected } else { -expected };
    diff * BPS_DENOMINATOR <= abs_expected * bps
}

/// Asserts `actual` is within `bps` basis points of `expected`, panicking
/// otherwise.
///
/// ```
/// use soroban_test_kit::assert_within_pct;
/// assert_within_pct!(1_000i128, 1_005i128, 50i128);
/// ```
#[macro_export]
macro_rules! assert_within_pct {
    ($actual:expr, $expected:expr, $bps:expr $(,)?) => {{
        let actual: i128 = $actual;
        let expected: i128 = $expected;
        let bps: i128 = $bps;
        if !$crate::asserts::within_pct::within_pct(actual, expected, bps) {
            panic!(
                "assertion failed: {} not within {} bps of {}",
                actual, bps, expected
            );
        }
    }};
}

#[cfg(test)]
mod test {
    use super::within_pct;

    #[test]
    fn within_boundary() {
        // Tolerance is relative to `expected`: 0.5% of 1_000 is 5.
        assert!(within_pct(1_005, 1_000, 50));
        assert!(within_pct(995, 1_000, 50));
        assert!(within_pct(1_000, 1_000, 0));
    }

    #[test]
    fn outside_boundary() {
        assert!(!within_pct(1_000, 1_006, 50));
        assert!(!within_pct(1_000, 994, 50));
    }

    #[test]
    fn zero_expected_only_matches_zero() {
        assert!(within_pct(0, 0, 100));
        assert!(!within_pct(1, 0, 100));
    }

    #[test]
    fn macro_passes() {
        assert_within_pct!(1_000i128, 1_005i128, 50i128);
    }

    #[test]
    #[should_panic(expected = "not within")]
    fn macro_panics() {
        assert_within_pct!(1_000i128, 1_100i128, 50i128);
    }
}

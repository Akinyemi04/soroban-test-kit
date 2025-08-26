//! Zero / non-zero assertions for `i128`.
//!
//! Tiny readability helpers for the very common "balance must be (non)zero"
//! check.
//!
//! ```
//! use soroban_test_kit::{assert_zero, assert_nonzero};
//! assert_zero!(0i128);
//! assert_nonzero!(5i128);
//! ```

/// Asserts an `i128` value is exactly zero.
#[macro_export]
macro_rules! assert_zero {
    ($v:expr $(,)?) => {{
        let v: i128 = $v;
        if v != 0 {
            panic!("assertion failed: expected zero, got {}", v);
        }
    }};
}

/// Asserts an `i128` value is non-zero.
#[macro_export]
macro_rules! assert_nonzero {
    ($v:expr $(,)?) => {{
        let v: i128 = $v;
        if v == 0 {
            panic!("assertion failed: expected non-zero, got 0");
        }
    }};
}

#[cfg(test)]
mod test {
    #[test]
    fn zero_passes() {
        assert_zero!(0i128);
    }

    #[test]
    fn nonzero_passes() {
        assert_nonzero!(1i128);
        assert_nonzero!(-1i128);
    }

    #[test]
    #[should_panic(expected = "expected zero")]
    fn nonzero_fails_zero_assert() {
        assert_zero!(7i128);
    }

    #[test]
    #[should_panic(expected = "expected non-zero")]
    fn zero_fails_nonzero_assert() {
        assert_nonzero!(0i128);
    }
}

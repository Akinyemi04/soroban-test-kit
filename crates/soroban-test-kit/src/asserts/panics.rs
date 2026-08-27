//! A generic "this closure panics" assertion.
//!
//! Friendlier than scattering `#[should_panic]` attributes: `assert_panics`
//! runs a closure and asserts it unwound via a panic, so it can be used inside
//! a larger test body and combined with other checks.
//!
//! ## `no_std` / panic-strategy note
//! This relies on [`std::panic::catch_unwind`], so it is only available when
//! `std` is present. The crate is `#![no_std]`, but the test harness links
//! `std`, so this helper is gated behind the opt-in `testutils` feature and
//! pulls in `std` explicitly. It requires the `unwind` panic strategy; under
//! `panic = "abort"` a panic terminates the process and cannot be caught.
//!
//! ```ignore
//! use soroban_test_kit::asserts::panics::assert_panics;
//! assert_panics(|| panic!("boom"));
//! ```

#[cfg(feature = "testutils")]
extern crate std;

/// Runs `f` and asserts that it panics. Panics itself (with a descriptive
/// message) if `f` returns normally.
///
/// Available only with the `testutils` feature and the `unwind` panic strategy.
#[cfg(feature = "testutils")]
pub fn assert_panics<F: FnOnce()>(f: F) {
    let prev = std::panic::take_hook();
    std::panic::set_hook(std::boxed::Box::new(|_| {}));
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
    std::panic::set_hook(prev);
    if result.is_ok() {
        panic!("assertion failed: expected closure to panic, but it returned normally");
    }
}

#[cfg(all(test, feature = "testutils"))]
mod test {
    use super::assert_panics;

    #[test]
    fn detects_a_panic() {
        assert_panics(|| panic!("boom"));
    }

    #[test]
    fn detects_an_assert_failure() {
        assert_panics(|| panic!("nope"));
    }

    #[test]
    #[should_panic(expected = "expected closure to panic")]
    fn fails_when_no_panic() {
        assert_panics(|| {
            let _ = 1 + 1;
        });
    }
}

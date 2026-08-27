//! An authorization-failure assertion.
//!
//! The most common thing a Soroban test needs to verify: that a privileged
//! call rejects a caller who hasn't authorized it. `assert_auth_required`
//! runs a closure in an `Env` with no matching `mock_auths`/`mock_all_auths`
//! and asserts the call panics because a `require_auth()` check failed.
//!
//! Built on [`assert_panics`](crate::asserts::panics::assert_panics), so it
//! shares its `testutils` feature gate and `std::panic::catch_unwind`
//! requirement (`unwind` panic strategy).
//!
//! ```ignore
//! use soroban_test_kit::asserts::auth::assert_auth_required;
//! use soroban_test_kit::prelude::*;
//!
//! let env = Env::default();
//! let admin = Address::generate(&env);
//! let id = env.register_contract(None, MockToken);
//! let token = MockTokenClient::new(&env, &id);
//! token.init(&admin);
//!
//! // No `env.mock_all_auths()` call, so `admin.require_auth()` inside
//! // `mint` has nothing to satisfy it.
//! let alice = Address::generate(&env);
//! assert_auth_required(|| token.mint(&alice, &100));
//! ```

#[cfg(feature = "testutils")]
extern crate std;

/// Asserts that `f` panics due to a missing authorization, i.e. it must be
/// run in an `Env` where the address that calls `require_auth()` has not
/// been authorized via `mock_all_auths()` or a matching `mock_auths` entry.
///
/// Panics itself (with a message noting an auth failure was expected) if `f`
/// returns normally — for example because the call was mistakenly run under
/// `mock_all_auths()`.
#[cfg(feature = "testutils")]
pub fn assert_auth_required<F: FnOnce()>(f: F) {
    let prev = std::panic::take_hook();
    std::panic::set_hook(std::boxed::Box::new(|_| {}));
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
    std::panic::set_hook(prev);
    if result.is_ok() {
        panic!(
            "assertion failed: expected an authorization failure, but the call succeeded \
             (is the caller authorized via mock_all_auths() or mock_auths()?)"
        );
    }
}

#[cfg(all(test, feature = "testutils"))]
mod test {
    use super::assert_auth_required;
    use crate::mocks::token::{MockToken, MockTokenClient};
    use soroban_sdk::{testutils::Address as _, Address, Env};

    #[test]
    fn rejects_call_with_no_authorization() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let id = env.register_contract(None, MockToken);
        let token = MockTokenClient::new(&env, &id);

        // `init` itself needs no authorization, but `mint` requires the
        // admin's, and no `env.mock_all_auths()` was called.
        token.init(&admin);
        let alice = Address::generate(&env);
        assert_auth_required(|| token.mint(&alice, &100));
    }

    #[test]
    #[should_panic(expected = "expected an authorization failure")]
    fn fails_when_authorization_is_present() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let id = env.register_contract(None, MockToken);
        let token = MockTokenClient::new(&env, &id);
        token.init(&admin);

        let alice = Address::generate(&env);
        assert_auth_required(|| token.mint(&alice, &100));
    }
}

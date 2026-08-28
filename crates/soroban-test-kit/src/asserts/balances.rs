//! A balance-delta assertion.
//!
//! The recurring pattern in a contract test is "this call changed Alice's
//! balance by exactly N" — snapshot before, run the action, snapshot after,
//! compare the delta. `assert_balance_change` packages that up so the test
//! body reads like the assertion it's making.
//!
//! Works against any token client whose `balance` method takes `&Address`
//! and returns `i128` (every mock token in this crate, and the real Stellar
//! Asset Contract client, all qualify) by taking a small getter closure
//! instead of requiring a shared trait.
//!
//! ```ignore
//! use soroban_test_kit::asserts::balances::assert_balance_change;
//! use soroban_test_kit::prelude::*;
//!
//! let env = Env::default();
//! env.mock_all_auths();
//! let admin = Address::generate(&env);
//! let id = env.register_contract(None, MockToken);
//! let token = MockTokenClient::new(&env, &id);
//! token.init(&admin);
//!
//! let alice = Address::generate(&env);
//! assert_balance_change(|who| token.balance(who), &alice, 1_000, || {
//!     token.mint(&alice, &1_000);
//! });
//! ```

use soroban_sdk::Address;

/// Runs `f` and asserts that `get_balance(who)` changed by exactly
/// `expected_delta` (`after - before`). `expected_delta` may be negative for
/// an expected decrease.
///
/// Panics with a message reporting the before/after/expected values if the
/// observed delta doesn't match.
pub fn assert_balance_change<G, F>(get_balance: G, who: &Address, expected_delta: i128, f: F)
where
    G: Fn(&Address) -> i128,
    F: FnOnce(),
{
    let before = get_balance(who);
    f();
    let after = get_balance(who);
    let actual_delta = after - before;
    if actual_delta != expected_delta {
        panic!(
            "assertion failed: expected balance to change by {}, but it changed by {} (before {}, after {})",
            expected_delta, actual_delta, before, after
        );
    }
}

#[cfg(test)]
mod test {
    use super::assert_balance_change;
    use crate::mocks::token::{MockToken, MockTokenClient};
    use soroban_sdk::{testutils::Address as _, Address, Env};

    #[test]
    fn passes_for_the_exact_change() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let id = env.register_contract(None, MockToken);
        let token = MockTokenClient::new(&env, &id);
        token.init(&admin);

        let alice = Address::generate(&env);
        assert_balance_change(
            |who| token.balance(who),
            &alice,
            1_000,
            || {
                token.mint(&alice, &1_000);
            },
        );
    }

    #[test]
    fn detects_a_decrease() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let id = env.register_contract(None, MockToken);
        let token = MockTokenClient::new(&env, &id);
        token.init(&admin);

        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        token.mint(&alice, &1_000);

        assert_balance_change(
            |who| token.balance(who),
            &alice,
            -400,
            || {
                token.transfer(&alice, &bob, &400);
            },
        );
    }

    #[test]
    #[should_panic(expected = "expected balance to change by 1000, but it changed by 500")]
    fn fails_when_delta_does_not_match() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let id = env.register_contract(None, MockToken);
        let token = MockTokenClient::new(&env, &id);
        token.init(&admin);

        let alice = Address::generate(&env);
        assert_balance_change(
            |who| token.balance(who),
            &alice,
            1_000,
            || {
                token.mint(&alice, &500);
            },
        );
    }
}

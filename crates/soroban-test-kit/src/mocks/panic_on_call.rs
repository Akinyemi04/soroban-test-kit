//! A contract mock whose method always panics.
//!
//! Use this as a failing dependency to verify how a caller handles a downstream
//! abort (e.g. that an error propagates instead of being silently swallowed).
//!
//! ## Panic message
//! `call` always panics with the exact message
//! `"MockPanicOnCall: intentional failure"`, so callers can match on it with
//! `#[should_panic(expected = ...)]`.
//!
//! ```ignore
//! let id = env.register_contract(None, MockPanicOnCall);
//! let client = MockPanicOnCallClient::new(&env, &id);
//! client.call(); // panics
//! ```

use soroban_sdk::{contract, contractimpl, Env};

/// The exact message `call` panics with.
pub const PANIC_MESSAGE: &str = "MockPanicOnCall: intentional failure";

#[contract]
pub struct MockPanicOnCall;

#[contractimpl]
impl MockPanicOnCall {
    /// Always panics with [`PANIC_MESSAGE`].
    pub fn call(_env: Env) {
        panic!("MockPanicOnCall: intentional failure");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    #[should_panic(expected = "intentional failure")]
    fn call_panics() {
        let env = Env::default();
        let id = env.register_contract(None, MockPanicOnCall);
        let client = MockPanicOnCallClient::new(&env, &id);
        client.call();
    }

    #[test]
    fn try_call_returns_err() {
        let env = Env::default();
        let id = env.register_contract(None, MockPanicOnCall);
        let client = MockPanicOnCallClient::new(&env, &id);
        // `try_call` lets the caller observe the failure without unwinding.
        assert!(client.try_call().is_err());
    }
}

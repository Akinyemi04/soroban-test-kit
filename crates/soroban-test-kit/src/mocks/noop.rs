//! A no-op / identity contract mock.
//!
//! The simplest possible cross-contract call target: `echo` returns its
//! argument unchanged and `ping` does nothing. Useful for testing call
//! routing, address plumbing, and that a caller forwards arguments correctly
//! without any side effects to reason about.
//!
//! ```ignore
//! let id = env.register_contract(None, MockNoop);
//! let client = MockNoopClient::new(&env, &id);
//! assert_eq!(client.echo(&42), 42);
//! client.ping();
//! ```

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct MockNoop;

#[contractimpl]
impl MockNoop {
    /// Return `val` unchanged.
    pub fn echo(_env: Env, val: i128) -> i128 {
        val
    }

    /// Do nothing. Useful as a trivial invocation target.
    pub fn ping(_env: Env) {}
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn echo_round_trip() {
        let env = Env::default();
        let id = env.register_contract(None, MockNoop);
        let client = MockNoopClient::new(&env, &id);

        assert_eq!(client.echo(&0), 0);
        assert_eq!(client.echo(&42), 42);
        assert_eq!(client.echo(&-7), -7);
        assert_eq!(client.echo(&i128::MAX), i128::MAX);
    }

    #[test]
    fn ping_runs() {
        let env = Env::default();
        let id = env.register_contract(None, MockNoop);
        let client = MockNoopClient::new(&env, &id);
        client.ping();
    }
}

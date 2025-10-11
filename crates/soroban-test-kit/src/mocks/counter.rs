//! A stateful counter contract mock.
//!
//! The canonical fixture for testing storage persistence across invocations:
//! `increment` bumps a stored `u32` and returns the new value, `get` reads it,
//! and `reset` clears it back to zero.
//!
//! ## Storage persistence
//! The count lives in **instance** storage under a single key, so it persists
//! across separate client calls within the same test `Env` (each call is a
//! distinct invocation reading the same stored value).
//!
//! ```ignore
//! let id = env.register_contract(None, MockCounter);
//! let client = MockCounterClient::new(&env, &id);
//! assert_eq!(client.increment(), 1);
//! assert_eq!(client.increment(), 2);
//! assert_eq!(client.get(), 2);
//! client.reset();
//! assert_eq!(client.get(), 0);
//! ```

use soroban_sdk::{contract, contractimpl, contracttype, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Count,
}

#[contract]
pub struct MockCounter;

#[contractimpl]
impl MockCounter {
    /// Increment the stored count and return the new value.
    pub fn increment(env: Env) -> u32 {
        let next = Self::get(env.clone()) + 1;
        env.storage().instance().set(&DataKey::Count, &next);
        next
    }

    /// Read the current count. Returns `0` before the first increment.
    pub fn get(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::Count)
            .unwrap_or(0)
    }

    /// Reset the count back to zero.
    pub fn reset(env: Env) {
        env.storage().instance().set(&DataKey::Count, &0u32);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn increment_persists_across_calls() {
        let env = Env::default();
        let id = env.register_contract(None, MockCounter);
        let client = MockCounterClient::new(&env, &id);

        assert_eq!(client.get(), 0);
        assert_eq!(client.increment(), 1);
        assert_eq!(client.increment(), 2);
        assert_eq!(client.increment(), 3);
        assert_eq!(client.get(), 3);
    }

    #[test]
    fn reset_clears_count() {
        let env = Env::default();
        let id = env.register_contract(None, MockCounter);
        let client = MockCounterClient::new(&env, &id);

        client.increment();
        client.increment();
        client.reset();
        assert_eq!(client.get(), 0);
        assert_eq!(client.increment(), 1);
    }
}

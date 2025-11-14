//! A call target whose return value is configured ahead of time.
//!
//! Lets a caller test how it handles an arbitrary downstream response without
//! writing a bespoke fixture: `set_return(val)` stores a value and `call()`
//! returns it. Before any `set_return`, `call` returns `0`.
//!
//! ```ignore
//! let id = env.register_contract(None, MockConfigurableReceiver);
//! let client = MockConfigurableReceiverClient::new(&env, &id);
//! client.set_return(&42);
//! assert_eq!(client.call(), 42);
//! ```

use soroban_sdk::{contract, contractimpl, contracttype, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Return,
}

#[contract]
pub struct MockConfigurableReceiver;

#[contractimpl]
impl MockConfigurableReceiver {
    /// Configure the value `call` will return.
    pub fn set_return(env: Env, val: i128) {
        env.storage().instance().set(&DataKey::Return, &val);
    }

    /// Return the configured value, or `0` if none was set.
    pub fn call(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Return)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn returns_configured_value() {
        let env = Env::default();
        let id = env.register_contract(None, MockConfigurableReceiver);
        let client = MockConfigurableReceiverClient::new(&env, &id);

        assert_eq!(client.call(), 0);
        client.set_return(&42);
        assert_eq!(client.call(), 42);
        client.set_return(&-9);
        assert_eq!(client.call(), -9);
    }
}

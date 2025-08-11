//! A contract mock that emits a configurable event on demand.
//!
//! This is the natural fixture for exercising event-assertion helpers: call
//! `emit(topic, data)` and then inspect `env.events()`.
//!
//! ## Event shape
//! The published event has a single-element topic tuple `(topic,)` where
//! `topic` is a [`Symbol`], and the data payload is the `i128` `data` value.
//!
//! ```ignore
//! let id = env.register_contract(None, MockEventEmitter);
//! let client = MockEventEmitterClient::new(&env, &id);
//! client.emit(&symbol_short!("transfer"), &100);
//! let events = env.events().all();
//! assert_eq!(events.len(), 1);
//! ```

use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct MockEventEmitter;

#[contractimpl]
impl MockEventEmitter {
    /// Publish an event with topic `(topic,)` and data `data`.
    pub fn emit(env: Env, topic: Symbol, data: i128) {
        env.events().publish((topic,), data);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{symbol_short, testutils::Events, Env};

    #[test]
    fn emitted_event_appears() {
        let env = Env::default();
        let id = env.register_contract(None, MockEventEmitter);
        let client = MockEventEmitterClient::new(&env, &id);

        client.emit(&symbol_short!("transfer"), &100);

        let events = env.events().all();
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn multiple_emits_accumulate() {
        let env = Env::default();
        let id = env.register_contract(None, MockEventEmitter);
        let client = MockEventEmitterClient::new(&env, &id);

        client.emit(&symbol_short!("a"), &1);
        client.emit(&symbol_short!("b"), &2);

        let events = env.events().all();
        assert_eq!(events.len(), 2);
    }
}

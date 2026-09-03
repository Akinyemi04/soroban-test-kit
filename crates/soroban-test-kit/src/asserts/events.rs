//! An event-emission assertion.
//!
//! Verifying a contract published an event means inspecting `env.events()`
//! and matching a contract ID, topics, and data — plumbing every test that
//! cares about events otherwise has to repeat. `assert_event_emitted`
//! packages that into one call using the same topics/data shapes accepted by
//! [`Events::publish`](soroban_sdk::events::Events::publish).
//!
//! ```ignore
//! use soroban_test_kit::asserts::events::assert_event_emitted;
//! use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env};
//!
//! let env = Env::default();
//! let id = env.register_contract(None, MockEventEmitter);
//! let client = MockEventEmitterClient::new(&env, &id);
//!
//! client.emit(&symbol_short!("transfer"), &100);
//!
//! assert_event_emitted(&env, &id, (symbol_short!("transfer"),), 100i128);
//! ```

use soroban_sdk::testutils::Events;
use soroban_sdk::{Address, Env, IntoVal, Val, Vec};

/// Asserts that `contract_id` published an event matching `topics` and
/// `data` at some point during the test `Env`'s history.
///
/// `topics` and `data` accept the same shapes `Events::publish` does (e.g. a
/// tuple of topic values, and any `IntoVal<Env, Val>` for the data payload).
/// Comparison is by value via the host environment, so equal contents match
/// regardless of how they were constructed — but comparison is also
/// type-aware, so `data` must be passed as the exact numeric type the
/// contract published (e.g. `100i128`, not a bare `100` that defaults to
/// `i32`), or the `Val`s carry different type tags and never match.
pub fn assert_event_emitted<T, D>(env: &Env, contract_id: &Address, topics: T, data: D)
where
    T: IntoVal<Env, Vec<Val>>,
    D: IntoVal<Env, Val>,
{
    let expected = (
        contract_id.clone(),
        topics.into_val(env),
        data.into_val(env),
    );
    if !env.events().all().contains(expected) {
        panic!(
            "assertion failed: expected {:?} to have emitted a matching event, but none was found",
            contract_id
        );
    }
}

#[cfg(test)]
mod test {
    use super::assert_event_emitted;
    use crate::mocks::event_emitter::{MockEventEmitter, MockEventEmitterClient};
    use soroban_sdk::{symbol_short, Env};

    #[test]
    fn matches_a_known_event() {
        let env = Env::default();
        let id = env.register_contract(None, MockEventEmitter);
        let client = MockEventEmitterClient::new(&env, &id);

        client.emit(&symbol_short!("transfer"), &100);

        assert_event_emitted(&env, &id, (symbol_short!("transfer"),), 100i128);
    }

    #[test]
    #[should_panic(expected = "but none was found")]
    fn fails_on_wrong_topic() {
        let env = Env::default();
        let id = env.register_contract(None, MockEventEmitter);
        let client = MockEventEmitterClient::new(&env, &id);

        client.emit(&symbol_short!("transfer"), &100);

        assert_event_emitted(&env, &id, (symbol_short!("mint"),), 100i128);
    }

    #[test]
    #[should_panic(expected = "but none was found")]
    fn fails_on_wrong_data() {
        let env = Env::default();
        let id = env.register_contract(None, MockEventEmitter);
        let client = MockEventEmitterClient::new(&env, &id);

        client.emit(&symbol_short!("transfer"), &100);

        assert_event_emitted(&env, &id, (symbol_short!("transfer"),), 999i128);
    }
}

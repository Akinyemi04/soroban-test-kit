//! Mock contracts for use as dependencies in other contracts' tests.
//!
//! Each mock is a real, registerable Soroban contract that stands in for an
//! external dependency so the contract under test can be exercised in
//! isolation. New mocks are tracked as individual GitHub issues — one file
//! per mock keeps them merge-safe.

pub mod allowance_token;
pub mod burnable_token;
pub mod configurable_receiver;
pub mod counter;
pub mod decimals_token;
pub mod event_emitter;
pub mod fee_token;
pub mod noop;
pub mod oracle;
pub mod panic_on_call;
pub mod sac;
pub mod token;

//! Mock contracts for use as dependencies in other contracts' tests.
//!
//! Each mock is a real, registerable Soroban contract that stands in for an
//! external dependency so the contract under test can be exercised in
//! isolation. New mocks (oracle, SAC-style asset, flash-loan receiver, …) are
//! tracked as individual seed issues — one file per mock keeps them merge-safe.

pub mod token;
pub mod burnable_token;
pub mod allowance_token;
pub mod noop;
pub mod panic_on_call;
pub mod event_emitter;

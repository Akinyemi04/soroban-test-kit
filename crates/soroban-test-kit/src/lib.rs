#![no_std]
//! # soroban-test-kit
//!
//! Reusable testing infrastructure for Soroban smart contracts: mock contracts,
//! assertion helpers, and (over time) fuzzing/invariant harnesses.
//!
//! Add it as a dev-dependency in any Soroban contract crate:
//!
//! ```toml
//! [dev-dependencies]
//! soroban-test-kit = { git = "https://github.com/your-org/soroban-test-kit" }
//! ```
//!
//! Then pull in the common items:
//!
//! ```ignore
//! use soroban_test_kit::prelude::*;
//! ```
//!
//! The crate is `no_std` like every Soroban contract. The mock contracts are
//! gated behind the `testutils` feature (enabled by default) because they rely
//! on the Soroban SDK test environment.

pub mod asserts;
pub mod mocks;
pub mod prelude;

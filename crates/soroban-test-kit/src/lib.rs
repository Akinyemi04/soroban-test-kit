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
//! soroban-test-kit = { git = "https://github.com/Akinyemi04/soroban-test-kit" }
//! ```
//!
//! Then pull in the common items:
//!
//! ```ignore
//! use soroban_test_kit::prelude::*;
//! ```
//!
//! The crate is `no_std` like every Soroban contract. The mocks themselves
//! build without any extra features; enable the optional `testutils` feature
//! for helpers that need the Soroban SDK test environment (e.g.
//! `asserts::panics::assert_panics`).

pub mod asserts;
pub mod mocks;
pub mod prelude;

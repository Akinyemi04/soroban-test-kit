//! Address membership assertions against a `Vec<Address>`.
//!
//! Membership checks against an allowlist or holder set read better as a named
//! assertion than a manual loop. [`address_in_set`] is the predicate;
//! `assert_address_in_set!` / `assert_address_not_in_set!` wrap it.
//!
//! ```ignore
//! use soroban_sdk::{testutils::Address as _, vec, Address, Env};
//! use soroban_test_kit::assert_address_in_set;
//!
//! let env = Env::default();
//! let a = Address::generate(&env);
//! let set = vec![&env, a.clone()];
//! assert_address_in_set!(a, set);
//! ```

use soroban_sdk::{Address, Vec};

/// Returns `true` when `addr` appears in `set`.
pub fn address_in_set(addr: &Address, set: &Vec<Address>) -> bool {
    set.iter().any(|a| &a == addr)
}

/// Asserts `addr` is present in `set`.
#[macro_export]
macro_rules! assert_address_in_set {
    ($addr:expr, $set:expr $(,)?) => {{
        let addr = $addr;
        let set = $set;
        if !$crate::asserts::address_set::address_in_set(&addr, &set) {
            panic!("address {:?} not found in set {:?}", addr, set);
        }
    }};
}

/// Asserts `addr` is absent from `set`.
#[macro_export]
macro_rules! assert_address_not_in_set {
    ($addr:expr, $set:expr $(,)?) => {{
        let addr = $addr;
        let set = $set;
        if $crate::asserts::address_set::address_in_set(&addr, &set) {
            panic!("address {:?} unexpectedly present in set {:?}", addr, set);
        }
    }};
}

#[cfg(test)]
mod test {
    use super::address_in_set;
    use soroban_sdk::{testutils::Address as _, vec, Address, Env};

    #[test]
    fn present_address() {
        let env = Env::default();
        let a = Address::generate(&env);
        let b = Address::generate(&env);
        let set = vec![&env, a.clone(), b.clone()];
        assert!(address_in_set(&a, &set));
        assert_address_in_set!(b, set);
    }

    #[test]
    fn absent_address() {
        let env = Env::default();
        let a = Address::generate(&env);
        let stranger = Address::generate(&env);
        let set = vec![&env, a];
        assert!(!address_in_set(&stranger, &set));
        assert_address_not_in_set!(stranger, set);
    }

    #[test]
    #[should_panic(expected = "not found in set")]
    fn missing_panics() {
        let env = Env::default();
        let a = Address::generate(&env);
        let stranger = Address::generate(&env);
        let set = vec![&env, a];
        assert_address_in_set!(stranger, set);
    }
}

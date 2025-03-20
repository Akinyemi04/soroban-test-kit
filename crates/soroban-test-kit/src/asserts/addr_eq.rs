//! Address-equality assertion.
//!
//! `assert_eq_addr!(a, b)` reads better than `assert_eq!` on two
//! [`Address`](soroban_sdk::Address)es and prints both addresses on mismatch.
//!
//! ```ignore
//! use soroban_sdk::{testutils::Address as _, Address, Env};
//! use soroban_test_kit::assert_eq_addr;
//!
//! let env = Env::default();
//! let a = Address::generate(&env);
//! assert_eq_addr!(a, a.clone());
//! ```

/// Asserts two `Address` values are equal, printing both on mismatch.
#[macro_export]
macro_rules! assert_eq_addr {
    ($a:expr, $b:expr $(,)?) => {{
        let a = $a;
        let b = $b;
        if a != b {
            panic!("address mismatch: left = {:?}, right = {:?}", a, b);
        }
    }};
}

#[cfg(test)]
mod test {
    use soroban_sdk::{testutils::Address as _, Address, Env};

    #[test]
    fn equal_addresses_pass() {
        let env = Env::default();
        let a = Address::generate(&env);
        assert_eq_addr!(a.clone(), a.clone());
    }

    #[test]
    #[should_panic(expected = "address mismatch")]
    fn unequal_addresses_panic() {
        let env = Env::default();
        let a = Address::generate(&env);
        let b = Address::generate(&env);
        assert_eq_addr!(a, b);
    }
}

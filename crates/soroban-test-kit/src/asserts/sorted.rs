//! Sortedness assertions for SDK `Vec`s.
//!
//! Many DeFi structures (price ladders, tick arrays) must stay ordered.
//! [`is_sorted_asc`] / [`is_sorted_desc`] check non-decreasing / non-increasing
//! order, and `assert_sorted!` / `assert_sorted_desc!` wrap them with a
//! descriptive panic.
//!
//! ```ignore
//! use soroban_sdk::{vec, Env};
//! use soroban_test_kit::{assert_sorted, assert_sorted_desc};
//!
//! let env = Env::default();
//! assert_sorted!(vec![&env, 1, 2, 2, 3]);
//! assert_sorted_desc!(vec![&env, 3, 2, 2, 1]);
//! ```

use soroban_sdk::Vec;

/// Returns `true` when `v` is non-decreasing (each element `>=` its predecessor).
pub fn is_sorted_asc<T>(v: &Vec<T>) -> bool
where
    T: soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>
        + soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>
        + Clone
        + PartialOrd,
{
    let mut prev: Option<T> = None;
    for item in v.iter() {
        if let Some(p) = prev {
            if item < p {
                return false;
            }
        }
        prev = Some(item);
    }
    true
}

/// Returns `true` when `v` is non-increasing (each element `<=` its predecessor).
pub fn is_sorted_desc<T>(v: &Vec<T>) -> bool
where
    T: soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>
        + soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>
        + Clone
        + PartialOrd,
{
    let mut prev: Option<T> = None;
    for item in v.iter() {
        if let Some(p) = prev {
            if item > p {
                return false;
            }
        }
        prev = Some(item);
    }
    true
}

/// Asserts an SDK `Vec` is sorted in non-decreasing (ascending) order.
#[macro_export]
macro_rules! assert_sorted {
    ($v:expr $(,)?) => {{
        let v = $v;
        if !$crate::asserts::sorted::is_sorted_asc(&v) {
            panic!("assertion failed: vec is not sorted ascending: {:?}", v);
        }
    }};
}

/// Asserts an SDK `Vec` is sorted in non-increasing (descending) order.
#[macro_export]
macro_rules! assert_sorted_desc {
    ($v:expr $(,)?) => {{
        let v = $v;
        if !$crate::asserts::sorted::is_sorted_desc(&v) {
            panic!("assertion failed: vec is not sorted descending: {:?}", v);
        }
    }};
}

#[cfg(test)]
mod test {
    use super::{is_sorted_asc, is_sorted_desc};
    use soroban_sdk::{vec, Env};

    #[test]
    fn ascending_passes() {
        let env = Env::default();
        assert!(is_sorted_asc(&vec![&env, 1i32, 2, 2, 3]));
        assert_sorted!(vec![&env, 1i32, 2, 2, 3]);
    }

    #[test]
    fn descending_passes() {
        let env = Env::default();
        assert!(is_sorted_desc(&vec![&env, 3i32, 2, 2, 1]));
        assert_sorted_desc!(vec![&env, 3i32, 2, 2, 1]);
    }

    #[test]
    #[should_panic(expected = "not sorted ascending")]
    fn unsorted_asc_panics() {
        let env = Env::default();
        assert_sorted!(vec![&env, 1i32, 3, 2]);
    }

    #[test]
    #[should_panic(expected = "not sorted descending")]
    fn unsorted_desc_panics() {
        let env = Env::default();
        assert_sorted_desc!(vec![&env, 3i32, 1, 2]);
    }
}

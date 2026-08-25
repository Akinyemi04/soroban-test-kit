//! Order-insensitive `Vec` equality.
//!
//! `assert_vec_eq_unordered!(a, b)` asserts two
//! [`soroban_sdk::Vec`](soroban_sdk::Vec)s contain the same *multiset* of
//! elements (same elements with the same multiplicities), ignoring order.
//! Useful when listing holders or events where order is not guaranteed.
//!
//! ```ignore
//! use soroban_sdk::{vec, Env};
//! use soroban_test_kit::assert_vec_eq_unordered;
//!
//! let env = Env::default();
//! assert_vec_eq_unordered!(vec![&env, 1, 2, 3], vec![&env, 3, 1, 2]);
//! ```

use soroban_sdk::Vec;

/// Returns `true` when `a` and `b` contain the same multiset of elements.
///
/// Runs in O(n²): for each element of `a` it removes one matching element from
/// a working copy of `b`. Fine for the small collections used in tests.
pub fn vec_eq_unordered<T>(a: &Vec<T>, b: &Vec<T>) -> bool
where
    T: soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>
        + soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>
        + Clone
        + PartialEq,
{
    if a.len() != b.len() {
        return false;
    }
    let mut remaining = b.clone();
    for item in a.iter() {
        let mut found = None;
        for (idx, other) in remaining.iter().enumerate() {
            if item == other {
                found = Some(idx as u32);
                break;
            }
        }
        match found {
            Some(idx) => {
                remaining.remove(idx);
            }
            None => return false,
        }
    }
    remaining.is_empty()
}

/// Asserts two SDK `Vec`s contain the same multiset of elements, ignoring order.
#[macro_export]
macro_rules! assert_vec_eq_unordered {
    ($a:expr, $b:expr $(,)?) => {{
        let a = $a;
        let b = $b;
        if !$crate::asserts::vec_unordered::vec_eq_unordered(&a, &b) {
            panic!("vec multiset mismatch: left = {:?}, right = {:?}", a, b);
        }
    }};
}

#[cfg(test)]
mod test {
    use super::vec_eq_unordered;
    use soroban_sdk::{vec, Env};

    #[test]
    fn reordered_vecs_are_equal() {
        let env = Env::default();
        let a = vec![&env, 1i32, 2, 3];
        let b = vec![&env, 3i32, 1, 2];
        assert!(vec_eq_unordered(&a, &b));
        assert_vec_eq_unordered!(a, b);
    }

    #[test]
    fn duplicates_must_match_multiplicity() {
        let env = Env::default();
        let a = vec![&env, 1i32, 1, 2];
        let b = vec![&env, 1i32, 2, 2];
        assert!(!vec_eq_unordered(&a, &b));
    }

    #[test]
    #[should_panic(expected = "multiset mismatch")]
    fn mismatch_panics() {
        let env = Env::default();
        let a = vec![&env, 1i32, 2, 3];
        let b = vec![&env, 1i32, 2, 4];
        assert_vec_eq_unordered!(a, b);
    }
}

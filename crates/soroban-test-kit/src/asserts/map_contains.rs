//! Map-contains assertions for [`soroban_sdk::Map`].
//!
//! `assert_map_contains!(map, key, value)` asserts the map has `key` bound to
//! `value`; `assert_map_contains_key!(map, key)` checks key presence only.
//!
//! ```ignore
//! use soroban_sdk::{map, Env};
//! use soroban_test_kit::{assert_map_contains, assert_map_contains_key};
//!
//! let env = Env::default();
//! let m = map![&env, (1u32, 100i128)];
//! assert_map_contains!(m, 1u32, 100i128);
//! assert_map_contains_key!(m, 1u32);
//! ```

/// Asserts `map` contains `key` bound to `value`.
#[macro_export]
macro_rules! assert_map_contains {
    ($map:expr, $key:expr, $value:expr $(,)?) => {{
        let map = &$map;
        let key = $key;
        let value = $value;
        match map.get(key.clone()) {
            Some(actual) => {
                if actual != value {
                    panic!(
                        "map key {:?} bound to {:?}, expected {:?}",
                        key, actual, value
                    );
                }
            }
            None => panic!("map does not contain key {:?}", key),
        }
    }};
}

/// Asserts `map` contains `key` (value unchecked).
#[macro_export]
macro_rules! assert_map_contains_key {
    ($map:expr, $key:expr $(,)?) => {{
        let map = &$map;
        let key = $key;
        if !map.contains_key(key.clone()) {
            panic!("map does not contain key {:?}", key);
        }
    }};
}

#[cfg(test)]
mod test {
    use soroban_sdk::{map, Env};

    #[test]
    fn present_key_value() {
        let env = Env::default();
        let m = map![&env, (1u32, 100i128), (2u32, 200i128)];
        assert_map_contains!(m, 1u32, 100i128);
        assert_map_contains!(m, 2u32, 200i128);
        assert_map_contains_key!(m, 1u32);
    }

    #[test]
    #[should_panic(expected = "does not contain key")]
    fn absent_key_panics() {
        let env = Env::default();
        let m = map![&env, (1u32, 100i128)];
        assert_map_contains!(m, 9u32, 0i128);
    }

    #[test]
    #[should_panic(expected = "expected")]
    fn wrong_value_panics() {
        let env = Env::default();
        let m = map![&env, (1u32, 100i128)];
        assert_map_contains!(m, 1u32, 999i128);
    }
}

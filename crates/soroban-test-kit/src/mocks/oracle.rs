//! A mock price-feed contract.
//!
//! DeFi contract tests (AMMs, vaults, liquidation logic) constantly need a
//! price for some asset without wiring up a real oracle. `MockOracle` stores
//! a single `i128` price per `Address` and lets a test set it directly.
//!
//! ```ignore
//! let admin = Address::generate(&env);
//! let id = env.register_contract(None, MockOracle);
//! let oracle = MockOracleClient::new(&env, &id);
//!
//! let asset = Address::generate(&env);
//! oracle.set_price(&asset, &1_050_000);
//! assert_eq!(oracle.get_price(&asset), 1_050_000);
//! ```

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Price(Address),
}

#[contract]
pub struct MockOracle;

#[contractimpl]
impl MockOracle {
    /// Set `asset`'s price. Overwrites any previously set value.
    pub fn set_price(env: Env, asset: Address, price: i128) {
        env.storage()
            .persistent()
            .set(&DataKey::Price(asset), &price);
    }

    /// Read `asset`'s last-set price. Returns `0` for an asset with no price
    /// set yet.
    pub fn get_price(env: Env, asset: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Price(asset))
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn returns_last_set_price() {
        let env = Env::default();
        let id = env.register_contract(None, MockOracle);
        let client = MockOracleClient::new(&env, &id);
        let asset = Address::generate(&env);

        client.set_price(&asset, &1_000);
        assert_eq!(client.get_price(&asset), 1_000);

        client.set_price(&asset, &1_050);
        assert_eq!(client.get_price(&asset), 1_050);
    }

    #[test]
    fn unknown_asset_has_zero_price() {
        let env = Env::default();
        let id = env.register_contract(None, MockOracle);
        let client = MockOracleClient::new(&env, &id);
        let asset = Address::generate(&env);

        assert_eq!(client.get_price(&asset), 0);
    }

    #[test]
    fn prices_are_independent_per_asset() {
        let env = Env::default();
        let id = env.register_contract(None, MockOracle);
        let client = MockOracleClient::new(&env, &id);
        let asset_a = Address::generate(&env);
        let asset_b = Address::generate(&env);

        client.set_price(&asset_a, &500);
        client.set_price(&asset_b, &900);

        assert_eq!(client.get_price(&asset_a), 500);
        assert_eq!(client.get_price(&asset_b), 900);
    }
}

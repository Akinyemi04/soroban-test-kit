//! A minimal mock fungible token.
//!
//! This is intentionally *not* a production token — it omits allowances,
//! events, and decimals metadata. Its only job is to be a believable token
//! dependency that a contract under test can hold balances in, mint from, and
//! transfer through. For DeFi contracts (AMMs, vaults, payment splitters) this
//! removes the need to deploy the full Stellar Asset Contract in every test.
//!
//! ```ignore
//! use soroban_sdk::{testutils::Address as _, Address, Env};
//! use soroban_test_kit::prelude::*;
//!
//! let env = Env::default();
//! env.mock_all_auths();
//!
//! let admin = Address::generate(&env);
//! let id = env.register_contract(None, MockToken);
//! let token = MockTokenClient::new(&env, &id);
//! token.init(&admin);
//!
//! let alice = Address::generate(&env);
//! token.mint(&alice, &1_000);
//! assert_eq!(token.balance(&alice), 1_000);
//! ```

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Balance(Address),
}

#[contract]
pub struct MockToken;

#[contractimpl]
impl MockToken {
    /// One-time setup. `admin` is the only address allowed to `mint`.
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Mint `amount` of the token to `to`. Admin-only.
    pub fn mint(env: Env, to: Address, amount: i128) {
        assert!(amount >= 0, "amount must be non-negative");
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("not initialized");
        admin.require_auth();

        let balance = Self::balance(env.clone(), to.clone());
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to), &(balance + amount));
    }

    /// Move `amount` from `from` to `to`. Requires `from`'s authorization.
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        assert!(amount >= 0, "amount must be non-negative");
        from.require_auth();

        let from_balance = Self::balance(env.clone(), from.clone());
        assert!(from_balance >= amount, "insufficient balance");
        let to_balance = Self::balance(env.clone(), to.clone());

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from), &(from_balance - amount));
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to), &(to_balance + amount));
    }

    /// Read `id`'s balance. Returns `0` for an unknown address.
    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(id))
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    fn setup(env: &Env) -> (MockTokenClient<'_>, Address) {
        let admin = Address::generate(env);
        let id = env.register_contract(None, MockToken);
        let client = MockTokenClient::new(env, &id);
        client.init(&admin);
        (client, admin)
    }

    #[test]
    fn mint_then_transfer() {
        let env = Env::default();
        env.mock_all_auths();
        let (token, _admin) = setup(&env);

        let alice = Address::generate(&env);
        let bob = Address::generate(&env);

        token.mint(&alice, &1_000);
        assert_eq!(token.balance(&alice), 1_000);

        token.transfer(&alice, &bob, &400);
        assert_eq!(token.balance(&alice), 600);
        assert_eq!(token.balance(&bob), 400);
    }

    #[test]
    fn unknown_address_has_zero_balance() {
        let env = Env::default();
        env.mock_all_auths();
        let (token, _admin) = setup(&env);
        let stranger = Address::generate(&env);
        assert_eq!(token.balance(&stranger), 0);
    }

    #[test]
    #[should_panic(expected = "insufficient balance")]
    fn cannot_overdraw() {
        let env = Env::default();
        env.mock_all_auths();
        let (token, _admin) = setup(&env);
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        token.mint(&alice, &100);
        token.transfer(&alice, &bob, &101);
    }
}

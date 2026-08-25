//! A mock fungible token with explicit mint and burn.
//!
//! This extends the bare [`MockToken`](super::token::MockToken) idea with a
//! supply-reducing `burn(from, amount)` so flows that change total supply can
//! be tested directly. It is intentionally minimal: no allowances, events, or
//! decimals metadata.
//!
//! ## Burn semantics
//! - `burn(from, amount)` requires `from`'s authorization (the holder burns
//!   their own tokens).
//! - Burning more than `from` holds panics with `"insufficient balance"`.
//! - `amount` must be non-negative.
//!
//! ```ignore
//! use soroban_sdk::{testutils::Address as _, Address, Env};
//! use soroban_test_kit::mocks::burnable_token::{MockBurnableToken, MockBurnableTokenClient};
//!
//! let env = Env::default();
//! env.mock_all_auths();
//! let admin = Address::generate(&env);
//! let id = env.register_contract(None, MockBurnableToken);
//! let token = MockBurnableTokenClient::new(&env, &id);
//! token.init(&admin);
//! let alice = Address::generate(&env);
//! token.mint(&alice, &1_000);
//! token.burn(&alice, &400);
//! assert_eq!(token.balance(&alice), 600);
//! ```

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Balance(Address),
}

#[contract]
pub struct MockBurnableToken;

#[contractimpl]
impl MockBurnableToken {
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

    /// Burn `amount` of `from`'s balance, reducing supply. Requires `from`'s
    /// authorization. Burning more than held panics with `"insufficient balance"`.
    pub fn burn(env: Env, from: Address, amount: i128) {
        assert!(amount >= 0, "amount must be non-negative");
        from.require_auth();

        let from_balance = Self::balance(env.clone(), from.clone());
        assert!(from_balance >= amount, "insufficient balance");

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from), &(from_balance - amount));
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

    fn setup(env: &Env) -> (MockBurnableTokenClient<'_>, Address) {
        let admin = Address::generate(env);
        let id = env.register_contract(None, MockBurnableToken);
        let client = MockBurnableTokenClient::new(env, &id);
        client.init(&admin);
        (client, admin)
    }

    #[test]
    fn mint_burn_balance_flow() {
        let env = Env::default();
        env.mock_all_auths();
        let (token, _admin) = setup(&env);

        let alice = Address::generate(&env);
        token.mint(&alice, &1_000);
        assert_eq!(token.balance(&alice), 1_000);

        token.burn(&alice, &400);
        assert_eq!(token.balance(&alice), 600);

        token.burn(&alice, &600);
        assert_eq!(token.balance(&alice), 0);
    }

    #[test]
    #[should_panic(expected = "insufficient balance")]
    fn cannot_burn_more_than_held() {
        let env = Env::default();
        env.mock_all_auths();
        let (token, _admin) = setup(&env);
        let alice = Address::generate(&env);
        token.mint(&alice, &100);
        token.burn(&alice, &101);
    }
}

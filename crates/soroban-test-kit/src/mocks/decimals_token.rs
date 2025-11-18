//! A mock fungible token exposing a fixed `decimals()`.
//!
//! Mirrors the bare [`MockToken`](super::token::MockToken) (mint / transfer /
//! balance) and adds a constant `decimals()` so tests that exercise scaling
//! logic can read the precision without deploying the full SAC.
//!
//! ## Decimals
//! `decimals()` is a fixed constant ([`DECIMALS`], `2`), chosen to differ from
//! the typical Stellar `7` so scaling bugs are easy to spot in tests. It is not
//! configurable at init by design (keeps the mock single-purpose).
//!
//! ```ignore
//! token.mint(&alice, &1_000);
//! assert_eq!(token.balance(&alice), 1_000);
//! assert_eq!(token.decimals(), 2);
//! ```

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

/// Fixed decimals reported by this mock.
pub const DECIMALS: u32 = 2;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Balance(Address),
}

#[contract]
pub struct MockDecimalsToken;

#[contractimpl]
impl MockDecimalsToken {
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

    /// Fixed token decimals. See [`DECIMALS`].
    pub fn decimals(_env: Env) -> u32 {
        DECIMALS
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    fn setup(env: &Env) -> (MockDecimalsTokenClient, Address) {
        let admin = Address::generate(env);
        let id = env.register_contract(None, MockDecimalsToken);
        let client = MockDecimalsTokenClient::new(env, &id);
        client.init(&admin);
        (client, admin)
    }

    #[test]
    fn mint_transfer_balance_flow() {
        let env = Env::default();
        env.mock_all_auths();
        let (token, _admin) = setup(&env);

        let alice = Address::generate(&env);
        let bob = Address::generate(&env);

        token.mint(&alice, &1_000);
        token.transfer(&alice, &bob, &250);
        assert_eq!(token.balance(&alice), 750);
        assert_eq!(token.balance(&bob), 250);
    }

    #[test]
    fn decimals_is_fixed() {
        let env = Env::default();
        env.mock_all_auths();
        let (token, _admin) = setup(&env);
        assert_eq!(token.decimals(), 2);
    }
}

//! A mock fungible token with a minimal allowance surface.
//!
//! This fills the gap between the bare [`MockToken`](super::token::MockToken)
//! and a full SAC-style mock: it adds `approve` / `allowance` / `transfer_from`
//! so spender flows (routers, vaults, payment pullers) can be tested. It is
//! intentionally minimal: no events, decimals, or allowance expiry ledgers.
//!
//! ## Allowance semantics
//! - `approve(from, spender, amount)` sets (overwrites) the allowance and
//!   requires `from`'s authorization.
//! - `transfer_from(spender, from, to, amount)` requires `spender`'s auth,
//!   decrements the allowance, and moves the balance.
//! - Spending more than the allowance panics with `"insufficient allowance"`;
//!   spending more than the balance panics with `"insufficient balance"`.
//!
//! ```ignore
//! token.mint(&alice, &1_000);
//! token.approve(&alice, &spender, &300);
//! token.transfer_from(&spender, &alice, &bob, &300);
//! ```

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Balance(Address),
    Allowance(Address, Address),
}

#[contract]
pub struct MockAllowanceToken;

#[contractimpl]
impl MockAllowanceToken {
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

    /// Set `spender`'s allowance over `from`'s balance to `amount` (overwrite).
    /// Requires `from`'s authorization.
    pub fn approve(env: Env, from: Address, spender: Address, amount: i128) {
        assert!(amount >= 0, "amount must be non-negative");
        from.require_auth();
        env.storage()
            .persistent()
            .set(&DataKey::Allowance(from, spender), &amount);
    }

    /// Read `spender`'s remaining allowance over `from`. `0` if never set.
    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Allowance(from, spender))
            .unwrap_or(0)
    }

    /// Move `amount` from `from` to `to`, spending `spender`'s allowance.
    /// Requires `spender`'s authorization.
    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        assert!(amount >= 0, "amount must be non-negative");
        spender.require_auth();

        let allowed = Self::allowance(env.clone(), from.clone(), spender.clone());
        assert!(allowed >= amount, "insufficient allowance");

        let from_balance = Self::balance(env.clone(), from.clone());
        assert!(from_balance >= amount, "insufficient balance");
        let to_balance = Self::balance(env.clone(), to.clone());

        env.storage().persistent().set(
            &DataKey::Allowance(from.clone(), spender),
            &(allowed - amount),
        );
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

    fn setup(env: &Env) -> (MockAllowanceTokenClient<'_>, Address) {
        let admin = Address::generate(env);
        let id = env.register_contract(None, MockAllowanceToken);
        let client = MockAllowanceTokenClient::new(env, &id);
        client.init(&admin);
        (client, admin)
    }

    #[test]
    fn approve_spend_exhaust() {
        let env = Env::default();
        env.mock_all_auths();
        let (token, _admin) = setup(&env);

        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        let spender = Address::generate(&env);

        token.mint(&alice, &1_000);
        token.approve(&alice, &spender, &300);
        assert_eq!(token.allowance(&alice, &spender), 300);

        token.transfer_from(&spender, &alice, &bob, &200);
        assert_eq!(token.balance(&bob), 200);
        assert_eq!(token.balance(&alice), 800);
        assert_eq!(token.allowance(&alice, &spender), 100);

        token.transfer_from(&spender, &alice, &bob, &100);
        assert_eq!(token.allowance(&alice, &spender), 0);
    }

    #[test]
    #[should_panic(expected = "insufficient allowance")]
    fn cannot_overspend_allowance() {
        let env = Env::default();
        env.mock_all_auths();
        let (token, _admin) = setup(&env);
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        let spender = Address::generate(&env);
        token.mint(&alice, &1_000);
        token.approve(&alice, &spender, &50);
        token.transfer_from(&spender, &alice, &bob, &51);
    }
}

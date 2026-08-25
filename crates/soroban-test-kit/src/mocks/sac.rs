//! A SAC-style asset mock.
//!
//! This mirrors the most-used parts of the Stellar Asset Contract surface —
//! `approve` / `allowance` / `transfer_from` / `decimals` — so DeFi tests can
//! exercise allowance-driven flows without deploying the full SAC.
//!
//! ## Divergences from the real SAC
//! - **No allowance expiry ledger.** The real SAC stores a `(amount,
//!   live_until_ledger)` pair and expires allowances; this mock stores only the
//!   amount and never expires it.
//! - **No events.** The real SAC publishes `approve` / `transfer` events; this
//!   mock omits them (use [`MockEventEmitter`](super::event_emitter) for event
//!   tests).
//! - **`decimals` is a fixed constant** ([`DECIMALS`]), not configured at init.
//! - **No `admin` / `clawback` / `set_authorized`** surface.
//!
//! ```ignore
//! token.mint(&alice, &1_000);
//! token.approve(&alice, &spender, &300);
//! token.transfer_from(&spender, &alice, &bob, &300);
//! assert_eq!(token.decimals(), 7);
//! ```

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

/// Fixed decimals reported by this mock (matches typical Stellar assets).
pub const DECIMALS: u32 = 7;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Balance(Address),
    Allowance(Address, Address),
}

#[contract]
pub struct MockSac;

#[contractimpl]
impl MockSac {
    /// One-time setup. `admin` is the only address allowed to `mint`.
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Mint `amount` to `to`. Admin-only.
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

    /// Set `spender`'s allowance over `from` to `amount`. Requires `from`'s auth.
    pub fn approve(env: Env, from: Address, spender: Address, amount: i128) {
        assert!(amount >= 0, "amount must be non-negative");
        from.require_auth();
        env.storage()
            .persistent()
            .set(&DataKey::Allowance(from, spender), &amount);
    }

    /// Read `spender`'s remaining allowance over `from`.
    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Allowance(from, spender))
            .unwrap_or(0)
    }

    /// Move `amount` from `from` to `to` using `spender`'s allowance.
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

    /// Fixed decimals. See [`DECIMALS`].
    pub fn decimals(_env: Env) -> u32 {
        DECIMALS
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    fn setup(env: &Env) -> (MockSacClient<'_>, Address) {
        let admin = Address::generate(env);
        let id = env.register_contract(None, MockSac);
        let client = MockSacClient::new(env, &id);
        client.init(&admin);
        (client, admin)
    }

    #[test]
    fn allowance_set_spend_exhaust() {
        let env = Env::default();
        env.mock_all_auths();
        let (token, _admin) = setup(&env);

        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        let spender = Address::generate(&env);

        token.mint(&alice, &1_000);
        token.approve(&alice, &spender, &300);
        assert_eq!(token.allowance(&alice, &spender), 300);

        token.transfer_from(&spender, &alice, &bob, &300);
        assert_eq!(token.balance(&bob), 300);
        assert_eq!(token.allowance(&alice, &spender), 0);
    }

    #[test]
    fn decimals_is_fixed() {
        let env = Env::default();
        env.mock_all_auths();
        let (token, _admin) = setup(&env);
        assert_eq!(token.decimals(), DECIMALS);
    }

    #[test]
    #[should_panic(expected = "insufficient allowance")]
    fn cannot_exceed_allowance() {
        let env = Env::default();
        env.mock_all_auths();
        let (token, _admin) = setup(&env);
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        let spender = Address::generate(&env);
        token.mint(&alice, &1_000);
        token.approve(&alice, &spender, &10);
        token.transfer_from(&spender, &alice, &bob, &11);
    }
}

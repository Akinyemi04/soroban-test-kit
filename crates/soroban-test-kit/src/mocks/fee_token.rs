//! A fee-on-transfer token mock.
//!
//! Fee-on-transfer tokens silently break AMMs and vaults that assume the
//! recipient receives exactly `amount`. This mock deducts a configurable
//! basis-point fee on every `transfer`, so the recipient receives `amount -
//! fee`. The fee is burned (removed from supply), not redirected.
//!
//! ## Fee rounding
//! The fee is `amount * fee_bps / 10_000` using integer division, which
//! **rounds the fee down** (and therefore rounds the recipient's credit up).
//! With `fee_bps == 0` transfers are fee-free. `fee_bps` is set at `init` and
//! must be in `0..=10_000`.
//!
//! ```ignore
//! token.init(&admin, &100); // 1% fee
//! token.mint(&alice, &1_000);
//! token.transfer(&alice, &bob, &1_000);
//! assert_eq!(token.balance(&bob), 990); // 1% fee deducted
//! ```

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

/// Denominator for basis-point math (100% == 10_000 bps).
pub const BPS_DENOMINATOR: i128 = 10_000;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    FeeBps,
    Balance(Address),
}

#[contract]
pub struct MockFeeToken;

#[contractimpl]
impl MockFeeToken {
    /// One-time setup. `admin` may `mint`; `fee_bps` is the per-transfer fee in
    /// basis points and must be in `0..=10_000`.
    pub fn init(env: Env, admin: Address, fee_bps: i128) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        assert!(
            (0..=BPS_DENOMINATOR).contains(&fee_bps),
            "fee_bps out of range"
        );
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::FeeBps, &fee_bps);
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

    /// Transfer `amount` from `from`, crediting `to` with `amount - fee`. The
    /// fee is burned. Requires `from`'s authorization.
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        assert!(amount >= 0, "amount must be non-negative");
        from.require_auth();

        let from_balance = Self::balance(env.clone(), from.clone());
        assert!(from_balance >= amount, "insufficient balance");

        let fee = amount * Self::fee_bps(env.clone()) / BPS_DENOMINATOR;
        let credited = amount - fee;
        let to_balance = Self::balance(env.clone(), to.clone());

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from), &(from_balance - amount));
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to), &(to_balance + credited));
    }

    /// The configured per-transfer fee in basis points.
    pub fn fee_bps(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::FeeBps)
            .expect("not initialized")
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

    fn setup(env: &Env, fee_bps: i128) -> (MockFeeTokenClient<'_>, Address) {
        let admin = Address::generate(env);
        let id = env.register_contract(None, MockFeeToken);
        let client = MockFeeTokenClient::new(env, &id);
        client.init(&admin, &fee_bps);
        (client, admin)
    }

    #[test]
    fn recipient_receives_amount_minus_fee() {
        let env = Env::default();
        env.mock_all_auths();
        let (token, _admin) = setup(&env, 100); // 1%

        let alice = Address::generate(&env);
        let bob = Address::generate(&env);

        token.mint(&alice, &1_000);
        token.transfer(&alice, &bob, &1_000);

        assert_eq!(token.balance(&alice), 0);
        assert_eq!(token.balance(&bob), 990); // 10 burned as fee
    }

    #[test]
    fn zero_fee_is_lossless() {
        let env = Env::default();
        env.mock_all_auths();
        let (token, _admin) = setup(&env, 0);
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        token.mint(&alice, &500);
        token.transfer(&alice, &bob, &500);
        assert_eq!(token.balance(&bob), 500);
    }

    #[test]
    fn fee_rounds_down() {
        let env = Env::default();
        env.mock_all_auths();
        let (token, _admin) = setup(&env, 100); // 1%
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        token.mint(&alice, &50);
        // fee = 50 * 100 / 10_000 = 0 (rounds down) -> bob gets full 50
        token.transfer(&alice, &bob, &50);
        assert_eq!(token.balance(&bob), 50);
    }

    #[test]
    #[should_panic(expected = "insufficient balance")]
    fn cannot_overdraw() {
        let env = Env::default();
        env.mock_all_auths();
        let (token, _admin) = setup(&env, 100);
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        token.mint(&alice, &10);
        token.transfer(&alice, &bob, &11);
    }
}

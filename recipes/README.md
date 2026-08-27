# Recipes

Copy-pasteable test patterns built from `soroban-test-kit`'s mocks and
asserts. Each recipe is self-contained and uses only shipped APIs — see the
[module table in the README](../README.md#whats-inside) for what's
available.

## Testing a contract that holds a token balance

Say your contract needs to hold a token balance for a user — a vault, an
escrow, a payment splitter. You don't want to deploy the full Stellar Asset
Contract just to exercise that path in a unit test. Use `MockToken` instead:

```rust
use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_test_kit::prelude::*;

#[test]
fn vault_tracks_deposited_balance() {
    let env = Env::default();
    env.mock_all_auths();

    // Stand up a mock token instead of deploying the full SAC.
    let admin = Address::generate(&env);
    let token_id = env.register_contract(None, MockToken);
    let token = MockTokenClient::new(&env, &token_id);
    token.init(&admin);

    // Give a user a balance to deposit.
    let user = Address::generate(&env);
    token.mint(&user, &1_000_000);

    // ... deploy and call your contract-under-test here, e.g.:
    // let vault_id = env.register_contract(None, MyVault);
    // let vault = MyVaultClient::new(&env, &vault_id);
    // vault.deposit(&user, &token_id, &500_000);

    // Whatever moved, assert the token balances landed where expected.
    // The mock's balance() is exact, but real contract math often needs
    // rounding tolerance:
    assert_approx_eq!(token.balance(&user), 500_000, 0);
}
```

Notes:

- `MockToken` is intentionally minimal (mint/transfer/balance, no
  allowances or decimals). If your contract needs `approve`/`transfer_from`,
  reach for `mocks::allowance_token::MockAllowanceToken` or the closer-to-SAC
  `mocks::sac::MockSac` instead.
- `token.mint(...)` requires the admin's authorization, which
  `env.mock_all_auths()` satisfies for every address in the test `Env`. To
  test that *your* contract correctly requires authorization from a specific
  caller, see `asserts::auth::assert_auth_required`.

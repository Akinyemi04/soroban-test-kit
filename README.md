# soroban-test-kit

[![CI](https://github.com/your-org/soroban-test-kit/actions/workflows/ci.yml/badge.svg)](https://github.com/your-org/soroban-test-kit/actions/workflows/ci.yml)

**Reusable testing infrastructure for Soroban smart contracts** — mock
contracts, assertion helpers, and fuzzing/invariant harnesses you add as a
single dev-dependency instead of rewriting in every repo.

Writing good Soroban tests means repeatedly hand-rolling the same scaffolding:
a stand-in token to hold balances, a fake price oracle, helpers to assert a
swap stayed within a fee tolerance, or to confirm a call panics without the
right authorization. `soroban-test-kit` factors that boilerplate into one
audited, `no_std` crate so contract authors can focus on testing *their* logic.

---

## What's inside

| Module | Status | Provides |
|---|---|---|
| `mocks::token` | ✅ shipped | A minimal, registerable mock fungible token (mint / transfer / balance) |
| `asserts` | ✅ shipped | `approx_eq` + `assert_approx_eq!` for rounding-tolerant math checks |
| `mocks::oracle` | 🚧 seed issue | Mock price-feed contract for DeFi tests |
| `mocks::sac` | 🚧 seed issue | Stellar Asset Contract–style mock |
| `asserts::auth` | 🚧 seed issue | `assert_auth_required!` helper |
| `asserts::budget` | 🚧 seed issue | Gas/CPU/memory budget ceilings |
| `harness` | 🚧 seed issue | Property-based & invariant fuzzing loops |
| `recipes/` | 🚧 seed issue | Documented, copy-pasteable test patterns |

The 🚧 items are scoped, beginner-friendly issues — see
[`OPEN_SOURCE_ISSUES.md`](./OPEN_SOURCE_ISSUES.md).

---

## Usage

Add it as a dev-dependency in any Soroban contract crate:

```toml
[dev-dependencies]
soroban-test-kit = { git = "https://github.com/your-org/soroban-test-kit" }
```

Then in your test module:

```rust
use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_test_kit::prelude::*;

#[test]
fn pool_accepts_deposits() {
    let env = Env::default();
    env.mock_all_auths();

    // Stand up a mock token instead of deploying the full SAC.
    let admin = Address::generate(&env);
    let token_id = env.register_contract(None, MockToken);
    let token = MockTokenClient::new(&env, &token_id);
    token.init(&admin);

    let lp = Address::generate(&env);
    token.mint(&lp, &1_000_000);

    // ... exercise your contract using `token_id`, then:
    assert_approx_eq!(token.balance(&lp), 1_000_000, 2); // rounding-tolerant
}
```

---

## Building & testing

```sh
rustup target add wasm32-unknown-unknown   # one-time
cargo test --workspace                     # run the suite
cargo build --release --target wasm32-unknown-unknown
```

Pinned to `soroban-sdk 21.7.7` to stay in lockstep with the rest of the
ecosystem (`soroban-amm`, `stellarspend-contracts`).

---

## Contributing

This repository participates in **[Drips Wave](https://docs.drips.network/wave/)**.
Issues are scoped to a single file each and tagged with point values. See
[`CONTRIBUTING.md`](./CONTRIBUTING.md) and
[`OPEN_SOURCE_ISSUES.md`](./OPEN_SOURCE_ISSUES.md) to get started.

## License

[MIT](./LICENSE).

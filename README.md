# soroban-test-kit

[![CI](https://github.com/Akinyemi04/soroban-test-kit/actions/workflows/ci.yml/badge.svg)](https://github.com/Akinyemi04/soroban-test-kit/actions/workflows/ci.yml)

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
| `mocks::sac` | ✅ shipped | Stellar Asset Contract–style mock (allowances, `transfer_from`, decimals) |
| `mocks` (burnable, allowance, fee-on-transfer, fixed-decimals, counter, event-emitter, no-op, panic-on-call, configurable-return) | ✅ shipped | Purpose-built stand-ins for common contract dependencies |
| `mocks::oracle` | ✅ shipped | Mock price-feed contract for DeFi tests |
| `asserts` (approx-eq, address-equality, sorted, vec-unordered, address-in-set, map-contains, zero/nonzero, panics, within-pct) | ✅ shipped | Ergonomic assertion helpers for common contract-test checks |
| `asserts::auth` | ✅ shipped | `assert_auth_required` helper (requires the `testutils` feature) |
| `recipes/` | ✅ shipped | Documented, copy-pasteable test patterns |
| `asserts::balances` | ✅ shipped | `assert_balance_change` helper |
| `asserts::events` | ✅ shipped | `assert_event_emitted` helper |
| `asserts::budget` | 🚧 open issue | Gas/CPU/memory budget ceilings |
| `mocks::flash_receiver` | 🚧 open issue | Configurable flash-loan receiver mock |
| `harness` | 🚧 open issue | Property-based & invariant fuzzing loops |

The 🚧 items are tracked as [open issues](https://github.com/Akinyemi04/soroban-test-kit/issues) —
see [`CONTRIBUTING.md`](./CONTRIBUTING.md) to get started.

---

## Usage

Add it as a dev-dependency in any Soroban contract crate:

```toml
[dev-dependencies]
soroban-test-kit = { git = "https://github.com/Akinyemi04/soroban-test-kit" }
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

## Development setup

1. Install Rust via [rustup](https://rustup.rs) (stable toolchain).
2. Clone the repo:
   ```sh
   git clone https://github.com/Akinyemi04/soroban-test-kit.git
   cd soroban-test-kit
   ```
3. Run the test suite:
   ```sh
   cargo test --workspace --all-features
   ```

Pinned to `soroban-sdk 21.7.7`. This crate ships `rlib` only — the mocks
are registered natively in a test `Env` (`env.register_contract(...)`),
never compiled to wasm themselves, since several are intentionally
interchangeable (same `balance`/`transfer`/`mint` surface) and that only
works when each lives in its own contract's wasm binary, not bundled
together in one.

---

## Roadmap

Near-term work is tracked as scoped, single-file
[open issues](https://github.com/Akinyemi04/soroban-test-kit/issues) —
mostly rounding out `asserts` (budget ceilings) and `mocks`
(flash-loan receiver), plus a property-based/invariant fuzzing `harness`
and CI coverage reporting. See the status table above for what's shipped.

Beyond that:

- Publish to [crates.io](https://crates.io) once the current API has had
  time to settle.
- A `cargo-soroban-test` companion CLI for scaffolding new test modules
  from the shipped mocks/asserts.

---

## Contributing

Contributions are welcome. Check the
[open issues](https://github.com/Akinyemi04/soroban-test-kit/issues) — many
are scoped to a single new file and labeled `good first issue` — and see
[`CONTRIBUTING.md`](./CONTRIBUTING.md) for the development workflow.

## Questions & contact

Bug reports, feature requests, and usage questions all go through
[GitHub Issues](https://github.com/Akinyemi04/soroban-test-kit/issues).
For anything else, reach the maintainer at
[@Akinyemi04](https://github.com/Akinyemi04).

## License

[MIT](./LICENSE).

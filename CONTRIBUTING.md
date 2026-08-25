# Contributing to soroban-test-kit

Thanks for your interest in contributing!

## Ground rules

- **One contributor per issue, one primary file per issue** where the issue
  says so. Don't expand scope outside the file listed in the issue unless a
  maintainer asks.
- **Every new behavior needs a test.** Tests live in a `#[cfg(test)]` module
  alongside the code.
- If you'd like to work on an open issue, leave a comment so it can be
  assigned to you before you start.

## Development workflow

1. Fork and branch from `main`:
   ```sh
   git checkout -b feat/short-description
   ```
   Prefixes: `feat/`, `fix/`, `refactor/`, `test/`, `docs/`, `chore/`.

2. Make sure the toolchain is set up:
   ```sh
   rustup target add wasm32-unknown-unknown
   ```

3. Before opening a PR, confirm all of these pass:
   ```sh
   cargo fmt --all -- --check
   cargo clippy --all-targets -- -D warnings
   cargo test --workspace --all-features
   cargo build --release --target wasm32-unknown-unknown
   ```

4. Open a PR referencing the issue with `Closes #<issue>`. Explain what
   changed and why.

## Code style

- Run `cargo fmt` before committing (default `rustfmt` settings).
- No `unsafe`.
- Prefer explicit, overflow-checked arithmetic.
- Don't add dependencies without discussion — this is a lean, `no_std` crate.

## PR checklist

- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --all-targets -- -D warnings` passes
- [ ] `cargo test --workspace` passes
- [ ] WASM build succeeds
- [ ] New behavior is covered by tests

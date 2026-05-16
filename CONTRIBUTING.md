# Contributing to Soroban Escrow

Thank you for your interest in contributing. This project participates in the [Stellar Wave Program](https://drips.network/wave/stellar) on Drips — contributors earn USDC rewards for resolving issues.

## Prerequisites

- [Rust](https://rustup.rs/) 1.75+
- [Soroban CLI](https://developers.stellar.org/docs/smart-contracts/getting-started/setup)

Install the WASM target:
```bash
rustup target add wasm32-unknown-unknown
```

## Setup

```bash
git clone https://github.com/laugh-tales/soroban-escrow
cd soroban-escrow
cargo build
cargo test
```

All tests should pass before you begin working on an issue.

## Workflow

1. Comment on the issue you want to work on and wait to be assigned
2. Fork the repository
3. Create a branch: `git checkout -b fix/your-issue-name`
4. Make your changes
5. Run tests: `cargo test`
6. Run formatter: `cargo fmt`
7. Run linter: `cargo clippy`
8. Commit and open a Pull Request against `main`

## Commit Messages

Use conventional commit format:
- `feat: add dispute timeout logic`
- `fix: correct escrow state transition`
- `docs: update deploy instructions`
- `test: add refund edge case tests`

## Questions

Open a GitHub Discussion or comment on the relevant issue.

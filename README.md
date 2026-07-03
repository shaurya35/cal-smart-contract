# calculator

An Anchor-based Solana program that keeps a calculator value on chain.

The program is structured as a small arithmetic contract: it initializes an
account with a number, then updates that stored value through calculator
instructions such as doubling the value or adding another number.

## What it does

The program stores a single calculator value inside an on-chain account. Each
instruction reads that account, applies the requested arithmetic operation, and
writes the updated value back.

- `init` creates the calculator account and sets the initial value
- `double` multiplies the stored value by 2
- `add` adds a provided number to the stored value

So if the value starts at `5`, the instruction flow could look like this:

```
init(5)  ->  5
double   ->  10
add(7)   ->  17
```

## How it works

1. The program receives an Anchor account context for the calculator state.
2. The calculator account stores the current number as account data.
3. Each instruction mutates the account-backed value directly.
4. Anchor handles account validation, serialization, and program entrypoint
   wiring around the Rust instruction logic.

## Account data

The calculator state is stored in an Anchor account with one numeric field:

```rust
pub num: u32
```

Anchor adds its account discriminator before the stored data, so the account
space includes the discriminator plus the calculator value.

## Project layout

```
calculator/
  Anchor.toml                       Anchor workspace configuration
  Cargo.toml                        Rust workspace configuration
  programs/calculator/src/lib.rs    program entrypoint and instructions
  programs/calculator/Cargo.toml    program package settings
```

## Build

You need Rust, the Solana CLI, and Anchor installed.

```bash
anchor build
```

This builds the program into a Solana deployable `.so` file under
`target/deploy/`.

## Test

```bash
cargo test
```

Use this for Rust-side tests and local program checks.

## Deploy

```bash
anchor deploy
```

Run this against your chosen cluster, for example localnet or devnet. The
program id is configured in `Anchor.toml`.

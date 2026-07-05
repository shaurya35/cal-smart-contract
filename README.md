# calculator

A small Anchor program that stores a number on chain and does math on it.
The wallet that creates a calculator account is the only one allowed to
change its value.

## Instructions

- `initialize(initial_value)` - create the account and set the starting value
- `double` - multiply by 2
- `halve` - divide by 2 (integer division)
- `add(amount)` - add a number
- `sub(amount)` - subtract a number

All math is checked, so overflow/underflow fails with a program error
instead of wrapping.

## State

```rust
#[account]
pub struct CalculatorState {
    pub value: u32,
    pub authority: Pubkey,
}
```

`authority` is set on initialize and enforced with `has_one = authority`
on every mutation.

## Build

```bash
anchor build
```

If you changed the program, regenerate the IDL and TS types:

```bash
anchor idl build -o target/idl/calculator.json -t target/types/calculator.ts
```

## Test

Tests live in `tests/calculator.spec.ts` and run with bun.

```bash
bun install   # first time only
anchor test
```

## Deploy

```bash
anchor deploy
```

Program ID lives in `Anchor.toml` and `declare_id!` in `lib.rs`. If they get
out of sync with the keypair in `target/deploy/`, run `anchor keys sync`.

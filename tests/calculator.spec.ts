import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair } from "@solana/web3.js";
import { describe, expect, it } from "bun:test";
import { Calculator } from "../target/types/calculator";

describe("calculator", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.calculator as Program<Calculator>;

  const calculator = Keypair.generate();

  const authority = provider.wallet.publicKey;

  async function fetchValue(): Promise<number> {
    const state = await program.account.calculatorState.fetch(calculator.publicKey);
    return state.value;
  }

  it("initialize sets the starting value and authority", async () => {
    await program.methods
      .initialize(5)
      .accountsPartial({
        authority,
        calculator: calculator.publicKey,
      })
      .signers([calculator])
      .rpc();

    const state = await program.account.calculatorState.fetch(calculator.publicKey);

    expect(state.value).toBe(5);
    expect(state.authority.equals(authority)).toBe(true);
  });

  it("double multiplies the value by 2", async () => {
    await program.methods
      .double()
      .accountsPartial({
        calculator: calculator.publicKey,
        authority,
      })
      .rpc();

    expect(await fetchValue()).toBe(10);
  });

  it("add increases the value", async () => {
    await program.methods
      .add(7)
      .accountsPartial({
        calculator: calculator.publicKey,
        authority,
      })
      .rpc();

    expect(await fetchValue()).toBe(17);
  });

  it("sub decreases the value", async () => {
    await program.methods
      .sub(2)
      .accountsPartial({
        calculator: calculator.publicKey,
        authority,
      })
      .rpc();

    expect(await fetchValue()).toBe(15);
  });

  it("halve divides the value by 2 (integer division)", async () => {
    await program.methods
      .halve()
      .accountsPartial({
        calculator: calculator.publicKey,
        authority,
      })
      .rpc();

    expect(await fetchValue()).toBe(7);
  });

  it("sub below zero fails with Underflow", async () => {
    let failed = false;
    try {
      await program.methods
        .sub(1000)
        .accountsPartial({
          calculator: calculator.publicKey,
          authority,
        })
        .rpc();
    } catch (err: any) {
      failed = true;
      expect(err.error.errorCode.number).toBe(6001); 
    }
    expect(failed).toBe(true);
    expect(await fetchValue()).toBe(7);
  });

  it("a different wallet cannot modify the calculator", async () => {
    const intruder = Keypair.generate();

    let failed = false;
    try {
      await program.methods
        .double()
        .accountsPartial({
          calculator: calculator.publicKey,
          authority: intruder.publicKey,
        })
        .signers([intruder])
        .rpc();
    } catch {
      failed = true;
    }
    expect(failed).toBe(true);

    expect(await fetchValue()).toBe(7);
  });
});

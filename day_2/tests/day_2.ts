import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day2 } from "../target/types/day_2";

describe("day_2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day2 as Program<Day2>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(new anchor.BN(908), new anchor.BN(898))
      .rpc();

    const tx2 = await program.methods
      .stringpassing("Hello from the tests")
      .rpc();
    console.log("Your transaction signatures", tx, tx2);
  });

  it("Array test", async () => {
    const tx = await program.methods
      .array([new anchor.BN(777), new anchor.BN(888)])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Underflow Test", async () => {
    const tx = await program.methods
      .checkedoperations(new anchor.BN(0), new anchor.BN(1))
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Adds two numbers", async () => {
    const tx = await program.methods
      .add(new anchor.BN(1), new anchor.BN(1))
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Subtracts two numbers", async () => {
    const tx = await program.methods
      .sub(new anchor.BN(2), new anchor.BN(1))
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Multiplies two numbers", async () => {
    const tx = await program.methods
      .mul(new anchor.BN(2), new anchor.BN(2))
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Divides two numbers", async () => {
    const tx = await program.methods
      .div(new anchor.BN(4), new anchor.BN(2))
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Returns the square root of a number", async () => {
    const tx = await program.methods.sqrt(10).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Returns the log10 of a number", async () => {
    const tx = await program.methods.log10(10).rpc();
    console.log("Your transaction signature", tx);
  });
});

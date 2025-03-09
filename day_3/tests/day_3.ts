import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { Day3 } from "../target/types/day_3";

describe("day_3", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day3 as Program<Day3>;

  it("Call boaty mcboatface", async () => {
    // Add your test here.
    const tx = await program.methods.boatyMcBoatface(new BN(20)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("should add", async () => {
    // Add your test here.
    const tx = await program.methods.add(new BN(20), new BN(10)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("should subtract", async () => {
    // Add your test here.
    const tx = await program.methods.sub(new BN(20), new BN(10)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Should mul", async () => {
    const tx = await program.methods
      .mul(new anchor.BN(10), new anchor.BN(3))
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Should div", async () => {
    const tx = await program.methods
      .div(new anchor.BN(10), new anchor.BN(3))
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Should mod", async () => {
    const tx = await program.methods
      .modulo(new anchor.BN(10), new anchor.BN(3))
      .rpc();
    console.log("Your transaction signature", tx);
  });
});

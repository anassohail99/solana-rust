import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { Day6 } from "../target/types/day_6";

describe("day_6", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day6 as Program<Day6>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Age checker", async () => {
    // Add your test here.
    const tx = await program.methods.ageChecker(new anchor.BN(35)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Age checker 2", async () => {
    // Add your test here.
    const tx = await program.methods.ageChecker2(new anchor.BN(6)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("looping", async () => {
    // Add your test here.
    const tx = await program.methods.looping().rpc();
    console.log("Your transaction signature", tx);
  });

  it("fixed array", async () => {
    // Add your test here.
    const tx = await program.methods.fixedArray().rpc();
    console.log("Your transaction signature", tx);
  });

  it("dynamic array", async () => {
    // Add your test here.
    const tx = await program.methods.dynamicArray().rpc();
    console.log("Your transaction signature", tx);
  });

  it("hash map", async () => {
    // Add your test here.
    const tx = await program.methods.hashMap("name", "Anas").rpc();
    console.log("Your transaction signature", tx);
  });

  it("structs", async () => {
    // Add your test here.
    const tx = await program.methods.structs("Anas", new BN(26)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("constants", async () => {
    // Add your test here.
    const tx = await program.methods.constants().rpc();
    console.log("Your transaction signature", tx);
  });

  it("type_cast", async () => {
    // Add your test here.
    const tx = await program.methods.typeCasting().rpc();
    console.log("Your transaction signature", tx);
  });
});

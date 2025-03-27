import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BasicStorage } from "../target/types/basic_storage";
import { assert } from "chai";

describe("basic_storage", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.BasicStorage as Program<BasicStorage>;
  const admin = provider.wallet.publicKey;

  // Generate a new keypair for the my_storage account
  const myStorageKeypair = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        myStorage: myStorageKeypair.publicKey,
      })
      .signers([myStorageKeypair])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Is initial value 0", async () => {
    const storageVariable = await program.account.myStorage.fetch(
      myStorageKeypair.publicKey
    );
    assert.equal(storageVariable.x.toNumber(), 0);
  });

  it("should increase the variable", async () => {
    await program.methods
      .increase()
      .accounts({
        myStorage: myStorageKeypair.publicKey,
      })
      .rpc();

    const storageVariable = await program.account.myStorage.fetch(
      myStorageKeypair.publicKey
    );

    assert.equal(storageVariable.x.toNumber(), 1);
  });

  it("should decrease the variable", async () => {
    await program.methods
      .increase()
      .accounts({
        myStorage: myStorageKeypair.publicKey,
      })
      .rpc();

    let storageVariable = await program.account.myStorage.fetch(
      myStorageKeypair.publicKey
    );

    assert.equal(storageVariable.x.toNumber(), 2);

    await program.methods
      .decrease()
      .accounts({
        myStorage: myStorageKeypair.publicKey,
      })
      .rpc();

    storageVariable = await program.account.myStorage.fetch(
      myStorageKeypair.publicKey
    );

    assert.equal(storageVariable.x.toNumber(), 1);
  });
});

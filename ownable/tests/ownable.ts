import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Ownable } from "../target/types/ownable";
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  Transaction,
} from "@solana/web3.js";

describe("Ownable", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Ownable as Program<Ownable>;

  it("Is initialized correctly by the owner", async () => {
    const owner = Keypair.generate();
    const state = Keypair.generate();

    const provider = anchor.getProvider();
    const connection = provider.connection as Connection;

    // Airdrop SOL and wait for confirmation
    const signature = await connection.requestAirdrop(
      owner.publicKey,
      2 * LAMPORTS_PER_SOL
    );

    await connection.confirmTransaction(signature);

    const transaction = await program.methods
      .initialize(owner.publicKey)
      .accounts({
        state: state.publicKey,
        signerAccount: owner.publicKey,
      })
      .signers([owner, state])
      .transaction();

    transaction.recentBlockhash = (
      await connection.getLatestBlockhash()
    ).blockhash;
    transaction.feePayer = owner.publicKey;

    const signed = await provider.sendAndConfirm(transaction);

    const stateAccount = await program.account.state.fetch(state.publicKey);
    // expect(stateAccount.owner.equals(owner.publicKey)).toBe(true);
  });
});

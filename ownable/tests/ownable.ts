import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Ownable } from "../target/types/ownable";

describe("Ownable", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Ownable as Program<Ownable>;
  const signer2 = anchor.web3.Keypair.generate();
  const signer3 = anchor.web3.Keypair.generate();

  it("Is signed by a single signer", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        signer1: program.provider.publicKey,
        signer2: signer2.publicKey,
        signer3: signer3.publicKey,
      })
      .signers([signer2, signer3])
      .rpc();

    console.log("The signer1: ", program.provider.publicKey.toBase58());
  });

  it("Is called by the owner", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        signer1: program.provider.publicKey,
      })
      .rpc();

    console.log("Transaction hash:", tx);
  });
});

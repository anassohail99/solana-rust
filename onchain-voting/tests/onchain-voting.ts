import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OnchainVoting } from "../target/types/onchain_voting";

describe("onchain-voting", () => {
  const provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  anchor.setProvider(provider);
  const program = anchor.workspace.OnchainVoting as Program<OnchainVoting>;
  let voteBank = anchor.web3.Keypair.generate();
  it("Creating vote bank for public to vote", async () => {
    const tx = await program.methods
      .initOnchainVoting()
      .accounts({
        voteAccount: voteBank.publicKey,
      })
      .signers([voteBank])
      .rpc();
    console.log("TxHash ::", tx);
    let voteBankData = await program.account.voteBank.fetch(voteBank.publicKey);
    console.log("Initialized? ", voteBankData.isOpenToVote);
  });

  it("Vote for GM", async () => {
    const tx = await program.methods
      .giveVote({ gm: {} })
      .accounts({
        voteAccount: voteBank.publicKey,
      })
      .rpc();
    console.log("TxHash ::", tx);

    let voteBankData = await program.account.voteBank.fetch(voteBank.publicKey);
    console.log(`Total GMs :: ${voteBankData.gm}`);
    console.log(`Total GNs :: ${voteBankData.gn}`);
  });

  it("Vote for GN", async () => {
    const tx = await program.methods
      .giveVote({ gn: {} })
      .accounts({
        voteAccount: voteBank.publicKey,
      })
      .rpc();
    console.log("TxHash ::", tx);

    let voteBankData = await program.account.voteBank.fetch(voteBank.publicKey);
    console.log(`Total GMs :: ${voteBankData.gm}`);
    console.log(`Total GNs :: ${voteBankData.gn}`);
  });
});

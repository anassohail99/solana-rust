import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { Crowdfund } from "../target/types/crowdfund";
import { assert } from "chai";

describe("crowdfund - Basic Tests", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Crowdfund as Program<Crowdfund>;
  const admin = provider.wallet.publicKey;

  // Generate PDA for the campaign
  const [campaignPDA, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("CROWDFUND"), admin.toBuffer()],
    program.programId
  );

  // Shared campaign details
  const campaignDetails = {
    name: "Test Campaign",
    description: "A test crowdfunding campaign",
    targetAmount: new anchor.BN(2 * LAMPORTS_PER_SOL),
    projectUrl: "https://example.com/project",
    progressUpdateUrl: "https://example.com/updates",
    projectImageUrl: "https://example.com/image.jpg",
    category: "Tech",
  };

  it("create_campaign: should create a campaign", async () => {
    await program.methods
      .createCampaign(
        campaignDetails.name,
        campaignDetails.description,
        campaignDetails.targetAmount,
        campaignDetails.projectUrl,
        campaignDetails.progressUpdateUrl,
        campaignDetails.projectImageUrl,
        campaignDetails.category
      )
      .accounts({
        user: admin,
      })
      .rpc();

    const campaignAccount = await program.account.campaign.fetch(campaignPDA);
    assert.equal(
      campaignAccount.admin.toBase58(),
      admin.toBase58(),
      "Admin should match"
    );
    assert.equal(
      campaignAccount.name,
      campaignDetails.name,
      "Name should match"
    );
    assert.equal(
      campaignAccount.amountDonated.toString(),
      "0",
      "Amount donated should be 0"
    );
  });

  it("donate: should donate to the campaign", async () => {
    await program.methods
      .createCampaign(
        campaignDetails.name,
        campaignDetails.description,
        campaignDetails.targetAmount,
        campaignDetails.projectUrl,
        campaignDetails.progressUpdateUrl,
        campaignDetails.projectImageUrl,
        campaignDetails.category
      )
      .accounts({
        user: admin,
      })
      .rpc();

    const donationAmount = new anchor.BN(LAMPORTS_PER_SOL / 2);
    await program.methods
      .donate(donationAmount)
      .accounts({
        campaign: campaignPDA,
        user: admin,
      })
      .rpc();

    const campaignAccount = await program.account.campaign.fetch(campaignPDA);
    assert.equal(
      campaignAccount.amountDonated.toString(),
      donationAmount.toString(),
      "Amount donated should match"
    );
  });

  it("withdraw: should withdraw from the campaign as admin", async () => {
    await program.methods
      .createCampaign(
        campaignDetails.name,
        campaignDetails.description,
        campaignDetails.targetAmount,
        campaignDetails.projectUrl,
        campaignDetails.progressUpdateUrl,
        campaignDetails.projectImageUrl,
        campaignDetails.category
      )
      .accounts({
        user: admin,
      })
      .rpc();

    await program.methods
      .donate(new anchor.BN(LAMPORTS_PER_SOL))
      .accounts({
        campaign: campaignPDA,
        user: admin,
      })
      .rpc();

    const withdrawAmount = new anchor.BN(LAMPORTS_PER_SOL / 2);
    await program.methods
      .withdraw(withdrawAmount)
      .accounts({
        campaign: campaignPDA,
        user: admin,
      })
      .rpc();

    const campaignAccount = await program.account.campaign.fetch(campaignPDA);
    assert.equal(
      campaignAccount.amountWithdrawn.toString(),
      withdrawAmount.toString(),
      "Amount withdrawn should match"
    );
  });

  it("get_campaign: should succeed for admin", async () => {
    await program.methods
      .createCampaign(
        campaignDetails.name,
        campaignDetails.description,
        campaignDetails.targetAmount,
        campaignDetails.projectUrl,
        campaignDetails.progressUpdateUrl,
        campaignDetails.projectImageUrl,
        campaignDetails.category
      )
      .accounts({
        user: admin,
      })
      .rpc();

    await program.methods
      .getCampaign()
      .accounts({
        campaign: campaignPDA,
      })
      .rpc();

    assert.ok(true, "get_campaign should succeed for admin");
  });
});

use anchor_lang::prelude::*;

declare_id!("4FuaVMkVCovjDe1Fs5Whhbd9HcX8D4gASXoi92ehQt2i");

#[program]
pub mod onchain_voting {
    use super::*;

    pub fn init_onchain_voting(_ctx: Context<InitOnChainVoting>) -> Result<()> {
        _ctx.accounts.vote_account.is_open_to_vote = true;
        msg!("Voting is open");
        Ok(())
    }

    pub fn give_vote(ctx: Context<GiveVote>, vote: VoteType) -> Result<()> {
        match vote {
            VoteType::GM => {
                msg!("Vote for GM");
                ctx.accounts.vote_account.gm += 1;
            }
            VoteType::GN => {
                msg!("Vote for GN");
                ctx.accounts.vote_account.gn += 1;
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitOnChainVoting<'info> {
    #[account( init,payer= signer,space=8+1+8+8)]
    vote_account: Account<'info, VoteBank>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GiveVote<'info> {
    #[account(mut)]
    pub vote_account: Account<'info, VoteBank>,
    pub signer: Signer<'info>,
}

#[account]
#[derive(Default)]
pub struct VoteBank {
    is_open_to_vote: bool,
    gm: u64,
    gn: u64,
}
#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum VoteType {
    GM,
    GN,
}

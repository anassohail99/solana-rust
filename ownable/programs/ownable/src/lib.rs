// #![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("6WVVVm5etQGQVfSj1fNve6Ne9snZximcrXZtR91MLVAc");

#[program]
pub mod ownable {
    use super::*;

    #[access_control(check(&ctx))]
    pub fn initialize(ctx: Context<OnlyOwner>, initial_owner: Pubkey) -> Result<()> {
        let owner_account = &mut ctx.accounts.state;
        owner_account.owner = initial_owner;
        msg!("Owner initialized: {}", initial_owner);
        Ok(())
    }
}

fn check(ctx: &Context<OnlyOwner>) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.signer_account.key(),
        ctx.accounts.state.owner,
        OnlyOwnerError::NotOwner
    );
    Ok(())
}

#[access_control(check(&ctx))]
fn update_owner(ctx: &Context<OnlyOwner>, _newowner: Pubkey) -> Result<()> {
    require_keys_eq!(
        _newowner,
        ctx.accounts.state.owner,
        SameOwnerError::SameOwner
    );

    Ok(())
}

#[derive(Accounts)]
pub struct OnlyOwner<'info> {
    #[account(init, payer = signer_account, space = 8 + 32)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub signer_account: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct State {
    pub owner: Pubkey,
}

#[error_code]
pub enum OnlyOwnerError {
    #[msg("Only owner can call this function!")]
    NotOwner,
}
#[error_code]
pub enum SameOwnerError {
    #[msg("Owner needs to be another address!")]
    SameOwner,
}

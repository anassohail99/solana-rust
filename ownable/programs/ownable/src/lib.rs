#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("6WVVVm5etQGQVfSj1fNve6Ne9snZximcrXZtR91MLVAc");

const OWNER: &str = "H26PJXPgY7tbxiSxEzLuSwbqA2Jaz1YWfoJPfEENWi4w";
#[program]
pub mod ownable {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let the_signer1: &mut Signer = &mut ctx.accounts.signer1;
        let the_signer2: &mut Signer = &mut ctx.accounts.signer2;
        let the_signer3: &mut Signer = &mut ctx.accounts.signer3;

        msg!("The signer1: {:?}", the_signer1.key);
        msg!("The signer2: {:?}", the_signer2.key);
        msg!("The signer3: {:?}", the_signer3.key);

        Ok(())
    }

    #[access_control(check_owner(&ctx))]
    pub fn only_owner(ctx: Context<OnlyOwner>) -> Result<()> {
        msg!("The owner: {:?}", *ctx.accounts.signer_account.key);

        Ok(())
    }
}

fn check_owner(ctx: &Context<OnlyOwner>) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.signer_account.key(),
        OWNER.parse::<Pubkey>().unwrap(),
        OnlyOwnerError::NotOwner
    );
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer1: Signer<'info>,
    pub signer2: Signer<'info>,
    pub signer3: Signer<'info>,
}
#[derive(Accounts)]
pub struct OnlyOwner<'info> {
    signer_account: Signer<'info>,
}

#[error_code]
pub enum OnlyOwnerError {
    #[msg("Only owner can call this function!")]
    NotOwner,
}

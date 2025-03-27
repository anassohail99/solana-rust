use anchor_lang::prelude::*;

declare_id!("HQ2c8yzBy4uAwuicum5KfNn9B3nYiJkWxvseukRvcvYb");

#[program]
pub mod basic_storage {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let my_storage = &mut ctx.accounts.my_storage;
        my_storage.x = 0;
        Ok(())
    }

    pub fn increase(ctx: Context<Increase>) -> Result<()> {
        ctx.accounts.my_storage.x += 1;
        Ok(())
    }

    pub fn decrease(ctx: Context<Decrease>) -> Result<()> {
        if ctx.accounts.my_storage.x == 0 {
            return Err(SubtractFromZero::ZeroSubtract.into());
        }
        ctx.accounts.my_storage.x -= 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = signer, 
        space = 8 + 8 // Discriminator (8) + u64 (8)
    )]
    pub my_storage: Account<'info, MyStorage>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increase<'info> {
    #[account(mut)]
    pub my_storage: Account<'info, MyStorage>,
}

#[derive(Accounts)]
pub struct Decrease<'info> {
    #[account(mut)]
    pub my_storage: Account<'info, MyStorage>,
}

#[account]
pub struct MyStorage {
    pub x: u64,
}

#[error_code]
pub enum SubtractFromZero {
    #[msg("Cannot Subtract from Zero")]
    ZeroSubtract,
}
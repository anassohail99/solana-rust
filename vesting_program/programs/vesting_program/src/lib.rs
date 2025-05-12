use anchor_lang::prelude::*;
use anchor_spl::token::*;

declare_id!("CBuJv5A5V2obuDJkvTsdRRyMbFDXKBhdysvbCAxvdCxV");

const ANCHOR_DISCRIMINATOR: usize = 8;

#[program]
pub mod vesting_program {
    use super::*;

    pub fn create_vesting_account(ctx: Context<CreateVestingAccount>) -> Result<()> {
        let vesting_account: &mut Pubkey = &mut ctx.accounts.vesting_account;

        // vesting_account.owner = ctx.accounts.signer.key();              // Store the creator's pubkey
        // vesting_account.mint = ctx.accounts.mint.key();                 // Store the token mint's pubkey
        // vesting_account.treasury_token_account = ctx.accounts.treasury_token_account.key(); // Store the new treasury PDA's pubkey
        // vesting_account.company_name = company_name;                    // Store the provided company name
        // vesting_account.treasury_bump = ctx.bumps.treasury_token_account; // Store the bump for the treasury PDA
        // vesting_account.vesting_bump = ctx.bumps.vesting_account;       // Store the bump for the vesting account PDA

        *ctx.accounts.vesting_account = VestingAccount{
            owner: ctx.accounts.signer.key(),
            mint: ctx.accounts.mint.key(),
            treasury_token_account: ctx.accounts.treasury_token_account.key(),
            company_name: company_name,
            treasury_bump: ctx.bumps.treasury_token_account,
            vesting_bump: ctx.bumps.vesting_account,
        };

        msg!("Vesting account created for company: {}", vesting_account.company_name);
        msg!("Owner: {}", vesting_account.owner);
        msg!("Mint: {}", vesting_account.mint);
        msg!("Treasury ATA (PDA): {}", vesting_account.treasury_token_account);

        Ok(()) 

        
    }
}

#[derive(InitSpace)] // Automatically calculates space for the account
pub struct VestingAccount {
    pub owner: Pubkey, // The public key of the company/entity that created this
    pub mint: Pubkey,  // The public key of the SPL Token to be vested
    pub treasury_token_account: Pubkey, // The PDA token account holding all tokens for this vesting setup
    #[max_len(50)] // Max length for the company name string
    pub company_name: String, // Name of the company, used as a seed for PDAs
    pub treasury_bump: u8,              // Bump seed for the treasury_token_account PDA
    pub vesting_bump: u8,               // Bump seed for this VestingAccount PDA
}

#[account]
#[derive(InitSpace)] // Automatically calculates space
pub struct EmployeeAccount {
    pub beneficiary: Pubkey, // Public key of the employee receiving the tokens
    pub start_time: i64,     // Unix timestamp when vesting begins
    pub end_time: i64,       // Unix timestamp when vesting ends
    pub cliff_time: i64,     // Unix timestamp before which no tokens can be claimed (the cliff)
    pub vesting_account: Pubkey, // Links back to the main VestingAccount
    pub total_amount: u64,   // Total number of tokens allocated to this employee
    pub total_withdrawn: u64, // Number of tokens already claimed by the employee
    pub bump: u8,            // Bump seed for this EmployeeAccount PDA
}

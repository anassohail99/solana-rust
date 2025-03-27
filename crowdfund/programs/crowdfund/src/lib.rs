#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, pubkey::Pubkey,
    system_instruction,
};

declare_id!("5h45rfdZGv4xS4PB8s1b7RJgdBbDVRDUnUdreE1ZSrfs");

#[program]
pub mod crowdfund {
    use super::*;

    pub fn create_campaign(
        ctx: Context<CreateCampaign>,
        name: String,
        description: String,
        target_amount: u64,
        project_url: String,
        progress_update_url: String,
        project_image_url: String,
        category: String,
    ) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign;

        // Set data only if the account is newly initialized
        if campaign.admin == Pubkey::default() {
            campaign.admin = *ctx.accounts.user.key;
            campaign.name = name;
            campaign.description = description;
            campaign.target_amount = target_amount;
            campaign.project_url = project_url;
            campaign.progress_update_url = progress_update_url;
            campaign.project_image_url = project_image_url;
            campaign.category = category;
            campaign.amount_donated = 0;
            campaign.amount_withdrawn = 0;
        }

        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> ProgramResult {
        let transfer_result = invoke(
            &system_instruction::transfer(
                &ctx.accounts.user.key(),
                &ctx.accounts.campaign.key(),
                amount,
            ),
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.campaign.to_account_info(),
            ],
        );

        if let Err(e) = transfer_result {
            return Err(e.into());
        }

        (&mut ctx.accounts.campaign).amount_donated += amount;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign;
        let user = &ctx.accounts.user;

        if campaign.admin != *user.key {
            return Err(ProgramError::IncorrectProgramId);
        }

        let rent_balance = Rent::get()?.minimum_balance(campaign.to_account_info().data_len());
        if **campaign.to_account_info().lamports.borrow() - rent_balance < amount {
            return Err(ProgramError::InsufficientFunds);
        }

        **campaign.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;
        (&mut ctx.accounts.campaign).amount_withdrawn += amount;
        Ok(())
    }

    pub fn get_campaign(ctx: Context<GetCampaign>) -> ProgramResult {
        let campaign = &ctx.accounts.campaign;
        let user = &ctx.accounts.user;
        if campaign.admin != *user.key {
            return Err(ProgramError::IncorrectProgramId);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateCampaign<'info> {
    #[account(
        init_if_needed,
        payer = user,
        space = 9000,
        seeds = [b"CROWDFUND".as_ref(), user.key().as_ref()],
        bump
    )]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetCampaign<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct Campaign {
    pub admin: Pubkey,
    pub name: String,
    pub description: String,
    pub target_amount: u64,
    pub project_url: String,
    pub progress_update_url: String,
    pub project_image_url: String,
    pub category: String,
    pub amount_donated: u64,
    pub amount_withdrawn: u64,
}

pub enum CampaignStatus {}

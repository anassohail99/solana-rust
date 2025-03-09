use anchor_lang::prelude::*;

declare_id!("65DffTdzK5sRsjtbHS3Mgqa5shf5fJouLpTQC1iJNc42");

#[program]
pub mod day_4 {
    use super::*;

    pub fn limit_ranger(_ctx: Context<LimitRange>, a: u64) -> Result<()> {
        require!(a >= 10, MyError::AisTooSmall);
        require!(a <= 100, MyError::AisTooBig);

        msg!("Result = {}", a);
        Ok(())
    }

    pub fn func(_ctx: Context<LimitRange>) -> Result<()> {
        msg!("Will this print?");
        Ok(())
        // return err!(MyError::AlwaysErrors);
    }
}

#[derive(Accounts)]
pub struct LimitRange {}
#[error_code]
pub enum MyError {
    #[msg("a is too small")]
    AisTooSmall,
    #[msg("a is too big")]
    AisTooBig,
    #[msg("Always errors")]
    AlwaysErrors,
}

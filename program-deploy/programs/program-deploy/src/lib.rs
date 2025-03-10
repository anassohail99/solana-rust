use anchor_lang::prelude::*;

declare_id!("J3QEZPTvVLtqgGoGYv5xjoX1uCGFsDFvS4jd5pfSBo4s");

#[program]
pub mod program_deploy {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

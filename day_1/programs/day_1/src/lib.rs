use anchor_lang::prelude::*;

declare_id!("2f8EZBCWT2CDZxjzJsFLsdYCxLe9ZxzSq1q25jqHCSru");

#[program]
pub mod day_1 {
    use super::*;

    pub fn helloWorldFunc(ctx: Context<helloWorldFunc>) -> Result<()> {
        msg!("Hello world!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct helloWorldFunc {}

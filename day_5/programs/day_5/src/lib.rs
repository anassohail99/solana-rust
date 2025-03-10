use anchor_lang::prelude::*;

declare_id!("HwRvwkvwGuy4vueSQfM32Bh5yGZJyhH86UdoQEtFtKor");

#[program]
pub mod day_5 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        let msg = "Greetings 2";
        msg!("Greetings from: {}", msg);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

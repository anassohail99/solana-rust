use anchor_lang::prelude::*;

declare_id!("nuvc2QeZHQtR7HrX96zPdgdbAcDQwWmQFkNhqiQxHcJ");

#[program]
pub mod day_3 {
    use super::*;

    pub fn boaty_mc_boatface(ctx: Context<BoatyMcBoatface>, a: u64) -> Result<()> {
        msg!("Greetings from: {} {}", ctx.program_id, a);
        Ok(())
    }

    pub fn add(_ctx: Context<BoatyMcBoatface>, a: u64, b: u64) -> Result<()> {
        let sum: u64 = a + b;
        msg!("The sum of {} and {} is {}", a, b, sum);
        Ok(())
    }

    pub fn sub(_ctx: Context<BoatyMcBoatface>, a: u64, b: u64) -> Result<()> {
        let diff: u64 = a + b;
        msg!("The difference of {} and {} is {}", a, b, diff);
        Ok(())
    }

    pub fn mul(_ctx: Context<BoatyMcBoatface>, a: u64, b: u64) -> Result<()> {
        let product = a * b;
        msg!("Product is {}", product);
        Ok(())
    }

    pub fn div(_ctx: Context<BoatyMcBoatface>, a: u64, b: u64) -> Result<()> {
        let quotient = a / b;
        msg!("Quotient is {}", quotient);
        Ok(())
    }

    pub fn modulo(_ctx: Context<BoatyMcBoatface>, a: u64, b: u64) -> Result<()> {
        let remainder = a % b;
        msg!("Remainder is {}", remainder);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BoatyMcBoatface {}

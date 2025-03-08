use anchor_lang::prelude::*;

declare_id!("FEex5uiNLFLQn3rnpYSvQ4wT5VSyzmfrhKvD9A4XLnPh");

#[program]
pub mod day_2 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        msg!("You sent {} and {}", a, b);
        Ok(())
    }

    pub fn stringpassing(_ctx: Context<Initialize>, msg: String) -> Result<()> {
        msg!("You pass this msg {} ", msg);
        Ok(())
    }

    pub fn array(_ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
        msg!("Your array {:?}", arr);
        Ok(())
    }

    pub fn checkedoperations(_ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let x: u64 = a - b;
        msg!("Your number {}", x);

        Ok(())
    }

    pub fn add(_ctx: Context<Initialize>, y: u64, z: u64) -> Result<()> {
        let x: u64 = y.checked_add(z).unwrap();
        msg!("Add {:?}", x);
        Ok(())
    }

    pub fn sub(_ctx: Context<Initialize>, y: u64, z: u64) -> Result<()> {
        let x: u64 = y.checked_sub(z).unwrap();
        msg!("Subtract {:?}", x);
        Ok(())
    }

    pub fn mul(_ctx: Context<Initialize>, y: u64, z: u64) -> Result<()> {
        let x: u64 = y.checked_mul(z).unwrap();
        msg!("Multiply {:?}", x);
        Ok(())
    }

    pub fn div(_ctx: Context<Initialize>, y: u64, z: u64) -> Result<()> {
        let x: u64 = y.checked_div(z).unwrap();
        msg!("Divide {:?}", x);
        Ok(())
    }

    pub fn sqrt(_ctx: Context<Initialize>, y: f32) -> Result<()> {
        let x: f32 = y.sqrt();
        msg!("Square root {:?}", x);
        Ok(())
    }

    pub fn log10(_ctx: Context<Initialize>, y: f32) -> Result<()> {
        let x: f32 = y.log10();
        msg!("log10 {:?}", x);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

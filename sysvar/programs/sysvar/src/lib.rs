use anchor_lang::prelude::*;

declare_id!("8VgUFQCDysmBzQipiXTEQEHY548aQqgxCotEsqwyiRah");

#[program]
pub mod sysvar {

    use super::*;
    use chrono::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        let clock: Clock = Clock::get()?;
        msg!(
            "Block timestamp: {}",
            // Get block.timestamp
            clock.unix_timestamp,
        );
        Ok(())
    }

    pub fn get_day_of_the_week(_ctx: Context<Initialize>) -> Result<()> {
        let clock: Clock = Clock::get()?;
        let time_stamp: i64 = clock.unix_timestamp; // current timestamp

        let date_time = chrono::DateTime::from_timestamp(time_stamp, 0).unwrap();
        let day_of_the_week = date_time.weekday();

        msg!("Week day is: {}", day_of_the_week);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

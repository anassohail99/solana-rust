use anchor_lang::prelude::*;

declare_id!("Ez8vU8jStBVC6JLiFjuCU4DmsYZiGAaUsVkNSz4njjs1");

#[program]
pub mod day_6 {
    use super::*;
    use std::collections::HashMap;

    const MEANING_OF_LIFE_AND_EXISTENCE: u64 = 42;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn age_checker(ctx: Context<Initialize>, age: u64) -> Result<()> {
        let result = if age >= 18 {
            "You are 18 years old or above"
        } else {
            "You are below 18 years old"
        };
        msg!("{:?}", result);
        Ok(())
    }

    pub fn age_checker2(ctx: Context<Initialize>, age: u64) -> Result<()> {
        match age {
            1 => {
                msg!(" Age is 1");
            }
            2 | 3 => {
                msg!("Age is either 2 or 3");
            }

            4..=6 => {
                msg!("Age is in between 4 to 6");
            }
            _ => {
                msg!("The age is something else");
            }
        }
        Ok(())
    }

    pub fn looping(ctx: Context<Initialize>) -> Result<()> {
        for i in 0..10 {
            msg!("This is loop {}", i);
        }

        for i in (0..10).step_by(2) {
            // do something...

            msg!("This is a stepper loop {}", i);
        }

        Ok(())
    }

    pub fn fixed_array(ctx: Context<Initialize>) -> Result<()> {
        let my_array: [u32; 5] = [10, 20, 50, 60, 70];
        let first_element = my_array[0];
        let third_element = my_array[2];

        let mut mutable_array: [u32; 3] = [100, 200, 300];
        mutable_array[1] = 250;

        msg!("Array {} {}", first_element, third_element);
        Ok(())
    }

    pub fn dynamic_array(ctx: Context<Initialize>) -> Result<()> {
        let mut dynamic_array: Vec<u32> = Vec::new();
        dynamic_array.push(10);
        dynamic_array.push(20);
        dynamic_array.push(30);

        let first_element = dynamic_array[0];
        let third_element = dynamic_array[2];

        msg!("Third element = {}", third_element);

        Ok(())
    }

    pub fn hash_map(ctx: Context<Initialize>, key: String, value: String) -> Result<()> {
        let mut my_map = HashMap::new();

        my_map.insert(key.to_string(), value.to_string());

        msg!("My Hashmap value is {}", my_map[&key]);
        Ok(())
    }

    pub fn structs(_ctx: Context<Initialize>, name: String, age: u64) -> Result<()> {
        struct Person {
            my_name: String,
            my_age: u64,
        }

        let mut person1: Person = Person {
            my_name: name,
            my_age: age,
        };

        msg!("{} is {} years old", person1.my_name, person1.my_age);

        person1.my_name = "Ali".to_string();
        person1.my_age = 18;

        msg!("{} is {} years old", person1.my_name, person1.my_age);

        Ok(())
    }

    pub fn constants(ctx: Context<Initialize>) -> Result<()> {
        msg!(&format!(
            "Answer to the ultimate question: {}",
            MEANING_OF_LIFE_AND_EXISTENCE
        ));
        Ok(())
    }

    pub fn type_casting(ctx: Context<Initialize>) -> Result<()> {
        let dynamic_array: Vec<u32> = Vec::from([1, 2, 3, 4, 5, 6]);
        let len = dynamic_array.len(); // this has type usize

        let another_var: u64 = 5; // this has type u64

        let len_plus_another_var = len as u64 + another_var;

        msg!("The result is {}", len_plus_another_var);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

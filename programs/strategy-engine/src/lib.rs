use anchor_lang::prelude::*;

pub mod logic;

declare_id!("7Xnzrm7QHgLwANg78gBg55DZ8eEaxXzzvf9BSMtKdUcT");

#[program]
pub mod strategy_engine {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

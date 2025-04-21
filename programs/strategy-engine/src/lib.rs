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

// #[cfg(test)]
// mod test_smoke {
//     use super::logic::conditions::*;

//     #[test]
//     fn test_force_include() {
//         // Just force the module to compile
//         let _ = ConditionBuilder::price_above(Default::default(), 42).build();
//     }
// }

use crate::logic::actions::ActionTree;
use crate::logic::conditions::ConditionTree;
use crate::logic::conditions::EvaluationContext;
// use crate::logic::parser::actionParser::translate_action_string;
// use crate::logic::parser::conditionParser::translate_condition_string;
// use crate::logic::parser::tokens::*;
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

    // pub fn create_vault(
    //     ctx: Context<CreateVault>,
    //     condition_str: String,
    //     action_str: String,
    //     execute_every_seconds: u64,
    // ) -> Result<()> {
    //     let vault = &mut ctx.accounts.vault;

    //     let condition_tree = translate_condition_string(&condition_str)?;
    //     let strategy = Strategy::new(condition_tree, action_tree, execute_every_seconds);

    //     vault.authority = *ctx.accounts.authority.key;
    //     vault.strategy = strategy;

    //     vault.balance = 0;
    //     vault.last_executed = Clock::get()?.unix_timestamp as u64;
    //     Ok(())
    // }
}

pub fn deposit(ctx: Context<DepositVault>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.balance = vault
        .balance
        .checked_add(amount)
        .ok_or_else(|| error!(ErrorCode::Overflow))?;
    Ok(())
}

pub fn withdraw(ctx: Context<WithdrawVault>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.balance = vault
        .balance
        .checked_sub(amount)
        .ok_or_else(|| error!(ErrorCode::Underflow))?;
    Ok(())
}

pub fn execute_strategy(ctx: Context<ExecuteVault>, ctx_eval: EvaluationContext) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let now = Clock::get()?.unix_timestamp as u64;
    if now - vault.last_executed < vault.strategy.execute_every_seconds {
        return Ok(());
    }
    if vault.strategy.condition_tree.evaluate(&ctx_eval) {
        vault.strategy.action_tree.execute();
        vault.last_executed = now;
    }
    Ok(())
}
// onchain account storing vault data
#[account]
pub struct VaultAccount {
    pub authority: Pubkey,
    pub strategy: Strategy,
    pub balance: u64,
    pub last_executed: u64,
}

#[derive(Accounts)]
pub struct Initialize {}

// ==== Account context types: these `CreateVault`, `DepositVault`, etc., are Rust structs you declare in `lib.rs` alongside your program module. They tell Anchor which accounts to expect and how to derive them.
// Context for creating a new vault PDA
#[derive(Accounts)]
#[instruction(condition_tree: ConditionTree, action_tree: ActionTree, execute_every_seconds: u64)]
pub struct CreateVault<'info> {
    #[account(init, payer = authority, space = 8+32+condition_tree.size() + action_tree.size()+8+8+8, seeds = [b"vault", authority.key().as_ref()], bump)]
    pub vault: Account<'info, VaultAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Deposit into vault (just updates balance)
#[derive(Accounts)]
pub struct DepositVault<'info> {
    #[account(mut, has_one = authority)]
    pub vault: Account<'info, VaultAccount>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct WithdrawVault<'info> {
    #[account(mut, has_one = authority)]
    pub vault: Account<'info, VaultAccount>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ExecuteVault<'info> {
    #[account(mut, has_one=authority)]
    pub vault: Account<'info, VaultAccount>,
    pub authority: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Strategy {
    pub condition_tree: ConditionTree,
    pub action_tree: ActionTree,
    pub execute_every_seconds: u64,
}

impl Strategy {
    pub fn new(
        condition_tree: ConditionTree,
        action_tree: ActionTree,
        execute_every_seconds: u64,
    ) -> Self {
        Self {
            condition_tree,
            action_tree,
            execute_every_seconds,
        }
    }
}
// #[cfg(test)]
// mod test_smoke {
//     use super::logic::conditions::*;

//     #[test]
//     fn test_force_include() {
//         // Just force the module to compile
//         let _ = ConditionBuilder::price_above(Default::default(), 42).build();
//     }
// }

#[error_code]
pub enum ErrorCode {
    #[msg("Overflow when adding to vault balance")]
    Overflow,
    #[msg("Underflow when subtracting from vault balance")]
    Underflow,
}

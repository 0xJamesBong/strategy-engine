use crate::logic::actions::ActionTree;
use crate::logic::conditions::ConditionTree;

use anchor_lang::prelude::*;

use std::collections::HashMap;

use super::conditions::EvaluationContext;

#[derive(Clone, Debug, PartialEq)]
pub struct Strategy {
    pub condition: ConditionTree,
    pub action: ActionTree,
    pub execute_every_seconds: u64,
}

impl Strategy {
    pub fn new(condition: ConditionTree, action: ActionTree, execute_every_seconds: u64) -> Self {
        Self {
            condition,
            action,
            execute_every_seconds,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Vault {
    pub strategy: Strategy,
    pub balance: u64,
}

impl Vault {
    pub fn new(strategy: Strategy) -> Self {
        Self {
            strategy,
            balance: 0,
        }
    }
    // deposits amount into the vault
    pub fn deposit(self, amount: u64) -> Self {
        Self {
            strategy: self.strategy,
            balance: self.balance + amount,
        }
    }
    // withdraws amount from the vault
    pub fn withdraw(self, amount: u64) -> Self {
        Self {
            strategy: self.strategy,
            balance: self.balance - amount,
        }
    }

    pub fn execute(self, ctx: &EvaluationContext) -> bool {
        if self.strategy.condition.evaluate(ctx) {
            let success = self.strategy.action.execute();
            success
        } else {
            false
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_strategy_execute() {
//         let token = Pubkey::default();
//     }
// }

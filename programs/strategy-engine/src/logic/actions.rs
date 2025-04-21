use anchor_lang::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum AtomicAction {
    Buy { token: Pubkey, amount: u64 },
    Sell { token: Pubkey, amount: u64 },
    Borrow { token: Pubkey, amount: u64 },
    Repay { token: Pubkey, amount: u64 },
    Lend { token: Pubkey, amount: u64 },
    Redeem { token: Pubkey, amount: u64 },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Atomic(AtomicAction),
    And(Box<Action>, Box<Action>),
}

impl Action {
    pub fn execute(&self) -> bool {
        match self {
            Action::Atomic(atomic) => match atomic {
                AtomicAction::Buy { token, amount } => {
                    msg!("Buying {} of {}", amount, token);
                    true
                }
                AtomicAction::Sell { token, amount } => {
                    msg!("Selling {} of {}", amount, token);
                    true
                }
                AtomicAction::Borrow { token, amount } => {
                    msg!("Borrowing {} of {}", amount, token);
                    true
                }
                AtomicAction::Repay { token, amount } => {
                    msg!("Repaying {} of {}", amount, token);
                    true
                }
                AtomicAction::Lend { token, amount } => {
                    msg!("Lending {} of {}", amount, token);
                    true
                }
                AtomicAction::Redeem { token, amount } => {
                    msg!("Redeeming {} of {}", amount, token);
                    true
                }
            },
            Action::And(left, right) => left.execute() && right.execute(),
        }
    }
}

pub struct ActionBuilder {
    action: Action,
}

impl ActionBuilder {
    pub fn buy(token: Pubkey, amount: u64) -> Self {
        Self {
            action: Action::Atomic(AtomicAction::Buy { token, amount }),
        }
    }

    pub fn sell(token: Pubkey, amount: u64) -> Self {
        Self {
            action: Action::Atomic(AtomicAction::Sell { token, amount }),
        }
    }

    pub fn borrow(token: Pubkey, amount: u64) -> Self {
        Self {
            action: Action::Atomic(AtomicAction::Borrow { token, amount }),
        }
    }

    pub fn repay(token: Pubkey, amount: u64) -> Self {
        Self {
            action: Action::Atomic(AtomicAction::Repay { token, amount }),
        }
    }

    pub fn lend(token: Pubkey, amount: u64) -> Self {
        Self {
            action: Action::Atomic(AtomicAction::Lend { token, amount }),
        }
    }

    pub fn redeem(token: Pubkey, amount: u64) -> Self {
        Self {
            action: Action::Atomic(AtomicAction::Redeem { token, amount }),
        }
    }

    pub fn and(self, other: ActionBuilder) -> Self {
        Self {
            action: Action::And(Box::new(self.action), Box::new(other.action)),
        }
    }

    pub fn build(self) -> Action {
        self.action
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_builder() {
        let token = Pubkey::new_unique();

        let action_1_prebuilt = ActionBuilder::buy(token, 100);
        let action_2_prebuilt = ActionBuilder::sell(token, 100);
        let action_3_prebuilt = ActionBuilder::borrow(token, 100);
        let action_4_prebuilt = ActionBuilder::repay(token, 100);
        let action_5_prebuilt = ActionBuilder::lend(token, 100);
        let action_6_prebuilt = ActionBuilder::redeem(token, 100);

        let action = action_1_prebuilt
            .and(action_2_prebuilt)
            .and(action_3_prebuilt)
            .and(action_4_prebuilt)
            .and(action_5_prebuilt)
            .and(action_6_prebuilt)
            .build();

        assert_eq!(action.execute(), true);
    }
}

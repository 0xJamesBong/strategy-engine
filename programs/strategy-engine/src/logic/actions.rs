use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum AtomicAction {
    Buy { token: Pubkey, amount: u64 },
    Sell { token: Pubkey, amount: u64 },
    Borrow { token: Pubkey, amount: u64 },
    Repay { token: Pubkey, amount: u64 },
    Lend { token: Pubkey, amount: u64 },
    Redeem { token: Pubkey, amount: u64 },
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum ActionType {
    Atomic(AtomicAction),
    // And(Box<Action>, Box<Action>),
    And { left: u8, right: u8 },
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct ActionNode {
    pub action_type: ActionType,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct ActionTree {
    pub nodes: Vec<ActionNode>,
    pub root_index: u8,
}

impl ActionTree {
    pub fn execute(&self) -> bool {
        self.execute_node(self.root_index)
    }

    pub fn size(&self) -> usize {
        // // 8 bytes for discriminator + 1 byte for root_index + nodes size
        // 8 + 1 + (self.nodes.len() * std::mem::size_of::<ActionNode>())

        let mut buf = Vec::new();
        self.serialize(&mut buf).unwrap();
        buf.len()
    }

    pub fn execute_node(&self, index: u8) -> bool {
        let node = &self.nodes[index as usize];

        match &node.action_type {
            ActionType::Atomic(atomic) => match atomic {
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

            ActionType::And { left, right } => {
                self.execute_node(*left) && self.execute_node(*right)
            }
        }
    }
}

pub struct ActionBuilder {
    nodes: Vec<ActionNode>,
    root_index: u8,
}

impl ActionBuilder {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            root_index: 0,
        }
    }

    fn with_node(mut self, node: ActionNode) -> Self {
        let index: u8 = self.nodes.len() as u8;
        self.nodes.push(node);
        self.root_index = index;
        self
    }

    fn buy(token: Pubkey, amount: u64) -> Self {
        Self::new().with_node(ActionNode {
            action_type: ActionType::Atomic(AtomicAction::Buy { token, amount }),
        })
    }

    pub fn sell(token: Pubkey, amount: u64) -> Self {
        Self::new().with_node(ActionNode {
            action_type: ActionType::Atomic(AtomicAction::Sell { token, amount }),
        })
    }

    pub fn borrow(token: Pubkey, amount: u64) -> Self {
        Self::new().with_node(ActionNode {
            action_type: ActionType::Atomic(AtomicAction::Borrow { token, amount }),
        })
    }

    pub fn repay(token: Pubkey, amount: u64) -> Self {
        Self::new().with_node(ActionNode {
            action_type: ActionType::Atomic(AtomicAction::Repay {
                token: token,
                amount,
            }),
        })
    }

    pub fn lend(token: Pubkey, amount: u64) -> Self {
        Self::new().with_node(ActionNode {
            action_type: ActionType::Atomic(AtomicAction::Lend {
                token: token,
                amount,
            }),
        })
    }

    pub fn redeem(token: Pubkey, amount: u64) -> Self {
        Self::new().with_node(ActionNode {
            action_type: ActionType::Atomic(AtomicAction::Redeem {
                token: token,
                amount,
            }),
        })
    }

    pub fn and(mut self, mut child: Self) -> Self {
        let left: u8 = self.root_index;
        let right: u8 = child.root_index;

        let mut nodes = vec![];
        nodes.append(&mut self.nodes);
        nodes.append(&mut child.nodes);

        let root = nodes.len() as u8;
        nodes.push(ActionNode {
            action_type: ActionType::And { left, right },
        });

        Self {
            nodes,
            root_index: root,
        }
    }

    pub fn build(self) -> ActionTree {
        ActionTree {
            nodes: self.nodes,
            root_index: self.root_index,
        }
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

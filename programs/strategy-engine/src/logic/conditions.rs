use anchor_lang::prelude::*;
use std::collections::HashMap;

pub struct EvaluationContext {
    pub token_prices: HashMap<Pubkey, u64>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum AtomicCondition {
    // Price-based conditions
    PriceAbove { token: Pubkey, price: u64 },
    PriceBelow { token: Pubkey, price: u64 },
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum ConditionType {
    Atomic(AtomicCondition),
    And { left: u8, right: u8 }, // And(Box<Condition>, Box<Condition>),
    Or { left: u8, right: u8 },  // Or(Box<Condition>, Box<Condition>),
    Not { child: u8 },           // Not(Box<Condition>),
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct ConditionNode {
    pub condition_type: ConditionType,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct ConditionTree {
    pub nodes: Vec<ConditionNode>,
    pub root_index: u8,
}

impl ConditionTree {
    pub fn evaluate(&self, ctx: &EvaluationContext) -> bool {
        self.evaluate_node(self.root_index, ctx)
    }

    pub fn size(&self) -> usize {
        // // 8 bytes for discriminator + 1 byte for root_index + nodes size
        // 8 + 1 + (self.nodes.len() * std::mem::size_of::<ConditionNode>())
        let mut buf = Vec::new();
        self.serialize(&mut buf).unwrap();
        buf.len()
    }

    pub fn evaluate_node(&self, index: u8, ctx: &EvaluationContext) -> bool {
        // evaluate this node at index number `index`
        let node = &self.nodes[index as usize];
        match &node.condition_type {
            ConditionType::Atomic(atomic) => match atomic {
                AtomicCondition::PriceAbove { token, price } => {
                    ctx.token_prices.get(token).map_or(false, |p| p > price)
                }
                AtomicCondition::PriceBelow { token, price } => {
                    ctx.token_prices.get(token).map_or(false, |p| p < price)
                }
            },
            ConditionType::And { left, right } => {
                self.evaluate_node(*left, ctx) && self.evaluate_node(*right, ctx)
            }
            ConditionType::Or { left, right } => {
                self.evaluate_node(*left, ctx) || self.evaluate_node(*right, ctx)
            }
            ConditionType::Not { child } => !self.evaluate_node(*child, ctx),
        }
    }
}

#[derive(Clone)]
pub struct ConditionBuilder {
    nodes: Vec<ConditionNode>,
    root_index: u8,
}

impl ConditionBuilder {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            root_index: 0,
        }
    }

    fn with_node(mut self, node: ConditionNode) -> Self {
        let index = self.nodes.len() as u8;
        self.nodes.push(node);
        self.root_index = index;
        self
    }

    pub fn price_above(token: Pubkey, price: u64) -> Self {
        Self::new().with_node(ConditionNode {
            condition_type: ConditionType::Atomic(AtomicCondition::PriceAbove { token, price }),
        })
    }

    pub fn price_below(token: Pubkey, price: u64) -> Self {
        Self::new().with_node(ConditionNode {
            condition_type: ConditionType::Atomic(AtomicCondition::PriceBelow { token, price }),
        })
    }

    pub fn and(mut self, mut other: Self) -> Self {
        /*  store the root indexes of the two subtrees self and other
         * These indces point to the "top" node of each tree, and they will
         * become the left and right children of the new And node
         */
        let left = self.root_index;
        let right = other.root_index;

        /* we create a new nodes vector and append all nodes from self and other into it.
         * append(0 oves the contents from the original vectors into nodes)
         */
        let mut nodes = vec![];
        nodes.append(&mut self.nodes);
        nodes.append(&mut other.nodes);

        /* This determines the index of the new root node (which will be added next).
         * Since indexing starts at 0, the next node will be at position nodes.len().
         */

        let root = nodes.len() as u8;
        nodes.push(ConditionNode {
            condition_type: ConditionType::And { left, right },
        });

        Self {
            nodes,
            root_index: root,
        }
    }

    pub fn or(mut self, mut other: Self) -> Self {
        let left = self.root_index;
        let right = other.root_index;

        let mut nodes = vec![];
        nodes.append(&mut self.nodes);
        nodes.append(&mut other.nodes);

        let root = nodes.len() as u8;
        nodes.push(ConditionNode {
            condition_type: ConditionType::Or { left, right },
        });

        Self {
            nodes,
            root_index: root,
        }
    }

    pub fn not(mut self) -> Self {
        let child = self.root_index;
        let mut nodes = vec![];
        nodes.append(&mut self.nodes);

        let root = nodes.len() as u8;
        nodes.push(ConditionNode {
            condition_type: ConditionType::Not { child },
        });
        Self {
            nodes,
            root_index: root,
        }
    }

    pub fn build(self) -> ConditionTree {
        ConditionTree {
            nodes: self.nodes,
            root_index: self.root_index,
        }
    }
}

// run `cargo clean`` and then `cargo test` to run the tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_condition_builder() {
        let token = Pubkey::default();

        let strategy_1 = ConditionBuilder::not(
            ConditionBuilder::price_above(token, 100)
                .and(ConditionBuilder::price_below(token, 200)),
        );

        let strategy_2 =
            ConditionBuilder::price_above(token, 400).and(ConditionBuilder::price_below(token, 10));

        let strategy_3 = strategy_1.or(strategy_2).build();
    }

    #[test]
    fn test_evaluate_not() {
        let token = Pubkey::new_unique();

        // NOT (price > 100)
        let condition = ConditionBuilder::not(ConditionBuilder::price_above(token, 100)).build();

        let mut prices = HashMap::new();
        prices.insert(token, 150);

        let mut context = EvaluationContext {
            token_prices: prices.clone(),
        };
        assert_eq!(condition.evaluate(&context), false);
        // now change the price to 50
        prices.insert(token, 50);
        context.token_prices = prices;
        assert_eq!(condition.evaluate(&context), true);
    }

    #[test]
    fn test_evaluate_and() {
        let token = Pubkey::new_unique();
        // Condition (price > 100) AND (price < 400)
        let condition_1_prebuilt = ConditionBuilder::price_above(token, 100)
            .and(ConditionBuilder::price_below(token, 400));

        let mut prices = HashMap::new();
        prices.insert(token, 150);

        let mut context = EvaluationContext {
            token_prices: prices.clone(),
        };

        let condition_1 = condition_1_prebuilt.clone().build();
        assert_eq!(condition_1.evaluate(&context), true);

        let condition_2 = ConditionBuilder::not(condition_1_prebuilt).build();
        assert_eq!(condition_2.evaluate(&context), false);
    }

    #[test]
    fn test_evaluate_or() {
        let token = Pubkey::new_unique();
        // Condition (price < 100) OR (price > 400)
        let condition_1_prebuilt =
            ConditionBuilder::price_below(token, 100).or(ConditionBuilder::price_above(token, 400));

        let mut prices = HashMap::new();
        prices.insert(token, 150);

        let mut context = EvaluationContext {
            token_prices: prices.clone(),
        };

        let condition_1 = condition_1_prebuilt.clone().build();
        assert_eq!(condition_1.evaluate(&context), false);

        let condition_2 = ConditionBuilder::not(condition_1_prebuilt).build();
        assert_eq!(condition_2.evaluate(&context), true);
    }

    #[test]
    fn test_evaluate_condition_tree_2() {
        let token = Pubkey::default();
        let mut token_prices = HashMap::new();
        token_prices.insert(token, 150); // Set current price to 150

        let context = EvaluationContext { token_prices };

        // Test case 1: NOT(price > 100 AND price < 200)
        let condition = ConditionBuilder::not(
            ConditionBuilder::price_above(token, 100)
                .and(ConditionBuilder::price_below(token, 200)),
        )
        .build();

        assert!(!condition.evaluate(&context));

        // Test case 2: price > 400 AND price < 10
        let condition2 = ConditionBuilder::price_above(token, 400)
            .and(ConditionBuilder::price_below(token, 10))
            .build();

        assert!(!condition2.evaluate(&context));

        // Test case 3: price > 100
        let condition3 = ConditionBuilder::price_above(token, 100).build();

        assert!(condition3.evaluate(&context));
    }
}

use anchor_lang::prelude::*;
use std::collections::HashMap;

pub struct EvaluationContext {
    pub token_prices: HashMap<Pubkey, u64>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AtomicCondition {
    // Price-based conditions
    PriceAbove { token: Pubkey, price: u64 },
    PriceBelow { token: Pubkey, price: u64 },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Condition {
    Atomic(AtomicCondition),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Not(Box<Condition>),
}

impl Condition {
    pub fn evaluate(&self, ctx: &EvaluationContext) -> bool {
        match self {
            Condition::Atomic(atomic) => match atomic {
                AtomicCondition::PriceAbove { token, price } => {
                    ctx.token_prices.get(token).map_or(false, |p| p > price)
                }
                AtomicCondition::PriceBelow { token, price } => {
                    ctx.token_prices.get(token).map_or(false, |p| p < price)
                }
            },
            Condition::And(left, right) => left.evaluate(ctx) && right.evaluate(ctx),
            Condition::Or(left, right) => left.evaluate(ctx) || right.evaluate(ctx),
            Condition::Not(inner) => !inner.evaluate(ctx),
        }
    }
}

pub struct ConditionBuilder {
    condition: Condition,
}

impl ConditionBuilder {
    pub fn price_above(token: Pubkey, price: u64) -> Self {
        Self {
            condition: Condition::Atomic(AtomicCondition::PriceAbove { token, price }),
        }
    }
    pub fn price_below(token: Pubkey, price: u64) -> Self {
        Self {
            condition: Condition::Atomic(AtomicCondition::PriceBelow { token, price }),
        }
    }
    pub fn and(self, other: ConditionBuilder) -> Self {
        Self {
            condition: Condition::And(Box::new(self.condition), Box::new(other.condition)),
        }
    }

    pub fn or(self, other: ConditionBuilder) -> Self {
        Self {
            condition: Condition::Or(Box::new(self.condition), Box::new(other.condition)),
        }
    }

    pub fn not(inner: ConditionBuilder) -> Self {
        Self {
            condition: Condition::Not(Box::new(inner.condition)),
        }
    }

    pub fn build(self) -> Condition {
        self.condition
    }
}

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
    fn test_evaluate_condition_tree() {
        let token = Pubkey::new_unique();
        let strategy = ConditionBuilder::not(ConditionBuilder::price_above(token, 100))
            .and(ConditionBuilder::price_below(token, 200))
            .build();

        let mut prices = HashMap::new();
        prices.insert(token, 150);

        let context = EvaluationContext {
            token_prices: prices,
        };

        // price > 100 AND price < 200 -> true
        assert_eq!(strategy.evaluate(&context), false);
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

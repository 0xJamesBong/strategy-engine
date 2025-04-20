use anchor_lang::prelude::*;

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

    // pub fn not(self) -> Self {
    //     Self {
    //         condition: Condition::Not(Box::new(self.condition)),
    //     }
    // }

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
}

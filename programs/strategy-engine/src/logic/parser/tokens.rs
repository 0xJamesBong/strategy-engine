use anchor_lang::prelude::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub const ACTION_KEYWORDS: &[&str] = &[
    "BUY", "SELL", "BORROW", "REPAY", "LEND", "REDEEM", "(", ")", ",",
];

#[derive(Debug, Clone, PartialEq)]
pub enum ConditionToken {
    And,
    Or,
    Not,
    LParen,
    RParen,
    Comma,
    PriceAbove,
    PriceBelow,
    Pubkey(String),
    Number(u64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActionToken {
    Buy,
    Sell,
    Borrow,
    Repay,
    Lend,
    Redeem,
    LParen,
    RParen,
    Comma,
    Pubkey(String),
    Number(u64),
}

pub static CONDITION_TOKEN_MAP: Lazy<HashMap<&'static str, ConditionToken>> = Lazy::new(|| {
    HashMap::from([
        ("AND", ConditionToken::And),
        ("OR", ConditionToken::Or),
        ("NOT", ConditionToken::Not),
        ("(", ConditionToken::LParen),
        (")", ConditionToken::RParen),
        (",", ConditionToken::Comma),
        ("PRICE_ABOVE", ConditionToken::PriceAbove),
        ("PRICE_BELOW", ConditionToken::PriceBelow),
    ])
});

pub static ACTION_TOKEN_MAP: Lazy<HashMap<&'static str, ActionToken>> = Lazy::new(|| {
    HashMap::from([
        ("BUY", ActionToken::Buy),
        ("SELL", ActionToken::Sell),
        ("BORROW", ActionToken::Borrow),
        ("REPAY", ActionToken::Repay),
        ("LEND", ActionToken::Lend),
        ("REDEEM", ActionToken::Redeem),
        ("(", ActionToken::LParen),
        (")", ActionToken::RParen),
        (",", ActionToken::Comma),
    ])
});

impl ConditionToken {
    pub fn parse_token_or_arg(s: &str) -> ConditionToken {
        if let Some(keyword) = CONDITION_TOKEN_MAP.get(s) {
            return keyword.clone();
        }
        if let Ok(num) = s.parse::<u64>() {
            return ConditionToken::Number(num);
        }
        ConditionToken::Pubkey(s.to_string())
    }

    pub fn from_keyword_to_token(s: &str) -> Option<Self> {
        CONDITION_TOKEN_MAP.get(s).cloned()
    }
}

impl ActionToken {
    pub fn parse_token_or_arg(s: &str) -> ActionToken {
        if let Some(keyword) = ACTION_TOKEN_MAP.get(s) {
            return keyword.clone();
        }
        if let Ok(num) = s.parse::<u64>() {
            return ActionToken::Number(num);
        }
        ActionToken::Pubkey(s.to_string())
    }
    pub fn from_keyword_to_token(s: &str) -> Option<Self> {
        ACTION_TOKEN_MAP.get(s).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_condition_token_parse() {
        let token = ConditionToken::parse_token_or_arg("PRICE_ABOVE");
        assert_eq!(token, ConditionToken::PriceAbove);
    }
}

// impl ConditionToken {
//     pub fn to_token(s: &str) -> Option<Self> {
//         // match str to CONDITION_KEYWORDS
//         let token = CONDITION_KEYWORDS.iter().find(|&&keyword| keyword == s);
//         // then match token to ConditionToken

//         match token {}
//         // match token {
//         //     "AND" => Some(ConditionToken::And),
//         //     "OR" => Some(ConditionToken::Or),
//         //     "NOT" => Some(ConditionToken::Not),
//         //     "(" => Some(ConditionToken::LParen),
//         //     ")" => Some(ConditionToken::RParen),
//         //     "," => Some(ConditionToken::Comma),
//         //     "PRICE_ABOVE" => Some(ConditionToken::PriceAbove),
//         //     "PRICE_BELOW" => Some(ConditionToken::PriceBelow),
//         //     _ => None,
//         // }
//     }

//     pub fn to_string(&self) -> String {
//         match self {
//             ConditionToken::And => "AND".to_string(),
//             ConditionToken::Or => "OR".to_string(),
//             ConditionToken::Not => "NOT".to_string(),
//             ConditionToken::LParen => "(".to_string(),
//             ConditionToken::RParen => ")".to_string(),
//             ConditionToken::Comma => ",".to_string(),
//             ConditionToken::PriceAbove => "PRICE_ABOVE".to_string(),
//             ConditionToken::PriceBelow => "PRICE_BELOW".to_string(),
//             ConditionToken::Pubkey(s) => s.clone(),
//             ConditionToken::Number(n) => n.to_string(),
//         }
//     }
// }

// impl ActionToken {
//     pub fn to_token(s: &str) -> Option<Self> {
//         match s {
//             "BUY" => Some(ActionToken::Buy),
//             "SELL" => Some(ActionToken::Sell),
//             "BORROW" => Some(ActionToken::Borrow),
//             "REPAY" => Some(ActionToken::Repay),
//             "LEND" => Some(ActionToken::Lend),
//             "REDEEM" => Some(ActionToken::Redeem),
//             "(" => Some(ActionToken::LParen),
//             ")" => Some(ActionToken::RParen),
//             "," => Some(ActionToken::Comma),
//             _ => None,
//         }
//     }
// }

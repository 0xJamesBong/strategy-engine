// "NOT (PRICE_ABOVE(token1,100) AND PRICE_BELOW(token1,200)) OR PRICE_ABOVE(token1,400)"

#[derive(Debug, Clone)]
pub enum Token {
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

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = vec![];

    let re = regex::Regex::new(r"\s+").unwrap();
    let cleaned = re.replace_all(input, "");

    let mut chars = cleaned.chars().peekable();
    while let Some(c) = chars.peek() {
        match c {
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            'A' => {
                let word: String = chars.by_ref().take(3).collect();
                if word == "AND" {
                    tokens.push(Token::And);
                }
            }
            'O' => {
                let word: String = chars.by_ref().take(2).collect();
                if word == "OR" {
                    tokens.push(Token::Or);
                }
            }
            'N' => {
                let word: String = chars.by_ref().take(3).collect();
                if word == "NOT" {
                    tokens.push(Token::Not);
                }
            }
            'P' => {
                let word: String = chars.by_ref().take(11).collect();
                if word == "PRICE_ABOVE" {
                    tokens.push(Token::PriceAbove);
                } else if word == "PRICE_BELOW" {
                    tokens.push(Token::PriceBelow);
                }
            }
            _ => {
                // Parse numbers or pubkeys
                let mut val = String::new();
                while let Some(c) = chars.peek() {
                    if c.is_alphanumeric() {
                        val.push(*c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if let Ok(num) = val.parse::<u64>() {
                    tokens.push(Token::Number(num));
                } else {
                    tokens.push(Token::Pubkey(val));
                }
            }
        }
    }

    tokens
}

fn parse_tokens(tokens: &[Token]) -> (ConditionBuilder, usize) {
    match tokens[0] {
        Token::Not => {
            let (subtree, consumed) = parse_tokens(&tokens[1..]);
            (subtree.not(), consumed + 1)
        }
        Token::LParen => {
            let (left, consumed_left) = parse_tokens(&tokens[1..]);

            match tokens[1 + consumed_left] {
                Token::And => {
                    let (right, consumed_right) = parse_tokens(&tokens[2 + consumed_left..]);
                    (left.and(right), 2 + consumed_left + consumed_right + 1)
                }
                Token::Or => {
                    let (right, consumed_right) = parse_tokens(&tokens[2 + consumed_left..]);
                    (left.or(right), 2 + consumed_left + consumed_right + 1)
                }
                _ => panic!("Expected AND/OR"),
            }
        }
        Token::PriceAbove | Token::PriceBelow => {
            if let [_, Token::Pubkey(ref addr), Token::Comma, Token::Number(price), Token::RParen, ..] =
                tokens
            {
                let pubkey = addr.parse::<Pubkey>().unwrap();
                let builder = match tokens[0] {
                    Token::PriceAbove => ConditionBuilder::price_above(pubkey, *price),
                    Token::PriceBelow => ConditionBuilder::price_below(pubkey, *price),
                    _ => unreachable!(),
                };
                (builder, 6)
            } else {
                panic!("Malformed atomic condition");
            }
        }
        _ => panic!("Unexpected token: {:?}", tokens[0]),
    }
}

pub fn translate_condition_string(input: &str) -> Result<ConditionTree> {
    let tokens = tokenize(input);
    let (builder, _) = parse_tokens(&tokens);
    Ok(builder.build())
}

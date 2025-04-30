use crate::logic::conditions::{ConditionBuilder, ConditionTree};
use anchor_lang::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, digit1, multispace0},
    combinator::map_res,
    error::ParseError,
    multi::fold_many0,
    sequence::{delimited, preceded, tuple},
    IResult, Parser,
};
use std::str::FromStr;

// --- Whitespace-tolerant combinator ---
/** 'a is the lifetime of the input string
* F is anything that implements Parser<&'a str>
* The return type is some parser that takes a &'a str as input.
* where F: Parser<&'a str> says that F is a parser that takes a &'a str as input.
* delimited(multispace0, inner, multispace0) returns a new parser.
*/

pub fn ws<'a, O, E: ParseError<&'a str>, F>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(multispace0, inner, multispace0)
}

// --- Parsing utilities ---
// fn parse_pubkey(input: &str) -> IResult<&str, Pubkey> {
//     map_res(nom::character::complete::alphanumeric1, Pubkey::from_str).parse(input)
// }
fn parse_pubkey(input: &str) -> IResult<&str, Pubkey> {
    // This is a parser that matches 1 or more alphanumeric ASCII characters ([a-zA-Z0-9]).
    let base_parser = alphanumeric1;
    // apply map_res (https://docs.rs/nom/latest/nom/combinator/fn.map_res.html)
    let mut pubkey_parser = map_res(base_parser, Pubkey::from_str);
    pubkey_parser.parse(input)
}

// fn parse_number(input: &str) -> IResult<&str, u64> {
//     map_res(digit1, |s: &str| s.parse::<u64>()).parse(input)
// }

fn parse_number(input: &str) -> IResult<&str, u64> {
    // Step 1: Base parser that matches 1+ digits as a string slice
    let base_parser = digit1;
    // Step 2: Apply map_res to convert the string slice to a u64
    let mut number_parser = map_res(base_parser, |s: &str| s.parse::<u64>());
    // Step 3: Parse the input string
    number_parser.parse(input)
}

// --- Atomic Conditions ---
pub fn parse_price_above(input: &str) -> IResult<&str, ConditionBuilder> {
    let (input, _) = ws(tag("PRICE_ABOVE")).parse(input)?;
    println!("input 1: {:?}", input);
    let (input, _) = ws(char('(')).parse(input)?;
    println!("input 2: {:?}", input);
    let (input, token) = ws(parse_pubkey).parse(input)?;
    println!("input 3: {:?}, token: {:?}", input, token);
    let (input, _) = ws(char(',')).parse(input)?;
    println!("input 4: {:?}", input);
    let (input, price) = ws(parse_number).parse(input)?;
    println!("input 5: {:?}, price: {:?}", input, price);
    let (input, _) = ws(char(')')).parse(input)?;
    println!("input 6: {:?}", input);
    Ok((input, ConditionBuilder::price_above(token, price)))
}

pub fn parse_price_below(input: &str) -> IResult<&str, ConditionBuilder> {
    let (input, _) = ws(tag("PRICE_BELOW")).parse(input)?;
    println!("input 1: {:?}", input);
    let (input, _) = ws(char('(')).parse(input)?;
    println!("input 2: {:?}", input);
    let (input, token) = ws(parse_pubkey).parse(input)?;
    println!("input 3: {:?}, token: {:?}", input, token);
    let (input, _) = ws(char(',')).parse(input)?;
    println!("input 4: {:?}", input);
    let (input, price) = ws(parse_number).parse(input)?;
    println!("input 5: {:?}, price: {:?}", input, price);
    let (input, _) = ws(char(')')).parse(input)?;
    println!("input 6: {:?}", input);
    // todo!()
    Ok((input, ConditionBuilder::price_below(token, price)))
}

pub fn parse_atomic_condition(input: &str) -> IResult<&str, ConditionBuilder> {
    alt((parse_price_above, parse_price_below)).parse(input)
}

// --- Parentheses and NOT ---
fn parse_parenthesized_condition(input: &str) -> IResult<&str, ConditionBuilder> {
    delimited(ws(char('(')), parse_condition_expr, ws(char(')'))).parse(input)
}

fn parse_not(input: &str) -> IResult<&str, ConditionBuilder> {
    let (input, _) = ws(tag("NOT")).parse(input)?;
    let (input, inner) = parse_condition_term(input)?;
    Ok((input, inner.not()))
}

// --- Term: Not, Atomic, Parentheses ---
fn parse_condition_term(input: &str) -> IResult<&str, ConditionBuilder> {
    alt((
        parse_not,
        parse_atomic_condition,
        parse_parenthesized_condition,
    ))
    .parse(input)
}

// --- AND precedence level ---
fn parse_condition_and(input: &str) -> IResult<&str, ConditionBuilder> {
    let (input, init) = parse_condition_term(input)?;
    fold_many0(
        preceded(ws(tag("AND")), parse_condition_term),
        move || init.clone(),
        |acc, next| acc.and(next),
    )
    .parse(input)
}

// --- OR precedence level ---
fn parse_condition_expr(input: &str) -> IResult<&str, ConditionBuilder> {
    let (input, init) = parse_condition_and(input)?;
    fold_many0(
        preceded(ws(tag("OR")), parse_condition_and),
        move || init.clone(),
        |acc, next| acc.or(next),
    )
    .parse(input)
}

// --- Final wrapper ---
pub fn translate_condition_string(input: &str) -> Result<ConditionTree> {
    let (_, builder) = parse_condition_expr(input).map_err(|_| error!(ErrorCode::ParseError))?;
    Ok(builder.build())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_price_above() {
        let token = Pubkey::new_unique();
        let input = format!("PRICE_ABOVE({}, 300)", token);
        let result = parse_price_above(&input);
        println!("result: {:?}", result);
    }

    #[test]
    fn test_parse_price_below() {
        let token = Pubkey::new_unique();
        let input = format!("PRICE_BELOW({}, 400)", token);
        let result = parse_price_below(&input);
        println!("result: {:?}", result);
    }

    #[test]
    fn test_parse_condition_expr() {
        let token = Pubkey::new_unique();
        let input = format!(
            "PRICE_ABOVE({}, 300) AND PRICE_BELOW({}, 400)",
            token, token
        );
        let result = parse_condition_expr(&input);
        println!(
            "\n
        result: {:?}",
            result
        );
    }

    #[test]
    fn test_translate_condition_string_1() {
        let token = Pubkey::new_unique();
        let input = format!(
            "PRICE_ABOVE({}, 300) AND PRICE_BELOW({}, 400)",
            token, token
        );
        let result = translate_condition_string(&input);
        println!("result: {:?}", result);

        let expr = result.unwrap().to_string_expr();
        expr.strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .unwrap_or(&expr);

        println!("expr: {}", expr);
        assert_eq!(expr, input);
    }

    #[test]
    fn test_translate_condition_string_2() {
        let token = Pubkey::new_unique();
        let input = format!("PRICE_ABOVE({}, 300) OR PRICE_BELOW({}, 400)", token, token);
        let result = translate_condition_string(&input);
        println!("result: {:?}", result);

        let expr = result.unwrap().to_string_expr();
        expr.strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .unwrap_or(&expr);

        println!("expr: {}", expr);
        assert_eq!(expr, input);
    }

    #[test]
    fn test_translate_condition_string_3() {
        let token = Pubkey::new_unique();
        let input = format!(
            "NOT(PRICE_ABOVE({}, 300) OR PRICE_BELOW({}, 400))",
            token, token
        );
        let result = translate_condition_string(&input);
        println!("result: {:?}", result);

        let expr = result.unwrap().to_string_expr();
        expr.strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .unwrap_or(&expr);

        println!("expr: {}", expr);
        assert_eq!(expr, input);
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Failed to parse condition string")]
    ParseError,
}

// You can add tests here using #[cfg(test)] mod tests {}

// use nom::{
//     branch::alt,
//     bytes::complete::tag,
//     character::complete::{char, multispace0},
//     combinator::map_res,
//     multi::fold_many0,
//     sequence::{delimited, preceded, tuple},
//     IResult, Parser,
// };

// use super::common::{parse_number, parse_pubkey};
// use super::tokens::{ConditionToken, CONDITION_KEYWORDS};
// use crate::logic::conditions::{ConditionBuilder, ConditionTree};
// use anchor_lang::prelude::*;

// type ParseResult<'a, T> = IResult<&'a str, T>;

// use crate::logic::parser::common::*;

// fn parse_keyword(input: &str) -> ParseResult<ConditionToken> {
//     map_res(tag, |s: &str| {
//         ConditionToken::from_str(s).ok_or_else(|| "Invalid token")
//     })
//     .parse(input)
// }

// fn parse_price_above(input: &str) -> ParseResult<ConditionBuilder> {
//     let (input, (_, token, _, price, _)) = tuple((
//         parse_keyword,
//         delimited(char('('), parse_pubkey, char(',')),
//         multispace0,
//         parse_number,
//         char(')'),
//     ))
//     .parse(input)?;

//     Ok((input, ConditionBuilder::price_above(token, price)))
// }

// fn parse_price_below(input: &str) -> ParseResult<ConditionBuilder> {
//     let (input, (_, token, _, price, _)) = tuple((
//         parse_keyword,
//         delimited(char('('), parse_pubkey, char(',')),
//         multispace0,
//         parse_number,
//         char(')'),
//     ))
//     .parse(input)?;

//     Ok((input, ConditionBuilder::price_below(token, price)))
// }

// fn parse_atomic_condition(input: &str) -> ParseResult<ConditionBuilder> {
//     alt((parse_price_above, parse_price_below)).parse(input)
// }

// fn parse_not(input: &str) -> ParseResult<ConditionBuilder> {
//     let (input, _) = parse_keyword.parse(input)?;
//     let (input, _) = multispace0.parse(input)?;
//     let (input, cond) = parse_parenthesized_condition(input)?;
//     Ok((input, cond.not()))
// }

// fn parse_parenthesized_condition(input: &str) -> ParseResult<ConditionBuilder> {
//     delimited(
//         preceded(multispace0, char('(')),
//         parse_condition_expr,
//         preceded(multispace0, char(')')),
//     )
//     .parse(input)
// }

// fn parse_condition_expr(input: &str) -> ParseResult<ConditionBuilder> {
//     let (input, first) = parse_condition_term(input)?;

//     fold_many0(
//         tuple((
//             preceded(multispace0, alt((parse_keyword, parse_keyword))),
//             preceded(multispace0, parse_condition_term),
//         )),
//         move || first.clone(),
//         |acc, (op, next)| match op {
//             ConditionToken::And => acc.and(next),
//             ConditionToken::Or => acc.or(next),
//             _ => unreachable!(),
//         },
//     )
//     .parse(input)
// }

// fn parse_condition_term(input: &str) -> ParseResult<ConditionBuilder> {
//     alt((
//         parse_not,
//         parse_atomic_condition,
//         parse_parenthesized_condition,
//     ))
//     .parse(input)
// }

// pub fn translate_condition_string(input: &str) -> Result<ConditionTree> {
//     let (_, builder) = parse_condition_expr(input).map_err(|_| error!(ErrorCode::ParseError))?;
//     Ok(builder.build())
// }

// #[error_code]
// pub enum ErrorCode {
//     #[msg("Failed to parse condition string")]
//     ParseError,
// }

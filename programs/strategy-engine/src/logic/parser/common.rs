// use nom::{
//     branch::alt,
//     bytes::complete::{tag, take_while1},
//     character::complete::{char, digit1, multispace0},
//     combinator::{map, map_res},
//     multi::fold_many0,
//     sequence::{delimited, preceded, tuple},
//     IResult, Parser,
// };

// use super::conditions::{ConditionBuilder, ConditionTree};
// use super::tokens::Token;
// use anchor_lang::prelude::*;
// use std::str::FromStr;

// type ParseResult<'a, T> = IResult<&'a str, T>;

// // Parses alphanumeric string (e.g., pubkey string)
// pub fn parse_pubkey(input: &str) -> ParseResult<Pubkey> {
//     map_res(take_while1(|c: char| c.is_alphanumeric()), |s: &str| {
//         Pubkey::from_str(s)
//     })
//     .parse(input)
// }

// // Parses a u64
// pub fn parse_number(input: &str) -> ParseResult<u64> {
//     map_res(digit1, |s: &str| s.parse::<u64>()).parse(input)
// }

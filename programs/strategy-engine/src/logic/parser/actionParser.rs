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
// use super::tokens::{ActionToken, ACTION_KEYWORDS};
// use crate::logic::actions::{ActionBuilder, ActionTree};
// use anchor_lang::prelude::*;

// type ParseResult<'a, T> = IResult<&'a str, T>;

// fn parse_keyword(input: &str) -> ParseResult<ActionToken> {
//     map_res(tag, |s: &str| {
//         ActionToken::from_str(s).ok_or_else(|| "Invalid token")
//     })
//     .parse(input)
// }

// fn parse_swap(input: &str) -> ParseResult<ActionBuilder> {
//     let (input, (_, token_in, _, token_out, _, amount, _)) = tuple((
//         parse_keyword,
//         delimited(char('('), parse_pubkey, char(',')),
//         multispace0,
//         delimited(multispace0, parse_pubkey, char(',')),
//         multispace0,
//         parse_number,
//         char(')'),
//     ))
//     .parse(input)?;

//     Ok((input, ActionBuilder::swap(token_in, token_out, amount)))
// }

// fn parse_send(input: &str) -> ParseResult<ActionBuilder> {
//     let (input, (_, token, _, recipient, _, amount, _)) = tuple((
//         parse_keyword,
//         delimited(char('('), parse_pubkey, char(',')),
//         multispace0,
//         delimited(multispace0, parse_pubkey, char(',')),
//         multispace0,
//         parse_number,
//         char(')'),
//     ))
//     .parse(input)?;

//     Ok((input, ActionBuilder::send(token, recipient, amount)))
// }

// fn parse_atomic_action(input: &str) -> ParseResult<ActionBuilder> {
//     alt((parse_swap, parse_send)).parse(input)
// }

// fn parse_sequence(input: &str) -> ParseResult<ActionBuilder> {
//     let (input, first) = parse_atomic_action(input)?;

//     fold_many0(
//         tuple((
//             preceded(multispace0, parse_keyword),
//             preceded(multispace0, parse_atomic_action),
//         )),
//         move || first.clone(),
//         |acc, (_, next)| acc.then(next),
//     )
//     .parse(input)
// }

// pub fn translate_action_string(input: &str) -> Result<ActionTree> {
//     let (_, builder) = parse_sequence(input).map_err(|_| error!(ErrorCode::ParseError))?;
//     Ok(builder.build())
// }

// #[error_code]
// pub enum ErrorCode {
//     #[msg("Failed to parse action string")]
//     ParseError,
// }

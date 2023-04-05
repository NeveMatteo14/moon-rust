use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, opt},
    multi::{many0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
};

use crate::ast::{Block, Expression, Field, ParList, PrefixExp, Var};

use super::{
    expression::parse_exp,
    statement::{parse_functioncall, parse_return, parse_stmt},
    util::{identifier, parse_string, ws},
    ParseResult,
};

/// Parse a block. A block is zero or more statements followed by an
/// optional return statement.
pub fn parse_block(input: &str) -> ParseResult<Block> {
    map(
        pair(many0(parse_stmt), opt(parse_return)),
        |(statements, return_stat)| Block {
            statements,
            return_stat,
        },
    )(input)
}

// use for explist!
fn parse_namelist(input: &str) -> ParseResult<Vec<String>> {
    map(separated_list1(ws(tag(",")), identifier), |result| {
        result.into_iter().map(String::from).collect()
    })(input)
}

pub fn parse_parlist(input: &str) -> ParseResult<ParList> {
    alt((
        map(
            pair(
                parse_namelist,
                opt(preceded(ws(char(',')), parse_dot_dot_dot)),
            ),
            |result| ParList(result.0, result.1.is_some()),
        ),
        map(parse_dot_dot_dot, |_| ParList(Vec::new(), true)),
    ))(input)
}

pub fn parse_var(input: &str) -> ParseResult<Var> {
    alt((
        map(identifier, |result| Var::NameVar(String::from(result))),
        map(
            pair(
                parse_prefixexp,
                delimited(ws(char('[')), parse_exp, ws(char(']'))),
            ),
            |result| Var::BracketVar((Box::new(result.0), result.1)),
        ),
        map(
            separated_pair(parse_prefixexp, char('.'), identifier),
            |result| Var::DotVar((Box::new(result.0), String::from(result.1))),
        ),
    ))(input)
}

pub fn parse_table_constructor(input: &str) -> ParseResult<Vec<Field>> {
    map(
        delimited(ws(char('{')), opt(parse_fieldlist), ws(char('}'))),
        |result| match result {
            Some(fields) => fields,
            None => Vec::new(),
        },
    )(input)
}

fn parse_fieldlist(input: &str) -> ParseResult<Vec<Field>> {
    separated_list1(ws(alt((char(','), char(';')))), parse_field)(input)
}

fn parse_field(input: &str) -> ParseResult<Field> {
    let result = alt((
        map(
            separated_pair(
                delimited(ws(char('[')), parse_exp, ws(char(']'))),
                ws(char('=')),
                parse_exp,
            ),
            |result| Field::BracketedAssign(result),
        ),
        map(
            separated_pair(ws(identifier), ws(char('=')), ws(parse_exp)),
            |result| Field::NameAssign((String::from(result.0), result.1)),
        ),
        map(parse_exp, |result| Field::UnnamedAssign(result)),
    ))(input);

    result
}

pub fn parse_prefixexp(input: &str) -> ParseResult<PrefixExp> {
    alt((
        map(parse_var, |var| PrefixExp::Var(var)),
        map(parse_functioncall, |fncall| PrefixExp::FunctionCall(fncall)),
        map(delimited(ws(char('(')), parse_exp, ws(char(')'))), |exp| {
            PrefixExp::Exp(exp)
        }),
    ))(input)
}

pub fn parse_funcbody(input: &str) -> ParseResult<(ParList, Block)> {
    terminated(
        pair(delimited(char('('), parse_parlist, char(')')), parse_block),
        ws(tag("end")),
    )(input)
}

pub fn parse_dot_dot_dot(input: &str) -> ParseResult<Expression> {
    // DotDotDot, // Used for a variable number of arguments in things like functions
    map(ws(tag("...")), |_| Expression::DotDotDot)(input)
}

pub fn parse_literal_string(input: &str) -> ParseResult<Expression> {
    // TODO(?): I'm ignoring string literals that aren't in double quotes for now
    map(ws(parse_string), |s| Expression::LiteralString(s))(input)
}

use nom::character::complete::char;
use nom::combinator::value;
use nom::multi::{many1, separated_list1};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    sequence::{delimited, pair, preceded, terminated, tuple},
};

use super::{
    util::*,
    ParseResult,
};

use crate::parser::expression;
use crate::parser::common;
use crate::ast::{Expression, FunctionCall, Statement, Args};

pub fn parse_stmt(input: &str) -> ParseResult<Statement> {

    alt((
        //parse_semicolon,
        parse_assignment,
        parse_function_decl,
        parse_break,
        parse_do_block,
        local_func_decl,
    ))(input)
}
/// Parse a single semicolon. Toss the result since it provides no
/// semantic information.
fn parse_semicolon(input: &str) -> ParseResult<()> {
    value((), char(';'))(input)
}

fn parse_assignment(input: &str) -> ParseResult<Statement> {
    //Assignment((Vec<Var>, Vec<Expression>))
    unimplemented!()
}

fn parse_args(input: &str) -> ParseResult<Args> {

    alt( 

        (map( separated_list1(ws(char(',')), expression::parse_exp), |result| Args::ExpList(result) )),

     )(input)
}
pub fn functioncall(input: &str) -> ParseResult<FunctionCall> {
    // FunctionCall((PrefixExp, Option<String>))
    // map( tuple( (ws(expression::parse_prefixexp), ws(parse_string)) ),  |result| FunctionCall(result))(input)

}

pub fn parse_functioncall_statement(input: &str) -> ParseResult<Statement> {
    // FunctionCall((PrefixExp, Option<String>))
    map( tuple( (ws(expression::parse_prefixexp), ws(parse_string)) ),  |result| Statement::FunctionCall(result))(input)
    unimplemented!()

}

fn parse_break(input: &str) -> ParseResult<Statement> {
    map(ws(tag("break")), |_| Statement::Break)(input)
}

fn parse_do_block(input: &str) ->ParseResult<Statement> {
    // DoBlock(Block)
    map(expression::parse_funcbody, |block| Statement::DoBlock(block.1))(input)
}

fn parse_while(input: &str) -> ParseResult<Statement> {
    // While((Expression, Block))

    unimplemented!()
}

fn parse_repeat(input: &str) -> ParseResult<Statement> {
    // Repeat((Block, Expression))
    unimplemented!()
}

fn parse_if(input: &str) -> ParseResult<Statement> {
    // If((Expression, Block, Vec<(Expression, Block)>, Option<Block>))
    unimplemented!()
}

fn parse_for_num(input: &str) -> ParseResult<Statement> {
    // ForNum((String, i64, i64, Option<i64>, Block))
    map( tuple( (preceded(ws(tag("for")), common::parse_block)) )  )(input)

    unimplemented!()
}

fn parse_for_generic(input: &str) -> ParseResult<Statement> {
    // ForGeneric((Vec<String>, Vec<Expression>, Block))
    unimplemented!()
}

fn parse_function_decl(input: &str) -> ParseResult<Statement> {
    // FunctionDecl((String, ParList, Block)) where String = name of function being declared
    map( tuple( (ws(tag("function")), ws(identifier), preceded(common::parse_parlist, expression::parse_funcbody)) ),  
    |result| Statement::FunctionDecl( (String::from(result.1), result.2.0, result.2.1)) )(input)

}

fn local_func_decl(input: &str) -> ParseResult<Statement> {
    // LocalFuncDecl((String, ParList, Block))
    map( tuple( (ws(tag("function")), ws(identifier), preceded(common::parse_parlist, expression::parse_funcbody)) ),  
    |result| Statement::LocalFuncDecl( (String::from(result.1), result.2.0, result.2.1)) )(input)

}

pub fn parse_return(input: &str) -> ParseResult<Vec<Expression>> {
    unimplemented!()
}


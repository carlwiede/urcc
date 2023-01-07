use std::process;
use core::slice::Iter;

use crate::data_types::*;

// Accept a list of tokens
// Return AST
pub fn parse(tokens: Vec<Token>) -> Prog
{
    let t: Iter<Token> = tokens.iter();
    let p: Prog = parse_program(t);

    //println!("After parse: {:?}", p);

    p
}

fn parse_expr(mut t: Iter<Token>) -> (Iter<Token>, Expr)
{
    match t.next() {
        Some(Token::IntLiteral(val)) => return (t, Expr::IntLiteral(*val)),
        Some(Token::Minus) => {
            let (t, expr) = parse_expr(t);
            return (t, Expr::UnaryOp(UnaryOp::Negation, Box::new(expr)));
        },
        Some(Token::BitComp) => {
            let (t, expr) = parse_expr(t);
            return (t, Expr::UnaryOp(UnaryOp::BitComp, Box::new(expr)));
        },
        Some(Token::LogNeg) => {
            let (t, expr) = parse_expr(t);
            return (t, Expr::UnaryOp(UnaryOp::LogNeg, Box::new(expr)));
        }
        _ => {
            parse_error("parse_expr didn't receive an integer literal or unary operator, exiting...");
            process::exit(1);   // Left to avoid compiler complaints :x
        },
    }
}

fn parse_statement(mut t: Iter<Token>) -> (Iter<Token>, Stmt)
{
    match t.next() {
        Some(Token::Keyword(val)) => {
            if val != "return" { parse_error("expected keyword 'return', but found something else") }
        },
        _ => parse_error("expected keyword 'int', but found something else"),
    }

    let (mut t, expr) = parse_expr(t);

    match t.next() {
        Some(Token::Semicolon) => (),
        _ => parse_error("expected semicolon, found somethin else"),
    }

    return (t, Stmt::Return(expr));
}

fn parse_function(mut t: Iter<Token>) -> Func
{
    let fn_name: String;
    
    match t.next() {
        Some(Token::Keyword(val)) => {
            if val != "int" { parse_error("expected keyword 'int', but found something else") }
        },
        _ => parse_error("expected keyword 'int', but found something else"),
    }

    match t.next() {
        Some(Token::Identifier(val)) => fn_name = val.to_string(),
        _ => {
            parse_error("expected function identifier, but found something else");
            process::exit(1); // Left here to avoid compiler complaints x)
        }
    }

    match t.next() {
        Some(Token::OpenParenthesis) => (),
        _ => parse_error("expected open parenthesis, but found something else"),
    }

    match t.next() {
        Some(Token::CloseParenthesis) => (),
        _ => parse_error("expected close parenthesis, but found something else"),
    }

    match t.next() {
        Some(Token::OpenBrace) => (),
        _ => parse_error("expected open brace, but found something else"),
    }

    let (mut t, stmt) = parse_statement(t);

    match t.next() {
        Some(Token::CloseBrace) => (),
        _ => parse_error("expected close brace, but found something else"),
    }

    return Func::Func(fn_name, stmt);
}

fn parse_program(t: Iter<Token>) -> Prog
{
    return Prog::Prog(parse_function(t));
}

fn parse_error(err_msg: &str)
{
    println!("PARSE ERROR: {}", err_msg);
    process::exit(1);
}
use std::{fs, process, env};
use core::slice::Iter;
use regex::Regex;

// Enum to represent the different types of tokens
#[derive(Debug, Clone)]
enum Token {
    OpenBrace,
    CloseBrace,
    OpenParenthesis,
    CloseParenthesis,
    Semicolon,
    Keyword (String),
    Identifier (String),
    IntegerLiteral (i64),
}

// Enums to represent nodes in an Abstract Syntax Tree (AST)
#[derive(Debug)]
enum Expr {
    IntLiteral(i64),
}

#[derive(Debug)]
enum Stmt {
    Return(Expr),
}

#[derive(Debug)]
enum Func {
    Func(String, Stmt),
}

#[derive(Debug)]
enum Prog {
    Prog(Func),
}

fn parse_expr(mut t: Iter<Token>) -> (Iter<Token>, Expr)
{
    match t.next() {
        Some(Token::IntegerLiteral(val)) => return (t, Expr::IntLiteral(*val)),
        _ => {
            println!("parse_expr didn't receive an integer literal, exiting...");
            process::exit(1);
        },
    }
}

fn parse_statement(mut t: Iter<Token>) -> (Iter<Token>, Stmt)
{
    match t.next() {
        Some(Token::Keyword(val)) => {
            if val != "return" {
                println!("expected keyword 'return', but found something else");
                process::exit(1);  
            }
        },
        _ => {
            println!("expected keyword 'int', but found something else");
            process::exit(1);  
        }
    }

    let (mut t, expr) = parse_expr(t);

    match t.next() {
        Some(Token::Semicolon) => (),
        _ => {
            println!("expected semicolon, but found something else");
            process::exit(1);  
        }
    }

    return (t, Stmt::Return(expr));
}

fn parse_function(mut t: Iter<Token>) -> Func
{
    let fn_name: String;
    
    match t.next() {
        Some(Token::Keyword(val)) => {
            if val != "int" {
                println!("expected keyword 'int', but found something else");
                process::exit(1);  
            };
        },
        _ => {
            println!("expected keyword 'int', but found something else");
            process::exit(1);
        },
    }

    match t.next() {
        Some(Token::Identifier(val)) => fn_name = val.to_string(),
        _ => {
            println!("expected function identifier, but found something else");
            process::exit(1);
        }
    }

    match t.next() {
        Some(Token::OpenParenthesis) => (),
        _ => {
            println!("expected open parenthesis, but found something else");
            process::exit(1);
        }
    }

    match t.next() {
        Some(Token::CloseParenthesis) => (),
        _ => {
            println!("expected close parenthesis, but found something else");
            process::exit(1);
        }
    }

    match t.next() {
        Some(Token::OpenBrace) => (),
        _ => {
            println!("expected open brace, but found something else");
            process::exit(1);
        }
    }

    let (mut t, stmt) = parse_statement(t);

    match t.next() {
        Some(Token::CloseBrace) => (),
        _ => {
            println!("expected close brace, but found something else");
            process::exit(1);
        }
    }

    return Func::Func(fn_name, stmt);
}

fn parse_program(t: Iter<Token>) -> Prog
{
    return Prog::Prog(parse_function(t));
}

// Accept a list of tokens
// Return AST
fn parse(tokens: Vec<Token>)
{
    let t: Iter<Token> = tokens.iter();
    let p: Prog = parse_program(t);
    println!("{:?}", p);
}

// Accept a file
// Return list of tokens
fn lex(file_path: String) -> Vec<Token>
{

    // Load file content to string
    let content = fs::read_to_string(file_path)
        .expect("Could not read file.");

    // Create buffer for storing intermediate tokens (for identifiers, keywords and values)
    let mut buffer: String = String::new();

    // Create vector to store the complete tokens
    let mut tokens: Vec<Token> = Vec::new();

    // Regex to test if the character is between a-z and a digit
    let a_z_num_re: Regex = Regex::new(r"^([a-zA-Z\d])+$").unwrap();

    for c in content.chars() {

        // If buffer has accumulated content and lexer runs into
        // non-letter or non-digit, write buffer to tokens
        if !a_z_num_re.is_match(&c.to_string()) && !buffer.is_empty() {
            
            // Buffer contains a keyword
            if buffer == "int"
            || buffer == "return" {
                tokens.push(Token::Keyword(buffer.clone()));
            } 
            
            // Buffer contains a number
            else if !buffer.parse::<i64>().is_err() {
                tokens.push(Token::IntegerLiteral(buffer.parse::<i64>().unwrap()));
            } 
            
            // Buffer contains an identifier
            else {
                tokens.push(Token::Identifier(buffer.clone()));
            }
            buffer.clear();
        }

        // Match every character and add to tokens or buffer
        match c {
            ' ' | '\n' => continue,
            ';' => tokens.push(Token::Semicolon),
            '{' => tokens.push(Token::OpenBrace),
            '}' => tokens.push(Token::CloseBrace),
            '(' => tokens.push(Token::OpenParenthesis),
            ')' => tokens.push(Token::CloseParenthesis),
            _ => buffer.push(c),
        }
    }

    // Debug print
    //println!("{:?}", tokens);

    return tokens;

}

fn main() 
{

    let path: String;

    // Not so robust argument reading to automate testing
    if env::args().len() > 1 {
        path = env::args().last().unwrap();
    }

    // Allow manual testing
    else {
        path = String::from("cases/week1/valid/return_2.c");
    }

    parse(lex(path));

}

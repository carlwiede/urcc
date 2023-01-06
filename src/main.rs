use std::{fs, process, env, io::Write};
use core::slice::Iter;
use regex::Regex;

static mut DEBUG: bool = false;

#[derive(Debug, Clone, Copy)]
enum UnaryOp {
    Negation,
    BitComp,
    LogNeg
}

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
    IntLiteral (u32),
    UnaryOp(UnaryOp),
}

// Enums to represent nodes in an Abstract Syntax Tree (AST)
#[derive(Debug)]
enum Expr {
    IntLiteral(u32),
    UnaryOp(UnaryOp, Box<Expr>)
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
        Some(Token::IntLiteral(val)) => return (t, Expr::IntLiteral(*val)),
        Some(Token::UnaryOp(op)) => {
            let (t, expr) = parse_expr(t);
            return (t, Expr::UnaryOp(*op, Box::new(expr)));
        },
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

// Accept a list of tokens
// Return AST
fn parse(tokens: Vec<Token>) -> Prog
{
    let t: Iter<Token> = tokens.iter();
    let p: Prog = parse_program(t);
    
    unsafe {
        if DEBUG {
            println!("After parse: {:?}", p);
        }
    }

    p
}

fn parse_error(err_msg: &str)
{
    println!("PARSE ERROR: {}", err_msg);
    process::exit(1);
}

// Accept a file
// Return list of tokens
fn lex(file_path: String) -> Vec<Token>
{

    // Load file content to string
    let content = fs::read_to_string(file_path)
        .expect("Could not read file.");

    // Define keywords in vector
    let mut keywords = Vec::new();
    keywords.push(String::from("int"));
    keywords.push(String::from("return"));

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
            if keywords.contains(&buffer) {
                tokens.push(Token::Keyword(buffer.clone()));
            } 
            
            // Buffer contains a number
            else if !buffer.parse::<u32>().is_err() {
                tokens.push(Token::IntLiteral(buffer.parse::<u32>().unwrap()));
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
            '-' => tokens.push(Token::UnaryOp(UnaryOp::Negation)),
            '~' => tokens.push(Token::UnaryOp(UnaryOp::BitComp)),
            '!' => tokens.push(Token::UnaryOp(UnaryOp::LogNeg)),
            _ => buffer.push(c),
        }
    }

    unsafe {
        if DEBUG {
            println!("After lex: {:?}", tokens);
        }
    }

    return tokens;

}

// Produce x86 assembly from the program
fn produce_assembly(p: Prog, ass_f: String) -> std::io::Result<()>
{
    let Prog::Prog(f) = p;
    let Func::Func(f_name, stmt) = f;
    let Stmt::Return(expr) = stmt;

    let Expr::IntLiteral(ret_val) = expr else {todo!()};

    let mut file = fs::File::create(ass_f)?;
    file.write_all(format!(".globl {f_name}\n").as_bytes()).expect("Failed to write .globl");
    file.write_all(format!("{f_name}:\n").as_bytes()).expect("Failed to write identifier");
    file.write_all(format!("\tmov\t${ret_val}, %rax\n").as_bytes()).expect("Failed to write movl");
    file.write_all(format!("\tret\n").as_bytes()).expect("Failed to write ret");

    Ok(())
}

// Print the input program "prettily"
#[allow(dead_code)]
fn pretty_print(p: Prog)
{
    println!("Pretty printing {:?} ...\n", p);

    let Prog::Prog(f) = p;
    let Func::Func(f_name, stmt) = f;
    let Stmt::Return(expr) = stmt;
    let Expr::IntLiteral(ret_val) = expr else {todo!()};

    println!("FUN INT {}:", f_name);
    println!("\tparams: ()");
    println!("\tbody:");
    println!("\t\tRETURN Int<{}>", ret_val);
   
}

fn main() 
{

    // TODO:    Finish week2
    //          Implement Token as struct instead of Enum

    let mut path: String = String::from("stages/stage_2/valid/bitwise.c");
    let args: Vec<String> = env::args().collect();
    let ass_f: String = String::from("assembly.s");

    // Not so robust argument reading to automate testing
    // Last argument is always the path
    match args.len() {
        1 => (),
        2 => {
            if args.contains(&String::from("debug")) {
                unsafe {
                    DEBUG = true;
                }
            } else {
                path = args[args.len()-1].clone();
            }
        },
        3 => {
            unsafe {
                DEBUG = true;
            }
            path = args[args.len()-1].clone();
        },
        _ => {
            println!("Invalid argument setup, aborting...");
            process::exit(1);
        },
    }

    // Get application name from path, don't ask
    let app_name: String = String::from( 
                         path.clone()
                             .split("/")
                             .collect::<Vec<&str>>()
                             .iter()
                             .rev()
                             .copied()
                             .collect::<Vec<&str>>()[0]
                             .split(".")
                             .collect::<Vec<&str>>()[0]
    );

    let p = parse(lex(path));

    // Produce the assembly
    match produce_assembly(p, ass_f.clone()) {
        Ok(_) => (),
        _ => {
            println!("Failed to produce assembly file!");
            process::exit(1);
        },
    }

    // Produce binary
    process::Command::new("gcc")
                     .arg(ass_f.clone())
                     .arg("-o")
                     .arg(app_name+".exe")
                     .output()
                     .expect("failed to run gcc on assembly file");

    // Delete assembly file
    fs::remove_file(ass_f.clone()).expect("Failed to remove assembly file");

}

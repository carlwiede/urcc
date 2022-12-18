use std::fs;
use regex::Regex;

#[derive(Debug, Clone)]
enum Token {
    OpenBrace,
    CloseBrace,
    OpenParenthesis,
    CloseParenthesis,
    Semicolon,
    Keyword {kind: String},
    Identifier {name: String},
    IntegerLiteral {value: i64},
}

fn main() 
{
    lex("cases/week1/multi_digit.c");
}

// Accept a file
// Return list of tokens
fn lex(file_path: &str)
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

        if !a_z_num_re.is_match(&c.to_string()) && !buffer.is_empty() {
            if buffer == "int"
            || buffer == "return" {
                tokens.push(Token::Keyword { kind: buffer.clone() });
            } else if !buffer.parse::<i64>().is_err() {
                tokens.push(Token::IntegerLiteral { value: buffer.parse::<i64>().unwrap() })
            } else {
                tokens.push(Token::Identifier { name: buffer.clone() });
            }
            buffer.clear();
        }

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

    println!("{:?}", tokens);

}

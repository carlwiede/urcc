use std::fs;
use regex::Regex;

use crate::data_types::*;

// Accept a file
// Return list of tokens
pub fn lex(file_path: String) -> Vec<Token>
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

    //println!("After lex: {:?}", tokens);

    return tokens;

}
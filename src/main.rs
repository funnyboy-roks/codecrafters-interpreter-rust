use std::env;
use std::fmt::{Display, Formatter};
use std::fs::{self, File};
use std::io::{self, Write};
use std::io::{BufRead, BufReader};

use anyhow::bail;

enum Token {
    EOF,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Var,
    Ident(String),
    String(String),
    Semicolon,
    Star,
    Dot,
    Comma,
    Plus,
    Minus,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let token_type = match self {
            Token::EOF => "EOF",
            Token::LParen => "LEFT_PAREN",
            Token::RParen => "RIGHT_PAREN",
            Token::LBrace => "LEFT_BRACE",
            Token::RBrace => "RIGHT_BRACE",
            Token::Var => "VAR",
            Token::Ident(_) => "IDENTIFIER",
            Token::String(_) => "STRING",
            Token::Semicolon => "SEMICOLON",
            Token::Star => "STAR",
            Token::Dot => "DOT",
            Token::Comma => "COMMA",
            Token::Plus => "PLUS",
            Token::Minus => "MINUS",
        };

        let lexeme = match self {
            Token::EOF => "".to_string(),
            Token::LParen => "(".to_string(),
            Token::RParen => ")".to_string(),
            Token::LBrace => "{".to_string(),
            Token::RBrace => "}".to_string(),
            Token::Var => "var".to_string(),
            Token::Ident(s) => s.to_string(),
            Token::String(s) => format!(r#""{}""#, s),
            Token::Semicolon => ";".to_string(),
            Token::Star => "*".to_string(),
            Token::Dot => ".".to_string(),
            Token::Comma => ",".to_string(),
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
        };

        let literal = match self {
            Token::String(s) => s.to_string(),
            _ => "null".to_string(),
        };

        write!(f, "{} {} {}", token_type, lexeme, literal)
    }
}

struct Lexer<'a, R> {
    reader: &'a mut R,
    line: usize,
    error: bool,
    done: bool,
}

impl<'a, R> Lexer<'a, R> {
    fn new(r: &'a mut R) -> Self {
        Self {
            reader: r,
            line: 1,
            error: false,
            done: false,
        }
    }
}

impl<'a, R> Lexer<'a, R>
where
    R: BufRead,
{
    fn read_token(&mut self) -> anyhow::Result<Token> {
        let mut buf = [0u8; 1];
        loop {
            let count = self.reader.read(&mut buf)?;
            if count == 0 {
                self.done = true;
                return Ok(Token::EOF);
            }

            let c = char::from_u32(buf[0] as u32).unwrap();

            return match c {
                '(' => Ok(Token::LParen),
                ')' => Ok(Token::RParen),
                '{' => Ok(Token::LBrace),
                '}' => Ok(Token::RBrace),
                ';' => Ok(Token::Semicolon),
                '*' => Ok(Token::Star),
                '.' => Ok(Token::Dot),
                ',' => Ok(Token::Comma),
                '+' => Ok(Token::Plus),
                '-' => Ok(Token::Minus),
                '\n' => {
                    self.line += 1;
                    continue;
                }
                _ => {
                    eprintln!("[line {}] Error: Unexpected Character: {}", self.line, c);
                    self.error = true;
                    continue;
                }
            };
        }
    }
}

impl<'a, R> Iterator for Lexer<'a, R>
where
    R: BufRead,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            self.read_token().ok()
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        bail!("Usage: {} tokenize <filename>", args[0]);
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file = File::open(filename)?;
            let mut file = BufReader::new(file);

            let mut lexer = Lexer::new(&mut file);

            for tok in &mut lexer {
                println!("{}", tok);
            }

            if lexer.error {
                std::process::exit(65);
            }
        }
        _ => {
            bail!("Unknown command: {}", command);
        }
    }

    return Ok(());
}

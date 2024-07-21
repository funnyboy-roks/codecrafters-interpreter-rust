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
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Slash,
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
            Token::Equal => "EQUAL",
            Token::EqualEqual => "EQUAL_EQUAL",
            Token::Bang => "BANG",
            Token::BangEqual => "BANG_EQUAL",
            Token::Less => "LESS",
            Token::LessEqual => "LESS_EQUAL",
            Token::Greater => "GREATER",
            Token::GreaterEqual => "GREATER_EQUAL",
            Token::Slash => "SLASH",
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
            Token::Equal => "=".to_string(),
            Token::EqualEqual => "==".to_string(),
            Token::Bang => "!".to_string(),
            Token::BangEqual => "!=".to_string(),
            Token::Less => "<".to_string(),
            Token::LessEqual => "<=".to_string(),
            Token::Greater => ">".to_string(),
            Token::GreaterEqual => ">=".to_string(),
            Token::Slash => "/".to_string(),
        };

        let literal = match self {
            Token::String(s) => s.to_string(),
            _ => "null".to_string(),
        };

        write!(f, "{} {} {}", token_type, lexeme, literal)
    }
}

struct Lexer {
    string: Vec<char>,
    index: usize,
    line: usize,
    column: usize,
    error: bool,
    done: bool,
}

impl Lexer {
    fn new(s: &str) -> Self {
        Self {
            string: s.chars().collect(),
            index: 0,
            line: 1,
            column: 0,
            error: false,
            done: false,
        }
    }
}

impl Lexer {
    fn peek_char(&mut self) -> Option<char> {
        if self.index >= self.string.len() {
            None
        } else {
            Some(self.string[self.index])
        }
    }

    fn read_char(&mut self) -> Option<char> {
        let out = self.peek_char();
        self.index += 1;
        out
    }

    fn read_token(&mut self) -> anyhow::Result<Token> {
        'main_lex: loop {
            let Some(c) = self.read_char() else {
                self.done = true;
                return Ok(Token::EOF);
            };

            self.column += 1;

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
                '=' => Ok(match self.peek_char() {
                    Some('=') => {
                        self.read_char();
                        Token::EqualEqual
                    }
                    _ => Token::Equal,
                }),
                '!' => Ok(match self.peek_char() {
                    Some('=') => {
                        self.read_char();
                        Token::BangEqual
                    }
                    _ => Token::Bang,
                }),
                '<' => Ok(match self.peek_char() {
                    Some('=') => {
                        self.read_char();
                        Token::LessEqual
                    }
                    _ => Token::Less,
                }),
                '>' => Ok(match self.peek_char() {
                    Some('=') => {
                        self.read_char();
                        Token::GreaterEqual
                    }
                    _ => Token::Greater,
                }),
                '/' => Ok(match self.peek_char() {
                    Some('/') => {
                        while let Some(c) = self.read_char() {
                            if c == '\n' {
                                break;
                            }
                        }
                        self.line += 1;
                        continue;
                    }
                    _ => Token::Slash,
                }),
                '"' => {
                    //
                    let start = self.index;
                    loop {
                        if let Some(c) = self.read_char() {
                            if c == '"' {
                                break;
                            }
                        } else {
                            eprintln!("[line {}] Error: Unterminated string.", self.line);
                            self.error = true;
                            continue 'main_lex;
                        }
                    }
                    let chars = &self.string[start..self.index - 1];

                    Ok(Token::String(chars.iter().collect()))
                }
                '\n' => {
                    self.line += 1;
                    self.column = 0;
                    continue;
                }
                whitespace if whitespace.is_whitespace() => {
                    continue;
                }
                _ => {
                    eprintln!("[line {}] Error: Unexpected character: {}", self.line, c);
                    self.error = true;
                    continue;
                }
            };
        }
    }
}

impl Iterator for Lexer {
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
            let file = fs::read_to_string(filename)?;

            let mut lexer = Lexer::new(&file);

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

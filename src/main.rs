use std::env;
use std::fmt::{Display, Formatter};
use std::fs;

use anyhow::bail;
use phf::phf_map;

static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "and" => Token::And,
    "class" => Token::Class,
    "else" => Token::Else,
    "false" => Token::False,
    "for" => Token::For,
    "fun" => Token::Fun,
    "if" => Token::If,
    "nil" => Token::Nil,
    "or" => Token::Or,
    "print" => Token::Print,
    "return" => Token::Return,
    "super" => Token::Super,
    "this" => Token::This,
    "true" => Token::True,
    "var" => Token::Var,
    "while" => Token::While,
};

#[derive(Clone)]
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
    Number(f64, String),
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    While,
}

impl Token {
    fn token_type(&self) -> &'static str {
        match self {
            Token::EOF => "EOF",
            Token::LParen => "LEFT_PAREN",
            Token::RParen => "RIGHT_PAREN",
            Token::LBrace => "LEFT_BRACE",
            Token::RBrace => "RIGHT_BRACE",
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
            Token::Number(_, _) => "NUMBER",
            Token::And => "AND",
            Token::Class => "CLASS",
            Token::Else => "ELSE",
            Token::False => "FALSE",
            Token::For => "FOR",
            Token::Fun => "FUN",
            Token::If => "IF",
            Token::Nil => "NIL",
            Token::Or => "OR",
            Token::Print => "PRINT",
            Token::Return => "RETURN",
            Token::Super => "SUPER",
            Token::This => "THIS",
            Token::True => "TRUE",
            Token::Var => "VAR",
            Token::While => "WHILE",
        }
    }

    fn lexeme(&self) -> String {
        match self {
            Token::EOF => "".to_string(),
            Token::LParen => "(".to_string(),
            Token::RParen => ")".to_string(),
            Token::LBrace => "{".to_string(),
            Token::RBrace => "}".to_string(),
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
            Token::Number(_, s) => s.to_string(),
            Token::And => "and".to_string(),
            Token::Class => "class".to_string(),
            Token::Else => "else".to_string(),
            Token::False => "false".to_string(),
            Token::For => "for".to_string(),
            Token::Fun => "fun".to_string(),
            Token::If => "if".to_string(),
            Token::Nil => "nil".to_string(),
            Token::Or => "or".to_string(),
            Token::Print => "print".to_string(),
            Token::Return => "return".to_string(),
            Token::Super => "super".to_string(),
            Token::This => "this".to_string(),
            Token::True => "true".to_string(),
            Token::Var => "var".to_string(),
            Token::While => "while".to_string(),
        }
    }

    fn tok_print(&self) -> String {
        match self {
            Token::EOF => "".to_string(),
            Token::LParen => "(".to_string(),
            Token::RParen => ")".to_string(),
            Token::LBrace => "{".to_string(),
            Token::RBrace => "}".to_string(),
            Token::Ident(s) => s.to_string(),
            Token::String(s) => format!("{}", s),
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
            Token::Number(n, _) => format!("{:?}", n),
            Token::And => "and".to_string(),
            Token::Class => "class".to_string(),
            Token::Else => "else".to_string(),
            Token::False => "false".to_string(),
            Token::For => "for".to_string(),
            Token::Fun => "fun".to_string(),
            Token::If => "if".to_string(),
            Token::Nil => "nil".to_string(),
            Token::Or => "or".to_string(),
            Token::Print => "print".to_string(),
            Token::Return => "return".to_string(),
            Token::Super => "super".to_string(),
            Token::This => "this".to_string(),
            Token::True => "true".to_string(),
            Token::Var => "var".to_string(),
            Token::While => "while".to_string(),
        }
    }

    fn literal(&self) -> String {
        match self {
            Token::String(s) => s.to_string(),
            Token::Number(n, _) => format!("{:?}", n),
            _ => "null".to_string(),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.token_type(),
            self.lexeme(),
            self.literal()
        )
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

    fn read_number(&mut self) -> Option<String> {
        let start = self.index;
        let mut had_dot = false;
        loop {
            if let Some(c) = self.peek_char() {
                if matches!(c, '0'..='9') {
                    self.read_char();
                } else if c == '.' && !had_dot {
                    self.read_char();
                    had_dot = true;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if start == self.index {
            None
        } else {
            if self.string[self.index - 1] == '.' {
                self.index -= 1;
            }
            let chars = &self.string[start..self.index];
            Some(chars.iter().collect())
        }
    }

    fn read_ident(&mut self) -> Option<String> {
        let start = self.index;
        loop {
            if let Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_') = self.peek_char() {
                self.read_char();
            } else {
                break;
            }
        }

        if start == self.index {
            None
        } else {
            if self.string[self.index - 1] == '.' {
                self.index -= 1;
            }
            let chars = &self.string[start..self.index];
            Some(chars.iter().collect())
        }
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
                '0'..='9' => {
                    self.index -= 1;
                    if let Some(num) = self.read_number() {
                        Ok(Token::Number(num.parse()?, num))
                    } else {
                        continue;
                    }
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    self.index -= 1;
                    if let Some(s) = self.read_ident() {
                        if let Some(kw) = KEYWORDS.get(&s) {
                            Ok(kw.clone())
                        } else {
                            Ok(Token::Ident(s))
                        }
                    } else {
                        continue;
                    }
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

    let file = fs::read_to_string(filename)?;
    let mut lexer = Lexer::new(&file);

    match command.as_str() {
        "tokenize" => {
            for tok in &mut lexer {
                println!("{}", tok);
            }

            if lexer.error {
                std::process::exit(65);
            }
        }
        "parse" => {
            for tok in &mut lexer.filter(|m| !matches!(m, Token::EOF)) {
                println!("{}", tok.tok_print());
            }
        }
        _ => {
            bail!("Unknown command: {}", command);
        }
    }

    return Ok(());
}

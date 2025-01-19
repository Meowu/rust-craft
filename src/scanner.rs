use std::{collections::HashMap, io::Error};

use crate::error_format::format_error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Clone, Debug)]
pub enum Literal {
    Indentifier(String),
    Number(f64),
    String(String),
}

#[derive(Clone, Debug)]
pub struct Token {
    pub t_type: TokenType,
    pub lexeme: Vec<u8>,
    pub line: usize,
    // pub col: f64,
    pub literal: Option<Literal>,
}

pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    col: i64,
    error: Option<String>,
    keywords: HashMap<String, TokenType>,
}

impl Default for Scanner {
    fn default() -> Self {
        Scanner {
            source: Vec::new(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            col: -1,
            error: None,
            keywords: HashMap::from([
                ("and".to_string(), TokenType::And),
                ("class".to_string(), TokenType::Class),
                ("else".to_string(), TokenType::Else),
                ("false".to_string(), TokenType::False),
                ("for".to_string(), TokenType::For),
                ("fun".to_string(), TokenType::Fun),
                ("if".to_string(), TokenType::If),
                ("nil".to_string(), TokenType::Nil),
                ("or".to_string(), TokenType::Or),
                ("print".to_string(), TokenType::Print),
                ("return".to_string(), TokenType::Return),
                ("super".to_string(), TokenType::Super),
                ("this".to_string(), TokenType::This),
                ("true".to_string(), TokenType::True),
                ("var".to_string(), TokenType::Var),
                ("while".to_string(), TokenType::While),
            ]),
        }
    }
}

pub fn scan_tokens(input: String) -> Result<Vec<Token>, Error> {
    let mut scanner: Scanner = Default::default();

    scanner.scan_tokens(input);

    Ok(scanner.tokens)
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner::default()
    }
    pub fn scan_tokens(&mut self, source: String) {
        self.source = source.into_bytes();
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            t_type: TokenType::Eof,
            lexeme: vec![],
            line: self.line,
            literal: None,
        });
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            '*' => self.add_token(TokenType::Star),
            ';' => self.add_token(TokenType::Semicolon),
            '!' => {
                let match_eq = self.matches('=');
                self.add_token(if match_eq {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                })
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
                self.col = 0;
            }
            '"' => self.string(),
            '=' => {
                let match_eq = self.matches('=');
                self.add_token(if match_eq {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                })
            }
            '<' => {
                let match_eq = self.matches('=');
                self.add_token(if match_eq {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                })
            }
            '>' => {
                let match_eq = self.matches('=');
                self.add_token(if match_eq {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                })
            }
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            _ => {
                if Self::is_ascii_digit(c) {
                    self.number();
                } else if Self::is_alpha(c) {
                    self.identifier();
                } else {
                    let error = format!("Invalid character: {}", c);
                    self.error = Some(error.clone());
                    format_error(&error, self.line, self.col);
                }
            }
        }
    }

    fn is_alpha(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn is_ascii_digit(c: char) -> bool {
        // c >= '0' && c <= '9'
        c.is_ascii_digit()
    }

    fn is_alpha_numeric(c: char) -> bool {
        Self::is_alpha(c) || Self::is_ascii_digit(c)
    }

    fn identifier(&mut self) {
        while Self::is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let literal = String::from_utf8(self.source[self.start..self.current].to_vec()).unwrap();
        // let token_type = self.keywords.get(&literal)
        //     .copied()  // 或者 .cloned()
        //     .unwrap_or(TokenType::Identifier);
        let token_type = if self.keywords.contains_key(&literal) {
            *self.keywords.get(&literal).unwrap()
        } else {
            TokenType::Identifier
        };
        self.add_token(token_type); // why not add_token_literal ?
    }

    fn number(&mut self) {
        while Self::is_ascii_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && Self::is_ascii_digit(self.peek_next()) {
            self.advance();
            while Self::is_ascii_digit(self.peek()) {
                self.advance();
            }
        }
        let digit = String::from_utf8(self.source[self.start..self.current].to_vec())
            .unwrap()
            .parse::<f64>()
            .unwrap();
        self.add_token_literal(TokenType::Number, Some(Literal::Number(digit)));
    }
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            let error = format!("Unterminated string.");
            self.error = Some(error.clone());
            format_error(&error, self.line, self.col);
            return;
        }
        self.advance();
        let str =
            String::from_utf8(self.source[self.start + 1..self.current - 1].to_vec()).unwrap();
        self.add_token_literal(TokenType::String, Some(Literal::String(str)));
    }

    fn matches(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if (self.source[self.current] as char) != c {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current] as char
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source[self.current + 1] as char;
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        // c as char
        char::from(c)
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        self.tokens.push(Token {
            t_type: token_type,
            literal,
            line: self.line,
            lexeme: self.source[self.start..self.current].to_vec(),
        });
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

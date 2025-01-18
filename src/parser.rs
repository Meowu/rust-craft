use crate::expr::{self, BinaryOp, Literal, UnaryOp, UnaryOpType};
use crate::expr::{BinaryOpType, Expr};
use crate::scanner::{self, *};

pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
}

pub enum Error {
    UnexpectedToken(Token),
    TokenMissmatch {
        expected: TokenType,
        found: Token,
        message: Option<String>,
    },
    ExpectedExpression {
        token_type: TokenType,
        line: usize,
        col: i64,
    },
}

impl Parser {
    pub fn parse(&mut self, tokens: Vec<Token>) -> Result<(), Error> {
        self.tokens = tokens;
        let expr = self.expression()?;

        Ok(())
    }

    fn expression(&mut self) -> Result<Expr, Error> {
        return self.equality();
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.comparison()?;
        while self.matches(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone(); // mutable borrow
            let right = Box::new(self.comparison()?);
            let binary_op = Self::token_to_binary_operator(&operator);
            expr = Expr::Binary(Box::new(expr), binary_op, right)
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        // Expr::Literal(Literal::Number(52.0))
        let mut expr = self.term()?;
        while self.matches(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            let binary_op = Self::token_to_binary_operator(&operator);
            expr = Expr::Binary(Box::new(expr), binary_op, Box::new(right))
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, Error> {
        // Expr::Literal(Literal::Number(52.0))
        let mut expr = self.unary()?;
        while self.matches(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            let binary_op = Self::token_to_binary_operator(&operator);
            expr = Expr::Binary(Box::new(expr), binary_op, Box::new(right))
        }
        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        if self.match_one(TokenType::False) {
            return Ok(Expr::Literal(Literal::False));
        }
        if self.match_one(TokenType::True) {
            return Ok(Expr::Literal(Literal::True));
        }
        if self.match_one(TokenType::Nil) {
            return Ok(Expr::Literal(Literal::Nil));
        }
        if self.matches(vec![TokenType::Number, TokenType::String]) {
            let literal = self.previous().clone().literal;
            match literal {
                Some(scanner::Literal::Number(n)) => {
                    return Ok(Expr::Literal(Literal::Number(n)));
                }
                Some(scanner::Literal::String(s)) => {
                    return Ok(Expr::Literal(Literal::String(s)));
                }
                _ => panic!("Expected an literal"),
            }
        }

        if self.match_one(TokenType::LeftParen) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        let current = self.peek();
        Err(Error::ExpectedExpression {
            token_type: current.t_type,
            line: current.line,
            col: -1,
        })
    }

    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr = self.factor()?;
        while self.matches(vec![TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            let binary_op = Self::token_to_binary_operator(&operator);
            expr = Expr::Binary(Box::new(expr), binary_op, Box::new(right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.matches(vec![TokenType::Minus, TokenType::Bang]) {
            let operator = self.previous().clone();
            // one and only another unary.
            let right = self.unary()?;
            let unary_op = Self::token_to_unary_op(&operator);
            return Ok(Expr::Unary(unary_op, Box::new(right)));
        }

        self.primary()
    }

    fn token_to_unary_op(token: &Token) -> UnaryOp {
        let Token { t_type, .. } = token;
        let line = token.line;
        let col = -1;
        match t_type {
            TokenType::Minus => UnaryOp {
                op_type: UnaryOpType::Minus,
                line,
                col,
            },
            TokenType::Bang => UnaryOp {
                op_type: UnaryOpType::Bang,
                line,
                col,
            },
            _ => UnaryOp {
                op_type: UnaryOpType::Bang,
                line,
                col: -1,
            },
        }
    }

    fn token_to_binary_operator(token: &Token) -> BinaryOp {
        let line = token.line;
        let col = -1;
        match token.t_type {
            TokenType::Equal => BinaryOp {
                op_type: BinaryOpType::Equal,
                line,
                col: -1,
            },
            TokenType::EqualEqual => BinaryOp {
                op_type: BinaryOpType::EqualEqual,
                line,
                col: -1,
            },
            TokenType::Greater => BinaryOp {
                op_type: BinaryOpType::Greater,
                line,
                col,
            },
            TokenType::GreaterEqual => BinaryOp {
                op_type: BinaryOpType::GreaterEqual,
                line,
                col,
            },
            TokenType::Less => BinaryOp {
                op_type: BinaryOpType::Less,
                line,
                col,
            },
            TokenType::LessEqual => BinaryOp {
                op_type: BinaryOpType::LessEqual,
                line,
                col,
            },
            TokenType::Slash => BinaryOp {
                op_type: BinaryOpType::Slash,
                line,
                col,
            },
            TokenType::Star => BinaryOp {
                op_type: BinaryOpType::Star,
                line,
                col,
            },
            _ => BinaryOp {
                op_type: BinaryOpType::LessEqual,
                line,
                col: -1,
            },
        }
    }

    fn matches(&mut self, token_types: Vec<TokenType>) -> bool {
        for token in token_types.iter() {
            if self.match_one(*token) {
                return true;
            }
        }
        false
    }

    fn match_one(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            return true;
        }
        false
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().t_type == TokenType::Semicolon {
                return;
            }
            match self.peek().t_type {
                TokenType::If
                | TokenType::Var
                | TokenType::For
                | TokenType::Fun
                | TokenType::While => return,
                _ => {}
            }
        }
        self.advance();
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().t_type == token_type
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, Error> {
        if self.check(token_type) {
            return Ok(self.advance().clone());
        }
        Err(Error::TokenMissmatch {
            expected: token_type,
            found: self.peek().clone(),
            message: Some(message.to_string()),
        })
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().t_type == TokenType::Eof
    }
}

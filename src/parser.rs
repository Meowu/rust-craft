use crate::expr::{self, BinaryOp, Literal};
use crate::expr::{BinaryOpType, Expr};
use crate::scanner::*;

pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn parse(&mut self, tokens: Vec<Token>) {
        self.tokens = tokens;
    }

    fn expression(&self) {
        return self.equality();
    }

    fn equality(&self) {
        let mut expr = self.comparison();
        while self.matches() {
            let operator = self.previous();
            let right = self.comparison();
            let binary_op = match operator.t_type {
                TokenType::Equal => BinaryOp {
                    op_type: BinaryOpType::Equal,
                    line: operator.line,
                    col: -1,
                },
                TokenType::EqualEqual => BinaryOp {
                    op_type: BinaryOpType::EqualEqual,
                    line: operator.line,
                    col: -1,
                },
                _ => BinaryOp {
                    op_type: BinaryOpType::EqualEqual,
                    line: operator.line,
                    col: -1,
                },
            };
            expr = Expr::Binary(Box::new(expr), binary_op, Box::new(right))
        }
    }

    fn comparison(&self) -> Expr {
        Expr::Literal(Literal::Number(52.0))
    }

    fn matches(&self) -> bool {
        return true;
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().t_type == token_type
        }
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

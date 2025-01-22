use core::f64;
use std::f64::NAN;

use crate::expr::{self, BinaryOp, Expr, Literal, UnaryOp, UnaryOpType};

pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoxType {
    Number,
    String,
    Boolean,
    Nil,
}

pub fn instance_of(val: &Value) -> LoxType {
    match val {
        Value::Nil => LoxType::Nil,
        Value::Number(_) => LoxType::Number,
        Value::String(_) => LoxType::String,
        Value::Boolean(_) => LoxType::Boolean,
    }
}

pub enum RuntimeError {}

pub struct Interpreter {}

impl Interpreter {
    fn evaluate(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(literal) => Ok(self.visit_literal(literal)),
            Expr::Unary(op, e) => self.visit_unary(*op, e),
            Expr::Binary(lhs, op, rhs) => self.visit_binary(lhs, op.clone(), rhs),
            Expr::Grouping(e) => self.evaluate(e),
            _ => Err("E".to_string()),
        }
    }
    fn visit_literal(&mut self, expr: &Literal) -> Value {
        match expr {
            Literal::String(s) => Value::String(s.clone()),
            Literal::Number(n) => Value::Number(*n),
            Literal::True => Value::Boolean(true),
            Literal::False => Value::Boolean(false),
            Literal::Nil => Value::Nil,
        }
    }

    fn visit_unary(&mut self, op: UnaryOp, expr: &Expr) -> Result<Value, String> {
        let val = self.evaluate(expr)?;

        match (op.op_type, &val) {
            (UnaryOpType::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
            (UnaryOpType::Bang, Value::Number(_)) => Ok(Value::Boolean(!Self::is_truthy(&val))),
            (_, Value::String(_)) => Err(format!(
                "Invalid use of unary operator '{:?}' on a String type at line {}, column {}.",
                op.op_type, op.line, op.col
            )),
            (_, Value::Boolean(_)) => Err(format!(
                "Invalid use of unary operator '{:?}' on a Boolean type at line {}, column {}.",
                op.op_type, op.line, op.col
            )),
            (_, Value::Nil) => Err(format!(
                "Invalid use of unary operator {:?} on a Nil type at line {}, column {}.",
                op.op_type, op.line, op.col
            )),
        }
    }

    fn visit_binary(&mut self, lhs: &Expr, op: BinaryOp, rhs: &Expr) -> Result<Value, String> {
        let left = self.evaluate(lhs).unwrap();
        let right = self.evaluate(rhs).unwrap();
        match (&left, op.op_type, &right) {
            (Value::Number(l), expr::BinaryOpType::Greater, Value::Number(r)) => {
                Ok(Value::Boolean(l > r))
            }
            (Value::Number(l), expr::BinaryOpType::GreaterEqual, Value::Number(r)) => {
                Ok(Value::Boolean(l >= r))
            }
            (Value::Number(l), expr::BinaryOpType::Less, Value::Number(r)) => {
                Ok(Value::Boolean(l < r))
            }
            (Value::Number(l), expr::BinaryOpType::LessEqual, Value::Number(r)) => {
                Ok(Value::Boolean(l <= r))
            }
            (Value::Number(l), expr::BinaryOpType::Plus, Value::Number(r)) => {
                Ok(Value::Number(l + r))
            }
            (Value::Number(l), expr::BinaryOpType::Minus, Value::Number(r)) => {
                Ok(Value::Number(l - r))
            }
            (Value::Number(l), expr::BinaryOpType::Star, Value::Number(r)) => {
                Ok(Value::Number(l * r))
            }
            (Value::Number(ln), expr::BinaryOpType::Slash, Value::Number(rn)) => {
                if *rn != 0.0 {
                    Ok(Value::Number(ln / rn))
                } else {
                    Err("Division by zero.".to_string())
                }
            }
            (Value::String(ls), expr::BinaryOpType::Plus, Value::String(rs)) => {
                Ok(Value::String(String::from(ls) + &rs))
            }
            (_, expr::BinaryOpType::EqualEqual, _) => {
                Ok(Value::Boolean(Self::equals(&left, &right)))
            }
            (_, expr::BinaryOpType::BangEqual, _) => {
                Ok(Value::Boolean(!Self::equals(&left, &right)))
            }
            _ => Err(format!(
                "Invalid operands for binary operator {:?} of types {:?} and {:?} at line {}, column {}.",
                op.op_type, instance_of(&left), instance_of(&right),  op.line, op.col
            )),
        }
    }

    fn is_truthy(val: &Value) -> bool {
        match val {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            _ => true,
        }
        // if let Value::Boolean(_) = val {
        //     true
        // } else {
        //     false
        // }
    }

    fn equals(lhs: &Value, rhs: &Value) -> bool {
        match (lhs, rhs) {
            (Value::Boolean(b1), Value::Boolean(b2)) => b1 == b2,
            (Value::String(s1), Value::String(s2)) => s1 == s2,
            (Value::Number(n1), Value::Number(n2)) => {
                if n1.is_nan() || n2.is_nan() {
                    false
                } else {
                    (n1 - n2).abs() < f64::EPSILON
                }
            }
            // Nil != Nil
            (Value::Nil, Value::Nil) => true,
            _ => false,
        }
    }
}

use std::f64::NAN;

use crate::expr::{self, Expr, Literal, UnaryOp, UnaryOpType};

pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

pub struct Interpreter {}

impl Interpreter {
    fn evaluate(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(literal) => Ok(self.visit_literal(literal)),
            Expr::Unary(op, e) => self.visit_unary(*op, e),
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
            _ => Err("Invalid Unary".to_string()),
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
}

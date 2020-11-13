#![allow(dead_code)]
extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;
use std::ops::{Add, Sub, Div, Mul};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> Self {
        Self(s.parse().unwrap())
    }
}

impl Add for Number {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self (
            self.0 + other.0,
        )
    }
}

impl Sub for Number {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self (
            self.0 - other.0,
        )
    }
}

impl Div for Number {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self (
            self.0 / other.0,
        )
    }
}

impl Mul for Number {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self (
            self.0 * other.0,
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Div,
    Mul,
}

impl Op {
    pub fn new(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "/" => Self::Div,
            "*" => Self::Mul,
            _ => panic!("Refusing to create; The following are supported: +, -, /, *"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub lhs: Number,
    pub rhs: Number,
    pub op: Op,
}

impl Expr {
    pub fn new(s: &str) -> Self {
         let unicode_vec = &mut UnicodeSegmentation::graphemes(s, true).collect::<Vec<&str>>();
         let rhs = Number::new(unicode_vec.pop().unwrap());
         let op = Op::new(unicode_vec.pop().unwrap());
         let lhs = Number::new(unicode_vec.pop().unwrap());

         Self { lhs, rhs, op }
    }
}


fn eval(e: Expr) -> Number {
    match e.op {
        Op::Add => e.lhs + e.rhs,
        Op::Sub => e.lhs - e.rhs,
        Op::Div => e.lhs / e.rhs,
        Op::Mul => e.lhs * e.rhs,
    }
}


 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_int() {
        assert_eq!(Number::new("123"), Number(123));
    }

    #[test]
    fn parse_op_add() {
        assert_eq!(Op::new("+"), Op::Add);
    }

    #[test]
    fn parse_op_sub() {
        assert_eq!(Op::new("-"), Op::Sub);
    }
       
    #[test]
    fn parse_op_div() {
        assert_eq!(Op::new("/"), Op::Div)
    }

    #[test]
    fn parse_op_mul() {
        assert_eq!(Op::new("*"), Op::Mul);
    }

    #[test]
    fn parse_expr() {
        let expr: Expr = Expr { lhs: Number::new("1"), rhs: Number::new("2"), op: Op::new("+") };
        assert_eq!(Expr::new("1+2"), expr);
    }

    #[test]
    fn evaluate_expr() {
        let expr: Expr = Expr::new("1+2");
        assert_eq!(eval(expr), Number::new("3"));
    }

}

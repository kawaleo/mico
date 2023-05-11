#![allow(dead_code)]
use std::iter::Peekable;
use std::slice::Iter;

use super::{
    ast::{BinaryExpr, Expr},
    lexer::Token,
};

pub fn parse(tokens: &[Token]) -> Expr {
    let mut iter = tokens.iter().peekable();
    parse_expression(&mut iter)
}

fn parse_expression(iter: &mut Peekable<Iter<Token>>) -> Expr {
    let mut lhs = parse_term(iter);
    while let Some(&&token) = iter.peek() {
        match token {
            Token::Plus => {
                iter.next();
                let rhs = parse_term(iter);
                lhs = Expr::BinaryExpr(BinaryExpr {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    operator: Token::Plus,
                });
            }
            Token::Minus => {
                iter.next();
                let rhs = parse_term(iter);
                lhs = Expr::BinaryExpr(BinaryExpr {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    operator: Token::Minus,
                });
            }
            _ => break,
        }
    }
    lhs
}

fn parse_term(iter: &mut Peekable<Iter<Token>>) -> Expr {
    let mut lhs = parse_factor(iter);
    while let Some(&&token) = iter.peek() {
        match token {
            Token::Times => {
                iter.next();
                let rhs = parse_factor(iter);
                lhs = Expr::BinaryExpr(BinaryExpr {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    operator: Token::Times,
                });
            }
            Token::Divide => {
                iter.next();
                let rhs = parse_factor(iter);
                lhs = Expr::BinaryExpr(BinaryExpr {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    operator: Token::Divide,
                });
            }
            _ => break,
        }
    }
    lhs
}

fn parse_factor(iter: &mut Peekable<Iter<Token>>) -> Expr {
    match iter.next().unwrap() {
        Token::Number(num) => {
            let mut expr = Expr::Number(*num);
            while let Some(&&Token::LParen) = iter.peek() {
                iter.next();
                let next_expr = parse_expression(iter);
                expr = Expr::BinaryExpr(BinaryExpr {
                    lhs: Box::new(expr),
                    rhs: Box::new(next_expr),
                    operator: Token::Times,
                });
                assert_eq!(iter.next(), Some(&Token::RParen));
            }
            expr
        }
        Token::LParen => {
            let expr = parse_expression(iter);
            assert_eq!(iter.next(), Some(&Token::RParen));
            expr
        }
        _ => panic!("Invalid factor"),
    }
}

pub fn evaluate(expr: &Expr) -> f64 {
    match expr {
        Expr::Number(num) => *num,
        Expr::BinaryExpr(bin_expr) => {
            let lhs = evaluate(&*bin_expr.lhs);
            let rhs = evaluate(&*bin_expr.rhs);
            match bin_expr.operator {
                Token::Plus => lhs + rhs,
                Token::Minus => lhs - rhs,
                Token::Times => lhs * rhs,
                Token::Divide => lhs / rhs,
                _ => panic!("Invalid operator"),
            }
        }
    }
}

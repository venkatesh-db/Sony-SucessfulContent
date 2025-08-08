
use crate::ast::{Expr, Stmt};
use crate::lexer::Token;

pub fn parse(tokens: &[Token]) -> Stmt {
    let mut iter = tokens.iter().peekable();

    match iter.next() {
        Some(Token::Let) => {
            if let Some(Token::Ident(name)) = iter.next() {
                if let Some(Token::Equals) = iter.next() {
                    let lhs = match iter.next() {
                        Some(Token::Number(n)) => Expr::Number(*n),
                        Some(Token::Ident(v)) => Expr::Var(v.clone()),
                        _ => panic!("Expected expression"),
                    };

                    let expr = match iter.next() {
                        Some(Token::Plus) => {
                            let rhs = match iter.next() {
                                Some(Token::Number(n)) => Expr::Number(*n),
                                Some(Token::Ident(v)) => Expr::Var(v.clone()),
                                _ => panic!("Expected right operand"),
                            };
                            Expr::Add(Box::new(lhs), Box::new(rhs))
                        }
                        Some(Token::Minus) => {
                            let rhs = match iter.next() {
                                Some(Token::Number(n)) => Expr::Number(*n),
                                Some(Token::Ident(v)) => Expr::Var(v.clone()),
                                _ => panic!("Expected right operand"),
                            };
                            Expr::Sub(Box::new(lhs), Box::new(rhs))
                        }
                        _ => lhs,
                    };

                    Stmt::Let(name.clone(), expr)
                } else {
                    panic!("Expected equals sign");
                }
            } else {
                panic!("Expected identifier");
            }
        }
        _ => panic!("Expected 'let' statement"),
    }
}
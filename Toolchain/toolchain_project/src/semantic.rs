
use std::collections::HashSet;
use crate::ast::{Stmt, Expr};

pub fn analyze(stmt: &Stmt, declared_vars: &mut HashSet<String>) {
    match stmt {
        Stmt::Let(name, expr) => {
            check_expr(expr, declared_vars);
            declared_vars.insert(name.clone());
        }
    }
}

fn check_expr(expr: &Expr, declared_vars: &HashSet<String>) {
    match expr {
        Expr::Number(_) => {}
        Expr::Var(v) => {
            if !declared_vars.contains(v) {
                panic!("Use of undeclared variable: {}", v);
            }
        }
        Expr::Add(lhs, rhs) | Expr::Sub(lhs, rhs) => {
            check_expr(lhs, declared_vars);
            check_expr(rhs, declared_vars);
        }
    }
}
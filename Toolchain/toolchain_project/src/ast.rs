
#[derive(Debug, Clone)]
pub enum Expr {
    Number(i32),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Var(String),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(String, Expr),
}
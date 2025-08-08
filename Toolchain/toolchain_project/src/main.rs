
mod lexer;
mod parser;
mod ast;
mod semantic;

use std::collections::HashSet;

fn main() {
    let input = "let x = 2 + 3;";
    println!("Input: {}", input);

    let tokens = lexer::lex(input);
    println!("Tokens: {:?}", tokens);

    let stmt = parser::parse(&tokens);
    println!("Parsed: {:?}", stmt);

    let mut declared_vars = HashSet::new();
    semantic::analyze(&stmt, &mut declared_vars);
}

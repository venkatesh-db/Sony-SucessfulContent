
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Ident(String),
    Equals,
    Number(i32),
    Plus,
    Minus,
    Semicolon,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\n' => {
                chars.next();
            }
            '=' => {
                tokens.push(Token::Equals);
                chars.next();
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();
            }
            '0'..='9' => {
                let mut num = 0;
                while let Some(&digit) = chars.peek() {
                    if digit.is_digit(10) {
                        num = num * 10 + digit.to_digit(10).unwrap() as i32;
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num));
            }
            'a'..='z' | 'A'..='Z' => {
                let mut ident = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() {
                        ident.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if ident == "let" {
                    tokens.push(Token::Let);
                } else {
                    tokens.push(Token::Ident(ident));
                }
            }
            _ => {
                chars.next();
            }
        }
    }

    tokens
}
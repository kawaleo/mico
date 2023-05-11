#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Times,
    Divide,
    LParen,
    RParen,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut iter = input.chars().peekable();
    while let Some(ch) = iter.next() {
        match ch {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Times),
            '/' => tokens.push(Token::Divide),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),

            c if c.is_whitespace() => continue,
            c if c.is_digit(10) => {
                let mut num_str = String::from(c);
                while let Some(&next_ch) = iter.peek() {
                    if next_ch.is_digit(10) || next_ch == '.' {
                        num_str.push(next_ch);
                        iter.next();
                    } else {
                        break;
                    }
                }
                let num = num_str.parse::<f64>().unwrap();
                tokens.push(Token::Number(num));
            }
            _ => panic!("Invalid character: {}", ch),
        }
    }
    tokens
}

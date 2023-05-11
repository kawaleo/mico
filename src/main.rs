use std::io;

use mico::interpreter::{
    lexer::tokenize,
    parser::{evaluate, parse},
};

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let tokens = tokenize(&input);
        println!("Tokens: {:#?}", tokens);
        let expr = parse(&tokens);
        println!("AST: {:#?}", expr);
        let result = evaluate(&expr);
        println!("Result: {}", result);
    }
}

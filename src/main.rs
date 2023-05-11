use std::env;
use std::io;

use mico::interpreter::{
    lexer::tokenize,
    parser::{evaluate, parse},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "+dbg" {
        debug();
    } else {
        no_debug();
    }
}

fn debug() {
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

fn no_debug() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let tokens = tokenize(&input);
        let expr = parse(&tokens);
        let result = evaluate(&expr);
        println!("Result: {}", result);
    }
}

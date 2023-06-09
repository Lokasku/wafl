pub mod parser;

use std::fs;
use std::env;

use parser::lexer::Lexer;
use parser::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {panic!("You must provide filecode.")}

    let content = fs::read_to_string(args[1].clone()).expect("Cannot read file for some reasons.");
    let mut lexer = Lexer::new(String::from(content));
    lexer.lex();

    let mut parser = Parser::new(lexer.output);
    parser.parse();

    dbg!(parser.output);
}

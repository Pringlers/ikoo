use ikoo::{lexer::Lexer, parser::Parser};
use std::fs;

fn main() {
    let Some(input) = std::env::args().nth(1) else {
        panic!("Please supply file to parse");
    };

    let input_file = fs::read_to_string(input).expect("Given file does not exist");

    println!("Parsing file contents -> \n{}", input_file);

    let lexer: Lexer = Lexer::new(&input_file);
    let mut parser = Parser::new(&input_file, lexer);

    let module = parser.parse_module();
    for statement in module.stmts {
        println!("{statement:#?}");
    }
}

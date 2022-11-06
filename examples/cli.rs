use ikoo::{lexer::Lexer, parser::Parser};

fn main() {
    let Some(input) = std::env::args().nth(1) else {
        panic!("Please supply argument to parse");
    };

    println!("Input: {input:?}");

    let lexer: Lexer = Lexer::new(&input);
    let mut parser = Parser::new(&input, lexer);

    let module = parser.parse_module();
    for stmt in module.stmts {
        println!("{stmt:#?}");
    }
}

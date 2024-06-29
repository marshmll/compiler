use ast::{evaluator::Evaluator, lexer::Lexer, parser::Parser, AST};

mod ast;

fn main() {
    let input = "1 + 1";

    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    //println!("{:#?}", tokens);

    let mut ast = AST::new();
    let mut parser = Parser::new(tokens);

    while let Some(statement) = parser.next_statement() {
        ast.add_statement(statement);
    }

    ast.visualize();

    let mut eval = Evaluator::new();
    ast.visit(&mut eval);

    println!("Result: {:?}", eval.last_value);
}

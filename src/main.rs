use ast::{lexer::Lexer, parser::Parser, AST};

mod ast;

fn main() {
    let input = "(8 + 1) + 2";

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
}

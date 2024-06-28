use ast::{lexer::Lexer, parser::Parser, AST};

mod ast;

fn main() {
    let input = "7 - 2";

    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    println!("{:#?}", tokens);

    let mut ast = AST::new();
    let mut parser = Parser::from_tokens(tokens);

    while let Some(stmt) = parser.next_statement() {
        ast.add_statement(stmt);
    }

    ast.visualize();
}

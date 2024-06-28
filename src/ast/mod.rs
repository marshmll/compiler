pub mod lexer;
pub mod parser;

pub struct AST {
    pub statements: Vec<ASTStatement>,
}

impl AST {
    pub fn new() -> Self {
        AST {
            statements: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, statement: ASTStatement) {
        self.statements.push(statement);
    }

    pub fn visit(&self, visitor: &mut dyn ASTVisitor) {
        for statement in self.statements.iter() {
            visitor.visit_statement(statement)
        }
    }

    pub fn visualize(&self) {
        let mut printer = ASTPrinter { indent: 0 };

        self.visit(&mut printer);
    }
}

pub trait ASTVisitor {
    fn do_visit_statement(&mut self, statement: &ASTStatement) {
        match &statement.kind {
            ASTStatementKind::Expression(expr) => {
                self.visit_expression(expr);
            }
        }
    }

    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.do_visit_statement(statement);
    }

    fn do_visit_expression(&mut self, expr: &ASTExpression) {
        match &expr.kind {
            ASTExpressionKind::Number(number) => {
                self.visit_number(number);
            }
        }
    }

    fn visit_expression(&mut self, expr: &ASTExpression) {
        self.do_visit_expression(expr);
    }

    fn visit_number(&mut self, number: &ASTNumberExpression);
}

pub struct ASTPrinter {
    indent: usize,
}

const LEVEL_INDENT: usize = 2;

impl ASTVisitor for ASTPrinter {
    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.print_width_indent("Statement: ");

        self.indent += LEVEL_INDENT;

        ASTVisitor::do_visit_statement(self, statement);

        self.indent -= LEVEL_INDENT;
    }

    fn visit_expression(&mut self, expr: &ASTExpression) {
        self.print_width_indent("Expression: ");

        self.indent += LEVEL_INDENT;

        ASTVisitor::do_visit_expression(self, expr);

        self.indent -= LEVEL_INDENT;
    }

    fn visit_number(&mut self, number: &ASTNumberExpression) {
        self.print_width_indent(&format!("Number: {}", number.number));
    }
}

impl ASTPrinter {
    fn print_width_indent(&mut self, text: &str) {
        println!("{}{}", " ".repeat(self.indent), text);
    }
}

pub enum ASTStatementKind {
    Expression(ASTExpression),
}

pub struct ASTStatement {
    kind: ASTStatementKind,
}

impl ASTStatement {
    pub fn new(kind: ASTStatementKind) -> Self {
        ASTStatement { kind }
    }

    pub fn expression(expr: ASTExpression) -> Self {
        ASTStatement::new(ASTStatementKind::Expression(expr))
    }
}

pub enum ASTExpressionKind {
    Number(ASTNumberExpression),
}

pub struct ASTNumberExpression {
    number: i64,
}

pub struct ASTExpression {
    kind: ASTExpressionKind,
}

impl ASTExpression {
    pub fn new(kind: ASTExpressionKind) -> Self {
        ASTExpression { kind }
    }

    pub fn number(number: i64) -> Self {
        ASTExpression::new(ASTExpressionKind::Number(ASTNumberExpression { number }))
    }
}

use lexer::Token;

pub mod evaluator;
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

    fn do_visit_expression(&mut self, expr: &ASTExpression) {
        match &expr.kind {
            ASTExpressionKind::Number(number) => {
                self.visit_number(number);
            }
            ASTExpressionKind::Binary(expr) => {
                self.visit_binary_expression(expr);
            }
            ASTExpressionKind::Parenthesized(expr) => {
                self.visit_parenthesized_expression(expr);
            }
        }
    }

    fn do_visit_binary_expression(&mut self, expr: &ASTBinaryExpression) {
        self.do_visit_expression(&expr.left);
        self.do_visit_expression(&expr.right);
    }

    fn do_visit_parenthesized_expression(&mut self, expr: &ASTParenthesizedExpression) {
        self.do_visit_expression(&expr.expression);
    }

    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.do_visit_statement(statement);
    }

    fn visit_expression(&mut self, expr: &ASTExpression) {
        self.do_visit_expression(expr);
    }

    fn visit_binary_expression(&mut self, expr: &ASTBinaryExpression) {
        self.do_visit_binary_expression(expr);
    }

    fn visit_parenthesized_expression(&mut self, expr: &ASTParenthesizedExpression) {
        self.do_visit_parenthesized_expression(expr);
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

    fn visit_binary_expression(&mut self, expr: &ASTBinaryExpression) {
        self.print_width_indent("Binary Expression: ");

        self.indent += LEVEL_INDENT;

        self.print_width_indent(&format!("Operator: {:?}", expr.operator.kind));
        self.visit_expression(&expr.left);
        self.visit_expression(&expr.right);

        self.indent -= LEVEL_INDENT;
    }

    fn visit_parenthesized_expression(&mut self, expr: &ASTParenthesizedExpression) {
        self.print_width_indent("Parenthesized Expression: ");

        self.indent += LEVEL_INDENT;

        self.visit_expression(&expr.expression);

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
    Binary(ASTBinaryExpression),
    Parenthesized(ASTParenthesizedExpression),
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

    pub fn binary(operator: ASTBinaryOperator, left: ASTExpression, right: ASTExpression) -> Self {
        ASTExpression::new(ASTExpressionKind::Binary(ASTBinaryExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }

    pub fn parenthesized(expression: ASTExpression) -> Self {
        ASTExpression::new(ASTExpressionKind::Parenthesized(
            ASTParenthesizedExpression {
                expression: Box::new(expression),
            },
        ))
    }
}

#[derive(Debug)]
pub enum ASTBinaryOperatorKind {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub struct ASTBinaryOperator {
    kind: ASTBinaryOperatorKind,
    token: Token,
}

impl ASTBinaryOperator {
    pub fn new(kind: ASTBinaryOperatorKind, token: Token) -> Self {
        Self { kind, token }
    }

    pub fn precedence(&self) -> u8 {
        match self.kind {
            ASTBinaryOperatorKind::Add => 1,
            ASTBinaryOperatorKind::Subtract => 1,
            ASTBinaryOperatorKind::Multiply => 2,
            ASTBinaryOperatorKind::Divide => 2,
        }
    }
}

pub struct ASTBinaryExpression {
    left: Box<ASTExpression>,
    operator: ASTBinaryOperator,
    right: Box<ASTExpression>,
}
pub struct ASTNumberExpression {
    number: i64,
}

pub struct ASTParenthesizedExpression {
    expression: Box<ASTExpression>,
}

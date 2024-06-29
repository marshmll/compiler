use super::{ASTBinaryExpression, ASTBinaryOperatorKind, ASTNumberExpression, ASTVisitor};

pub struct Evaluator {
    pub last_value: Option<i64>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self { last_value: None }
    }
}

impl ASTVisitor for Evaluator {
    fn visit_number(&mut self, number: &ASTNumberExpression) {
        self.last_value = Some(number.number);
    }

    fn visit_binary_expression(&mut self, expr: &ASTBinaryExpression) {
        self.visit_expression(&expr.left);
        let left = self.last_value.unwrap();

        self.visit_expression(&expr.right);
        let right = self.last_value.unwrap();

        self.last_value = Some(match expr.operator.kind {
            ASTBinaryOperatorKind::Add => left + right,
            ASTBinaryOperatorKind::Subtract => left - right,
            ASTBinaryOperatorKind::Multiply => left * right,
            ASTBinaryOperatorKind::Divide => left / right,
        })
    }
}

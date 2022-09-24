use crate::ast;

pub trait Visitor {
    type Output;
    fn visit(&self, node: Box<ast::Node>) -> Self::Output;
}

pub struct PostOrderVisitor;

impl Visitor for PostOrderVisitor {
    type Output = i32;
    fn visit(&self, node: Box<ast::Node>) -> i32 {
        match *node {
            ast::Node::Num(num) => num,
            ast::Node::BinOp { op, left, right } => match op {
                ast::Operator::Add => self.visit(left) + self.visit(right),
                ast::Operator::Subtract => self.visit(left) - self.visit(right),
                ast::Operator::Multiply => self.visit(left) * self.visit(right),
                ast::Operator::Divide => {
                    let right = self.visit(right);
                    if right == 0 {
                        panic!("divide 0")
                    } else {
                        self.visit(left) / right
                    }
                }
            },
        }
    }
}

pub struct RPNVisitor;

impl Visitor for RPNVisitor {
    type Output = String;
    fn visit(&self, node: Box<ast::Node>) -> Self::Output {
        match *node {
            ast::Node::Num(num) => num.to_string(),
            ast::Node::BinOp { op, left, right } => match op {
                ast::Operator::Add => format!("{} {} +", self.visit(left), self.visit(right)),
                ast::Operator::Subtract => format!("{} {} -", self.visit(left), self.visit(right)),
                ast::Operator::Multiply => format!("{} {} *", self.visit(left), self.visit(right)),
                ast::Operator::Divide => format!("{} {} /", self.visit(left), self.visit(right)),
            },
        }
    }
}

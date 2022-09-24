use crate::ast;

pub trait Visitor {
    fn visit(&self, node: Box<ast::Node>) -> i32;
}

pub struct PostOrderVisitor;

impl Visitor for PostOrderVisitor {
    fn visit(&self, node: Box<ast::Node>) -> i32 {
        match *node {
            ast::Node::Num(num) => num,
            ast::Node::BinOp{op, left, right} => {
                match op {
                    ast::Operator::Add => {
                        self.visit(left) + self.visit(right)
                    }
                    ast::Operator::Subtract => {
                        self.visit(left) - self.visit(right)
                    }
                    ast::Operator::Multiply => {
                        self.visit(left) * self.visit(right)
                    }
                    ast::Operator::Divide => {
                        let right = self.visit(right);
                        if right == 0 {
                            panic!("divide 0")
                        } else {
                            self.visit(left) / right
                        }
                    }
                }
            }
        }
    }
}

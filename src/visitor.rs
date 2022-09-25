use std::collections::HashMap;

use crate::ast;

pub trait Visitor {
    type Output;
    fn visit(&self, node: Box<ast::Node>, env: &mut HashMap<String, i32>) -> Self::Output;
}

pub struct PostOrderVisitor;

impl Visitor for PostOrderVisitor {
    type Output = i32;
    fn visit(&self, node: Box<ast::Node>, env: &mut HashMap<String, i32>) -> i32 {
        match *node {
            ast::Node::Num(num) => num,
            ast::Node::UnaryOp { op, operand } => match op {
                ast::Operator::Add => self.visit(operand, env),
                ast::Operator::Subtract => -self.visit(operand, env),
                _ => panic!("invalid operation {:?}", op)
            }
            ast::Node::BinOp { op, left, right } => match op {
                ast::Operator::Add => self.visit(left, env) + self.visit(right, env),
                ast::Operator::Subtract => self.visit(left, env) - self.visit(right, env),
                ast::Operator::Multiply => self.visit(left, env) * self.visit(right, env),
                ast::Operator::Divide => {
                    let right = self.visit(right, env);
                    if right == 0 {
                        panic!("divide 0")
                    } else {
                        self.visit(left, env) / right
                    }
                }
            },
            ast::Node::Compound { children } => {
                for child in children {
                    self.visit(child, env);
                }
                0
            }
            ast::Node::Assign { left, right } => {
                if let ast::Node::Var(s) = *left {
                    let evaled = self.visit(right, env);
                    env.insert(s, evaled);
                }
                0
            }
            ast::Node::Var(id) => {
                if env.contains_key(&id) {
                    env.get(&id);
                    0
                } else {
                    panic!("undefined variable: {}", id)
                }
            }
            ast::Node::NoOp => {
                0
            }
        }
    }
}

// pub struct RPNVisitor;

// impl Visitor for RPNVisitor {
//     type Output = String;
//     fn visit(&self, node: Box<ast::Node>) -> Self::Output {
//         match *node {
//             ast::Node::Num(num) => num.to_string(),
//             ast::Node::BinOp { op, left, right } => match op {
//                 ast::Operator::Add => format!("{} {} +", self.visit(left), self.visit(right)),
//                 ast::Operator::Subtract => format!("{} {} -", self.visit(left), self.visit(right)),
//                 ast::Operator::Multiply => format!("{} {} *", self.visit(left), self.visit(right)),
//                 ast::Operator::Divide => format!("{} {} /", self.visit(left), self.visit(right)),
//             },
//         }
//     }
// }

// pub struct LispStyleVisitor;

// impl Visitor for LispStyleVisitor {
//     type Output = String;
//     fn visit(&self, node: Box<ast::Node>) -> Self::Output {
//         match *node {
//             ast::Node::Num(num) => num.to_string(),
//             ast::Node::BinOp { op, left, right } => match op {
//                 ast::Operator::Add => format!("(+ {} {})", self.visit(left), self.visit(right)),
//                 ast::Operator::Subtract => format!("(- {} {})", self.visit(left), self.visit(right)),
//                 ast::Operator::Multiply => format!("(* {} {})", self.visit(left), self.visit(right)),
//                 ast::Operator::Divide => format!("(/ {} {})", self.visit(left), self.visit(right)),
//             },
//         }
//     }
// }
use std::collections::HashMap;
use crate::{parser::Parser, visitor::Visitor};

pub struct Intepreter<T, V: Visitor<Output = T>> {
    parser: Parser,
    visitor: V,
    env: HashMap<String, i32>,
}

impl<T, V: Visitor<Output = T>> Intepreter<T, V> {
    pub fn new(text: String, visitor: V) -> Self {
        Intepreter {
            parser: Parser::new(text),
            visitor: visitor,
            env: HashMap::new()
        }
    }

    pub fn eval(&mut self) -> T {
        let node = self.parser.parse();
        self.visitor.visit(node, &mut self.env)
    }

    pub fn get_env(&self) -> &HashMap<String, i32> {
        &self.env
    }
}


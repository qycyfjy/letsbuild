use crate::{parser::Parser, visitor::Visitor};

pub struct Intepreter<T, V: Visitor<Output = T>> {
    parser: Parser,
    visitor: V
}

impl<T, V: Visitor<Output = T>> Intepreter<T, V> {
    pub fn new(text: String, visitor: V) -> Self {
        Intepreter {
            parser: Parser::new(text),
            visitor: visitor
        }
    }

    pub fn eval(& mut self) -> T {
        let node = self.parser.parse();
        self.visitor.visit(node)
    }
}


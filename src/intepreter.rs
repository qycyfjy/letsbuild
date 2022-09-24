use crate::{parser::Parser, visitor::{Visitor, PostOrderVisitor}};

pub struct Intepreter {
    parser: Parser,
    visitor: PostOrderVisitor
}

impl Intepreter {
    pub fn new(text: String) -> Self {
        Intepreter {
            parser: Parser::new(text),
            visitor: PostOrderVisitor{}
        }
    }

    pub fn eval(& mut self) -> i32 {
        let node = self.parser.parse();
        self.visitor.visit(node)
    }
}


use crate::{lexer::Lexer, token::Token, ast};

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(text: String) -> Self {
        let mut p = Self {
            lexer: Lexer::new(text),
            current_token: None,
        };
        p.current_token = p.lexer.get_next_token();
        p
    }

    pub fn parse(&mut self) -> Box<ast::Node> {
        self.expr()
    }
}

impl Parser {
    fn error(&self) -> ! {
        panic!("invalid syntax")
    }

    fn eat(&mut self, expect: Token) {
        if let Some(token) = &self.current_token {
            if std::mem::discriminant(token) == std::mem::discriminant(&expect) {
                self.current_token = self.lexer.get_next_token();
                return
            }
        }
        self.error()
    }

    // factor : INTEGER | LPAREN expr RPAREN | (PLUS | MINUS) factor
    fn factor(&mut self) -> Box<ast::Node> {
        match &self.current_token {
            Some(Token::Plus) => {
                self.eat(Token::tt_plus());
                Box::new(ast::Node::UnaryOp { op: ast::Operator::Add, operand: self.factor() })
            }
            Some(Token::Minus) => {
                self.eat(Token::tt_minus());
                Box::new(ast::Node::UnaryOp { op: ast::Operator::Subtract, operand: self.factor() })
            }
            Some(Token::Integer(num)) => {
                let num = *num;
                self.eat(Token::tt_integer());
                Box::new(ast::Node::Num(num))
            }
            Some(Token::LParen) => {
                self.eat(Token::tt_lparen());
                let node = self.expr();
                self.eat(Token::tt_rparen());
                node
            }
            _ => {
                self.error()
            }
        }
    }

    // term : factor ((MUL | DIV) factor)*
    fn term(&mut self) -> Box<ast::Node> {
        let mut left = self.factor();

        while let Some(token) = &self.current_token {
            match token {
                Token::Multiply => {
                    self.eat(Token::tt_multiply());
                    left = Box::new(ast::Node::BinOp { op: ast::Operator::Multiply, left, right: self.factor() })
                }
                Token::Divide => {
                    self.eat(Token::tt_divide());
                    left = Box::new(ast::Node::BinOp { op: ast::Operator::Divide, left, right: self.factor() })
                }
                _ => {
                    break
                }
            }
        } 
        left
    }

    // expr : term ((PLUS | MINUS) term)*
    fn expr(&mut self) -> Box<ast::Node> {
        let mut left = self.term();

        while let Some(token) = &self.current_token {
            match token {
                Token::Plus => {
                    self.eat(Token::tt_plus());
                    left = Box::new(ast::Node::BinOp { op: ast::Operator::Add, left, right: self.term() });
                }
                Token::Minus => {
                    self.eat(Token::tt_minus());
                    left = Box::new(ast::Node::BinOp { op: ast::Operator::Subtract, left, right: self.term() });
                }
                _ => {
                    break;
                }
            }
        }
        left
    }
}

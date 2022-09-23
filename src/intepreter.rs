use crate::{token::Token, lexer::Lexer};

pub struct Intepreter {
    lexer: Lexer,
}

impl Intepreter {
    pub fn new(text: String) -> Self {
        let mut i = Intepreter {
            lexer: Lexer::new(text),
        };
        i.lexer.get_next_token();
        i
    }
}

// Parser
impl Intepreter {
    fn factor(&mut self) -> i32 {
        if let Some(Token::Integer(num)) = self.lexer.current_token().take() {
            self.lexer.get_next_token();
            num
        } else {
            panic!("expect a integer")
        }
    }

    fn term(&mut self) -> i32 {
        let mut left = self.factor();
        while let Some(token) = self.lexer.current_token() {
            match token {
                Token::Multiply => {
                    self.lexer.get_next_token();
                    left *= self.factor()
                }
                Token::Divide => {
                    self.lexer.get_next_token();
                    let right = self.factor();
                    if right == 0 {
                        panic!("divide 0");
                    }
                    left /= right;
                }
                _ => break,
            }
        }
        left
    }

    pub fn expr(&mut self) -> i32 {
        let mut left = self.term();
        while let Some(token) = self.lexer.current_token() {
            match token {
                Token::Plus => {
                    self.lexer.get_next_token();
                    left += self.term()
                }
                Token::Subtract => {
                    self.lexer.get_next_token();
                    left -= self.term()
                }
                _ => break,
            }
        }
        left
    }
}


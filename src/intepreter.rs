use crate::token::Token;

pub struct Intepreter {
    text: String,
    pos: usize,
    current_token: Option<Token>,
}

impl Intepreter {
    pub fn new(text: String) -> Self {
        let mut i = Intepreter {
            text,
            pos: 0,
            current_token: None,
        };
        i.get_next_token();
        i
    }
}

// Parser
impl Intepreter {
    fn factor(&mut self) -> i32 {
        if let Some(Token::Integer(num)) = self.current_token.take() {
            self.get_next_token();
            num
        } else {
            panic!("expect a integer")
        }
    }

    fn term(&mut self) -> i32 {
        let mut left = self.factor();
        while let Some(token) = &self.current_token {
            match token {
                Token::Multiply => {
                    self.get_next_token();
                    left *= self.factor()
                }
                Token::Divide => {
                    self.get_next_token();
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
        while let Some(token) = &self.current_token {
            match token {
                Token::Plus => {
                    self.get_next_token();
                    left += self.term()
                }
                Token::Subtract => {
                    self.get_next_token();
                    left -= self.term()
                }
                _ => break,
            }
        }
        left
    }
}

// Lexer
impl Intepreter {
    fn advance(&mut self) {
        self.pos += 1;
    }

    fn get_next_token(&mut self) {
        self.skip_whitespaces();

        if let Some(current_char) = self.text.chars().nth(self.pos) {
            match current_char {
                c if c.is_digit(10) => {
                    self.current_token = Some(self.get_integer());
                }
                c if c == '+' => {
                    self.advance();
                    self.current_token = Some(Token::Plus);
                }
                c if c == '-' => {
                    self.advance();
                    self.current_token = Some(Token::Subtract);
                }
                c if c == '*' => {
                    self.advance();
                    self.current_token = Some(Token::Multiply);
                }
                c if c == '/' => {
                    self.advance();
                    self.current_token = Some(Token::Divide);
                }
                _ => unreachable!(),
            }
        } else {
            self.current_token = Some(Token::Eof)
        }
    }

    fn get_integer(&mut self) -> Token {
        let mut integer = 0;
        while let Some(current_char) = self.text.chars().nth(self.pos) {
            if let Some(digit) = current_char.to_digit(10) {
                let digit = digit as i32;
                integer = integer * 10 + digit;
                self.advance()
            } else {
                break;
            }
        }
        Token::Integer(integer)
    }

    fn skip_whitespaces(&mut self) {
        while let Some(current_char) = self.text.chars().nth(self.pos) {
            if current_char.is_whitespace() {
                self.advance()
            } else {
                break;
            }
        }
    }
}

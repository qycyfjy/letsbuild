use crate::token::Token;

pub struct Lexer {
    text: String,
    pos: usize,
    current_token: Option<Token>,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Lexer {
            text,
            pos: 0,
            current_token: None,
        }
    }

    pub fn current_token(&mut self) -> &mut Option<Token> {
        &mut self.current_token
    }

    pub fn get_next_token(&mut self) {
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
                c if c == '(' => {
                    self.advance();
                    self.current_token = Some(Token::LParen);
                }
                c if c == ')' => {
                    self.advance();
                    self.current_token = Some(Token::RParen);
                }
                _ => unreachable!(),
            }
        } else {
            self.current_token = Some(Token::Eof)
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
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
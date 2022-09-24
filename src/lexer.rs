use crate::token::Token;

pub struct Lexer {
    text: String,
    pos: usize,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Lexer {
            text,
            pos: 0,
        }
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        self.skip_whitespaces();

        if let Some(current_char) = self.text.chars().nth(self.pos) {
            match current_char {
                c if c.is_digit(10) => {
                    Some(self.get_integer())
                }
                c if c == '+' => {
                    self.advance();
                    Some(Token::Plus)
                }
                c if c == '-' => {
                    self.advance();
                    Some(Token::Minus)
                }
                c if c == '*' => {
                    self.advance();
                    Some(Token::Multiply)
                }
                c if c == '/' => {
                    self.advance();
                    Some(Token::Divide)
                }
                c if c == '(' => {
                    self.advance();
                    Some(Token::LParen)
                }
                c if c == ')' => {
                    self.advance();
                    Some(Token::RParen)
                }
                _ => unreachable!(),
            }
        } else {
            Some(Token::Eof)
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
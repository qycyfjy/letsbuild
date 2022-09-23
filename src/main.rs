use std::io::{self, Write};

enum Token {
    Integer(i32),
    Plus,
    Subtract,
    Multiply,
    Divide,
    Eof,
}

struct Intepreter {
    text: String,
    pos: usize,
    current_token: Option<Token>,
}

impl Intepreter {
    fn new(text: String) -> Self {
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

    fn expr(&mut self) -> i32 {
        let mut left = self.factor();
        while let Some(token) = self.current_token.take() {
            self.get_next_token();
            match token {
                // Token::Plus => left += self.factor(),
                // Token::Subtract => left -= self.factor(),
                Token::Multiply => left *= self.factor(),
                Token::Divide => {
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

fn main() {
    let stdin = io::stdin();
    loop {
        print!("calc> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let mut i = Intepreter::new(line);
        println!("{}", i.expr());
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn intepret(text: String) -> i32 {
        let mut i = Intepreter::new(text);
        return i.expr();
    }

    #[test]
    fn test_single_digit_ints_add() {
        assert_eq!(intepret("1+1".to_string()), 2);
        assert_eq!(intepret("0+0".to_string()), 0);
        assert_eq!(intepret("9+9".to_string()), 18);
    }

    #[test]
    fn test_multi_digits_ints_add() {
        assert_eq!(intepret("11+1".to_string()), 12);
        assert_eq!(intepret("1010101+2222".to_string()), 1012323);
        assert_eq!(intepret("99+100".to_string()), 199);
    }

    #[test]
    fn test_add_with_spaces() {
        assert_eq!(intepret("   11 +   1".to_string()), 12);
        assert_eq!(intepret("1010101  + 2222".to_string()), 1012323);
        assert_eq!(intepret("99 + 100".to_string()), 199);
    }

    #[test]
    fn test_minus() {
        assert_eq!(intepret("   11 -   1".to_string()), 10);
        assert_eq!(intepret("1012222-2222".to_string()), 1010000);
        assert_eq!(intepret("99 - 100".to_string()), -1);
    }

    #[test]
    fn test_only_add_minus() {
        assert_eq!(intepret("1 + 2 - 3 - 4".to_string()), -4);
        assert_eq!(intepret("2 - 2 + 3 - 3".to_string()), 0);
        assert_eq!(intepret("1 + 1 + 1 + 1 + 1".to_string()), 5);
        assert_eq!(intepret("9+9+9-9-9".to_string()), 9);
    }

    #[test]
    fn test_only_multiply_divide() {
        assert_eq!(intepret("1 * 2 * 3 / 4".to_string()), 1);
        assert_eq!(intepret("2 / 2 * 3 / 3".to_string()), 1);
        assert_eq!(intepret("1 / 1 / 1 / 1 / 1".to_string()), 1);
        assert_eq!(intepret("9*9*9*9/9".to_string()), 729);
    }
}

use std::{
    fmt::Display,
    io::{self, Write},
};

enum Token {
    Integer(i32),
    Plus,
    Eof,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Integer(num) => write!(f, "Token(INTEGER, {})", num),
            Token::Plus => write!(f, "Token(PLUS, )"),
            Token::Eof => write!(f, "Token(EOF, )"),
        }
    }
}

struct Intepreter {
    text: String,
    pos: usize,
    current_token: Option<Token>,
}

impl Intepreter {
    fn new(text: String) -> Self {
        Intepreter {
            text,
            pos: 0,
            current_token: None,
        }
    }

    fn get_next_token(&mut self) {
        self.pos = self.skip_whitespaces(self.pos);
        if let Some(current_char) = self.text.chars().nth(self.pos) {
            if let Some(_) = current_char.to_digit(10) {
                let (num, pos) = self.get_integer(self.pos);
                self.pos = pos;
                self.current_token = Some(Token::Integer(num));
                return;
            }

            if current_char == '+' {
                self.pos += 1;
                self.current_token = Some(Token::Plus);
                return;
            }
        }

        self.current_token = Some(Token::Eof)
    }

    // fn eat(&mut self, tt: Token) {
    //     if let Some(t) = self.current_token {
    //         if std::mem::discriminant(&tt) == std::mem::discriminant(&t) {
    //             self.get_next_token()
    //         } else {
    //             self.error()
    //         }
    //     }
    // }

    fn expr(&mut self) -> i32 {
        self.get_next_token();

        if let Some(Token::Integer(left)) = self.current_token {
            self.get_next_token();
            self.get_next_token();
            if let Some(Token::Integer(right)) = self.current_token {
                self.get_next_token();
                left + right
            } else {
                panic!("error input")
            }
        } else {
            panic!("error input")
        }
    }
}

impl Intepreter {
    fn get_integer(&self, mut idx: usize) -> (i32, usize) {
        let mut integer = 0;
        while let Some(current_char) = self.text.chars().nth(idx) {
            if let Some(digit) = current_char.to_digit(10) {
                let digit = digit as i32;
                integer = integer * 10 + digit;
                idx += 1;
            } else {
                break;
            }
        }
        (integer, idx)
    }

    fn skip_whitespaces(&self, mut idx: usize) -> usize {
        while let Some(current_char) = self.text.chars().nth(idx) {
            if current_char.is_whitespace() {
                idx += 1;
            } else {
                break;
            }
        }
        idx
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

pub enum Token {
    Integer(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
    Eof,
}

impl Token {
    #[inline]
    pub fn tt_integer() -> Token {
        Token::Integer(0)
    }

    #[inline]
    pub fn tt_plus() -> Token {
        Token::Plus
    }

    #[inline]
    pub fn tt_minus() -> Token {
        Token::Minus
    }

    #[inline]
    pub fn tt_multiply() -> Token {
        Token::Multiply
    }

    #[inline]
    pub fn tt_divide() -> Token {
        Token::Divide
    }

    #[inline]
    pub fn tt_lparen() -> Token {
        Token::LParen
    }

    #[inline]
    pub fn tt_rparen() -> Token {
        Token::RParen
    }
}
pub enum Token {
    Integer(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,

    Begin,
    End,
    Dot,
    Id(String),
    Assign,
    Semi,

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

    #[inline]
    pub fn tt_begin() -> Token {
        Token::Begin
    }

    #[inline]
    pub fn tt_end() -> Token {
        Token::End
    }

    #[inline]
    pub fn tt_dot() -> Token {
        Token::Dot
    }

    #[inline]
    pub fn tt_id() -> Token {
        Token::Id(String::new())
    }

    #[inline]
    pub fn tt_assign() -> Token {
        Token::Assign
    }

    #[inline]
    pub fn tt_semi() -> Token {
        Token::Semi
    }
}
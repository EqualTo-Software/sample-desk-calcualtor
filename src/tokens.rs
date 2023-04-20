use std::fmt;

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
    Power,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Number(f64),
    Name(String),
    OpenParenthesis,
    CloseParenthesis,
    Plus,
    Minus,
    Times,
    Divide,
    Power,
    Illegal(String),
    EoI,
}

impl fmt::Display for Token {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Illegal(s) => write!(fmt, "Illegal Token: {}", s),
            Token::EoI => write!(fmt, "End Of Input"),
            Token::Number(f) => write!(fmt, "{}", f),
            Token::Name(s) => write!(fmt, "{}", s),
            Token::OpenParenthesis => write!(fmt, "("),
            Token::CloseParenthesis => write!(fmt, ")"),
            Token::Plus => write!(fmt, "+"),
            Token::Minus => write!(fmt, "-"),
            Token::Times => write!(fmt, "*"),
            Token::Divide => write!(fmt, "/"),
            Token::Power => write!(fmt, "^"),
        }
    }
}

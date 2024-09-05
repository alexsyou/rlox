use crate::tokentype::TokenType;
use std::fmt;

struct Token {
    atype: TokenType,
    lexeme: String,
    literal: String,
    line: i32
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {}", self.atype, self.lexeme, self.literal)
    }
}

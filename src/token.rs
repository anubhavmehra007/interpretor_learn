#[derive(Debug)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT(String),
    INT(String),
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
}
impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TokenType::INT(val1), TokenType::INT(val2)) => val1 == val2,
            (TokenType::IDENT(val1), TokenType::IDENT(val2)) => val1 == val2,
            (TokenType::ILLEGAL, TokenType::ILLEGAL) => true,
            (TokenType::EOF, TokenType::EOF) => true,
            (TokenType::ASSIGN, TokenType::ASSIGN) => true,
            (TokenType::PLUS, TokenType::PLUS) => true,
            (TokenType::COMMA, TokenType::COMMA) => true,
            (TokenType::SEMICOLON, TokenType::SEMICOLON) => true,
            (TokenType::LPAREN, TokenType::LPAREN) => true,
            (TokenType::RPAREN, TokenType::RPAREN) => true,
            (TokenType::LBRACE, TokenType::LBRACE) => true,
            (TokenType::RBRACE, TokenType::RBRACE) => true,
            (TokenType::FUNCTION, TokenType::FUNCTION) => true,
            (TokenType::LET, TokenType::LET) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_equal() {
        assert_eq!(TokenType::LET, TokenType::LET)
    }
    #[test]
    fn test_uneual() {
        assert_ne!(TokenType::LET, TokenType::RBRACE)
    }
    #[test]
    fn test_equal_value() {
        assert_eq!(
            TokenType::IDENT(String::from("abc")),
            TokenType::IDENT(String::from("abc"))
        );
    }
    #[test]
    fn test_unequal_value() {
        assert_ne!(
            TokenType::IDENT(String::from("labc")),
            TokenType::IDENT(String::from("abc"))
        );
    }
}

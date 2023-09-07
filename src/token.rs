
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
        match(self, other) {
            (TokenType::INT(val1), TokenType::INT(val2)) => {
                val1 == val2
            },
            (TokenType::IDENT(val1), TokenType::IDENT(val2)) => {
                val1 == val2
            },
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
            _ => false
        }
        }
       } 
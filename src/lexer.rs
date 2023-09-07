use crate::TokenType;
use std::str;
#[derive(Debug)]
pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn from(input: &str) -> Lexer {
        let mut lexer = Lexer {
            input: Vec::from(input.as_bytes()),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lexer.read_char();
        lexer
    }
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    pub fn next_token(&mut self) -> TokenType {
        let token;
        self.skip_whitespace();
        let ch = self.ch as char;
        match ch {
            '=' => token = TokenType::ASSIGN,
            ';' => token = TokenType::SEMICOLON,
            '(' => token = TokenType::LPAREN,
            ')' => token = TokenType::RPAREN,
            '{' => token = TokenType::LBRACE,
            '}' => token = TokenType::RBRACE,
            ',' => token = TokenType::COMMA,
            '+' => token = TokenType::PLUS,
            _ => {
                if self.is_letter() {
                    let str = self.read_indentifier();
                    match str {
                        "let" => token = TokenType::LET,
                        "fn" => token = TokenType::FUNCTION,
                        _ => token = TokenType::IDENT(String::from(str)),
                    };
                    return token;
                } else if self.is_digit() {
                    let str = self.read_digit();
                    token = TokenType::INT(String::from(str));
                    return token;
                } else if ch == '\0' {
                    token = TokenType::EOF
                } else {
                    token = TokenType::ILLEGAL
                }
            }
        }
        self.read_char();
        token
    }
    fn is_letter(&self) -> bool {
        let ch = self.ch as char;
        ('a' <= ch && ch <= 'z') || ('A' <= ch && ch <= 'Z') || ch == '_'
    }
    fn is_digit(&self) -> bool {
        let ch = self.ch as char;
        ('0' <= ch && ch <= '9') || ch == '.'
    }
    fn read_digit(&mut self) -> &str {
        let position = self.position;
        while self.is_digit() {
            self.read_char()
        }
        let s = match str::from_utf8(&(self.input)[position..self.position]) {
            Ok(v) => v,
            Err(_e) => panic!("Invalid sequence of charcters"),
        };
        s
    }
    fn read_indentifier(&mut self) -> &str {
        let position = self.position;
        while self.is_letter() {
            self.read_char()
        }
        let s = match str::from_utf8(&(self.input)[position..self.position]) {
            Ok(v) => v,
            Err(_e) => panic!("Invalid sequence of charcters"),
        };
        s
    }
    fn skip_whitespace(&mut self) {
        let mut ch = self.ch as char;
        while ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
            self.read_char();
            ch = self.ch as char;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        let input = "=+(){},;";
        let mut output: Vec<TokenType> = vec![];
        let correct_output: Vec<TokenType> = vec![
            TokenType::ASSIGN,
            TokenType::PLUS,
            TokenType::LPAREN,
            TokenType::RPAREN,
            TokenType::LBRACE,
            TokenType::RBRACE,
            TokenType::COMMA,
            TokenType::SEMICOLON,
            TokenType::EOF,
        ];
        let mut lexer = Lexer::from(input);
        let mut token = lexer.next_token();
        while token != TokenType::EOF {
            output.push(token);
            token = lexer.next_token();
        }
        output.push(token);
        assert_eq!(output, correct_output);
    }

    #[test]
    fn test_2() {
        let input = "
            let five = 5;
            let ten = 10.2;
            let add = fn(x,y) {
                x + y;
            };

            let result = add(five, ten);
        ";
        let mut output: Vec<TokenType> = vec![];
        let correct_output: Vec<TokenType> = vec![
            TokenType::LET,
            TokenType::IDENT(String::from("five")),
            TokenType::ASSIGN,
            TokenType::INT(String::from("5")),
            TokenType::SEMICOLON,
            TokenType::LET,
            TokenType::IDENT(String::from("ten")),
            TokenType::ASSIGN,
            TokenType::INT(String::from("10.2")),
            TokenType::SEMICOLON,
            TokenType::LET,
            TokenType::IDENT(String::from("add")),
            TokenType::ASSIGN,
            TokenType::FUNCTION,
            TokenType::LPAREN,
            TokenType::IDENT(String::from("x")),
            TokenType::COMMA,
            TokenType::IDENT(String::from("y")),
            TokenType::RPAREN,
            TokenType::LBRACE,
            TokenType::IDENT(String::from("x")),
            TokenType::PLUS,
            TokenType::IDENT(String::from("y")),
            TokenType::SEMICOLON,
            TokenType::RBRACE,
            TokenType::SEMICOLON,
            TokenType::LET,
            TokenType::IDENT(String::from("result")),
            TokenType::ASSIGN,
            TokenType::IDENT(String::from("add")),
            TokenType::LPAREN,
            TokenType::IDENT(String::from("five")),
            TokenType::COMMA,
            TokenType::IDENT(String::from("ten")),
            TokenType::RPAREN,
            TokenType::SEMICOLON,
            TokenType::EOF,
        ];
        let mut lexer = Lexer::from(input);
        let mut token = lexer.next_token();
        while token != TokenType::EOF {
            output.push(token);
            token = lexer.next_token();
        }
        output.push(token);
        assert_eq!(output, correct_output);
    }
}

use lexer::Lexer;
use token::TokenType;
pub mod lexer;
pub mod token;
fn main() {
        let input = "=+(){},";
        let mut lexer  = Lexer::from(input);
        dbg!(lexer.next_token());
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_1() {
        let input = "=+(){},;";
        let mut output : Vec<TokenType> = vec![];
        let correct_output: Vec<TokenType> = vec![
            TokenType::ASSIGN,
            TokenType::PLUS,
            TokenType::LPAREN,
            TokenType::RPAREN,
            TokenType::LBRACE,
            TokenType::RBRACE,
            TokenType::COMMA,
            TokenType::SEMICOLON,
            TokenType::EOF
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
        let mut output : Vec<TokenType> = vec![];
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
            TokenType::EOF
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
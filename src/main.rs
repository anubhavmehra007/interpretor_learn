use lexer::Lexer;
use token::TokenType;
pub mod lexer;
pub mod repl;
pub mod token;
fn main() {
    let mut tokens = vec![];
    loop {
        tokens.clear();
        match repl::repl() {
            Some(input) => {
                if input == "ziglord" {
                    break;
                } else {
                    let mut lexer = Lexer::from(&input);
                    loop {
                        let token = lexer.next_token();
                        match token {
                            TokenType::EOF => {
                                tokens.push(token);
                                break;
                            }
                            _ => tokens.push(token),
                        }
                    }
                    println!("{:?}", tokens);
                }
            }
            None => continue,
        }
    }
}

#[cfg(test)]
mod test;

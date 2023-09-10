use crate::lexer::{Lexer, TokenType};
use std::io::{self, Write};
#[derive(Debug)]
pub struct Repl<'a> {
    tokens: Vec<TokenType>,
    exit_word: &'a str,
}
impl<'a> Repl<'a> {
    pub fn new(exit_word: &'a str) -> Repl<'a> {
        Repl {
            tokens: vec![],
            exit_word,
        }
    }
    fn prompt_and_input(&self) -> Option<String> {
        let prompt = ">>";
        let mut line = String::new();
        print!("{}", prompt);
        let _ = io::stdout().flush();
        let b1 = std::io::stdin().read_line(&mut line).unwrap();
        line = String::from(line.trim());
        if b1 > 2 {
            Some(line)
        } else {
            None
        }
    }
    pub fn start(&mut self) {
        loop {
            self.tokens.clear();
            match self.prompt_and_input() {
                Some(input) => {
                    if input == self.exit_word {
                        break;
                    } else {
                        self.interpret(&input);
                    }
                }
                None => continue,
            }
        }
    }
    fn interpret(&mut self, input: &str) {
        let mut lexer = Lexer::from(input);
        loop {
            let token = lexer.next_token();
            match token {
                TokenType::EOF => {
                    self.tokens.push(token);
                    break;
                }
                _ => self.tokens.push(token),
            }
        }
        println!("{:?}", self.tokens);
    }
}

use repl::Repl;
pub mod lexer;
pub mod repl;
fn main() {
    let mut repl = Repl::new("ziglord");
    repl.start();
}

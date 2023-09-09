use std::io::{self, Write};
pub fn repl() -> Option<String> {
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

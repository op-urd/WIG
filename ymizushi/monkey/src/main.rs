mod monkey;
use monkey::lang::{Lexer};
use std::io::{self, Write};

fn main() {
    print!(">> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let lexer = Lexer::new(input);
            let token = lexer.next_token();
            println!("{}", token);
            io::stdout().flush().unwrap();
        }
        Err(error) => println!("error: {}", error)
    }
}

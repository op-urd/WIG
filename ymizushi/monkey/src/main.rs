mod monkey;
use monkey::lang::{Lexer, TokenType};
use std::io::{self, Write};

fn main() {
    print!(">> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let lexer = Lexer::new(input);
            loop {
                let t = lexer.next_token();
                if t.token_type == TokenType::Eof {
                    break;
                }
                println!("{}", t);
            }
            io::stdout().flush().unwrap();
        }
        Err(error) => println!("error: {}", error)
    }
}

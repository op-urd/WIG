mod monkey;
use monkey::lang::{Lexer, Token, TokenType};
use std::io::{self, Write};

fn main() {
    let input_str = String::from("let five = 5;
let ten = 10;
let add = fn(x, y) {
    x + y;
}

let result = add(five, ten);
!-/*5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;");
    let lexer = Lexer::new(input_str);

    print!(">> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            print!("{}", input);
            io::stdout().flush().unwrap();
        }
        Err(error) => println!("error: {}", error)
    }
}

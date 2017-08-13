use std::char;
use std::cell::Cell;
use std::fmt;

pub struct Lexer {
    input: String,
    position: Cell<usize>,
    read_position: Cell<usize>,
    ch: Cell<char>
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let l = Lexer {
            input: input,
            position: Cell::new(0),
            read_position: Cell::new(0),
            ch: Cell::new(' ')
        };
        l.read_char();
        l
    }

    pub fn read_char(&self) {
        if self.read_position.get() >= self.input.len() {
            self.ch.set('\0');
        } else {
            let c = self.input.chars().nth(self.read_position.get()).unwrap();
            self.ch.set(c);
        }
        self.position.set(self.read_position.get());
        self.read_position.set(self.read_position.get() + 1);
    }

    pub fn next_token(&self) -> Token {
        self.skip_white_space();
        self.get_token(self.ch.get())
    }

    pub fn get_token(&self, s: char) -> Token {
        let token = match s {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token {
                        token_type: TokenType::Eq,
                        value: Some(Value::Str(String::from("==")))
                    }
                } else {
                    Token {
                        token_type: TokenType::Assign,
                        value: Some(Value::Str(String::from("=")))
                    }
                }
            }
            '+' => {
                Token {
                    token_type: TokenType::Plus,
                    value: Some(Value::Str(String::from("+")))
                }
            }
            '-' => {
                Token {
                    token_type: TokenType::Minus,
                    value: Some(Value::Str(String::from("-")))
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token {
                        token_type: TokenType::NotEq,
                        value: Some(Value::Str(String::from("!=")))
                    }
                } else {
                    Token {
                        token_type: TokenType::Bang,
                        value: Some(Value::Str(String::from("!")))
                    }
                }
            }
            '/' => {
                Token {
                    token_type: TokenType::Slash,
                    value: Some(Value::Str(String::from("/")))
                }
            }
            '*' => {
                Token {
                    token_type: TokenType::Asterisk,
                    value: Some(Value::Str(String::from("*")))
                }
            }
            '<' => {
                Token {
                    token_type: TokenType::Lt,
                    value: Some(Value::Str(String::from("<")))
                }
            }
            '>' => {
                Token {
                    token_type: TokenType::Gt,
                    value: Some(Value::Str(String::from(">")))
                }
            }
            ';' => {
                Token {
                    token_type: TokenType::Semicolon,
                    value: Some(Value::Str(String::from(";")))
                }
            }
            '(' => {
                Token {
                    token_type: TokenType::Lparen,
                    value: Some(Value::Str(String::from("(")))
                }
            }
            ')' => {
                Token {
                    token_type: TokenType::Rparen,
                    value: Some(Value::Str(String::from(")")))
                }
            }
            ',' => {
                Token {
                    token_type: TokenType::Comma,
                    value: Some(Value::Str(String::from(",")))
                }
            }
            '{' => {
                Token {
                    token_type: TokenType::Lbrace,
                    value: Some(Value::Str(String::from("{")))
                }
            }
            '}' => {
                Token {
                    token_type: TokenType::Rbrace,
                    value: Some(Value::Str(String::from("}")))
                }
            }
            '\0' => {
                Token {
                    token_type: TokenType::Eof,
                    value: Some(Value::Str(String::from("\0")))
                }
            }
            c => {
                if is_letter(c) {
                    let identifier = self.read_identifier();
                    return Token {
                        token_type: self.lookup_ident(&identifier),
                        value: Some(Value::Str(identifier))
                    }
                } else if is_digit(c) {
                    return Token {
                        token_type: TokenType::Int,
                        value: Some(Value::Str(self.read_number()))
                    }
                } else {
                    Token {
                        token_type: TokenType::Illegal,
                        value: None
                    }
                }
            }
        };
        self.read_char();
        token
    }

    pub fn read_identifier(&self) -> String {
        let start_position = self.position.get();
        while is_letter(self.ch.get()) {
            self.read_char();
        }
        let end_position = self.position.get();
        let s = &(self.input)[start_position..end_position];
        s.to_string()
    }

    pub fn lookup_ident(&self, s: &str) -> TokenType {
        match s {
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "return" => TokenType::Return,
            _ => TokenType::Ident
        }
    }

    pub fn read_number(&self) -> String {
        let start_position = self.position.get();
        while is_digit(self.ch.get()) {
            self.read_char();
        }
        let end_position = self.position.get();
        let s = &(self.input)[start_position..end_position];
        s.to_string()
    }

    pub fn skip_white_space(&self) {
        while self.ch.get() == ' ' || self.ch.get() == '\t' || self.ch.get() == '\n' || self.ch.get() == '\r' {
            self.read_char();
        }
    }

    pub fn peek_char(&self) -> char {
        if self.read_position.get() >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position.get()).unwrap()
        }
    }
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}



#[derive(Debug)]
pub enum Value {
    Str(String),
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match *self {
            Value::Str(ref s) => {
                match *other {
                    Value::Str(ref o) => {
                        *s == *o
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Illegal,
    Eof,

    Ident,
    Int,

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Eq,
    NotEq,

    Lt,
    Gt,

    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return
}



#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<Value>,
}

impl fmt::Display for Token  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", 1.0)
    }
}

#[test]
fn peek_char() {
    let input = String::from("!=");
    let l = &Lexer::new(input);
    assert_eq!('=', l.peek_char());
    assert_eq!('=', l.peek_char());
}

#[test]
fn test_is_digit() {
    assert_eq!(is_digit('0'), true);
    assert_eq!(is_digit('1'), true);
    assert_eq!(is_digit('8'), true);
    assert_eq!(is_digit('9'), true);
}

#[test]
fn test_is_letter() {
    assert_eq!(is_letter('_'), true);
    assert_eq!(is_letter('a'), true);
    assert_eq!(is_letter('b'), true);
    assert_eq!(is_letter('y'), true);
    assert_eq!(is_letter('z'), true);
    assert_eq!(is_letter('A'), true);
    assert_eq!(is_letter('B'), true);
    assert_eq!(is_letter('Y'), true);
    assert_eq!(is_letter('Z'), true);
}


#[test]
fn next_token() {
    let input = String::from("let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;");
    let tests = vec![
        (TokenType::Let, Value::Str(String::from("let"))),
        (TokenType::Ident, Value::Str(String::from("five"))),
        (TokenType::Assign, Value::Str(String::from("="))),
        (TokenType::Int, Value::Str(String::from("5"))),
        (TokenType::Semicolon, Value::Str(String::from(";"))),
        (TokenType::Let, Value::Str(String::from("let"))),
        (TokenType::Ident, Value::Str(String::from("ten"))),
        (TokenType::Assign, Value::Str(String::from("="))),
        (TokenType::Int, Value::Str(String::from("10"))),
        (TokenType::Semicolon, Value::Str(String::from(";"))),
        (TokenType::Let, Value::Str(String::from("let"))),
        (TokenType::Ident, Value::Str(String::from("add"))),
        (TokenType::Assign, Value::Str(String::from("="))),
        (TokenType::Function, Value::Str(String::from("fn"))),
        (TokenType::Lparen, Value::Str(String::from("("))),
        (TokenType::Ident, Value::Str(String::from("x"))),
        (TokenType::Comma, Value::Str(String::from(","))),
        (TokenType::Ident, Value::Str(String::from("y"))),
        (TokenType::Rparen, Value::Str(String::from(")"))),
        (TokenType::Lbrace, Value::Str(String::from("{"))),
        (TokenType::Ident, Value::Str(String::from("x"))),
        (TokenType::Plus, Value::Str(String::from("+"))),
        (TokenType::Ident, Value::Str(String::from("y"))),
        (TokenType::Semicolon, Value::Str(String::from(";"))),
        (TokenType::Rbrace, Value::Str(String::from("}"))),
        (TokenType::Semicolon, Value::Str(String::from(";"))),
        (TokenType::Let, Value::Str(String::from("let"))),
        (TokenType::Ident, Value::Str(String::from("result"))),
        (TokenType::Assign, Value::Str(String::from("="))),
        (TokenType::Ident, Value::Str(String::from("add"))),
        (TokenType::Lparen, Value::Str(String::from("("))),
        (TokenType::Ident, Value::Str(String::from("five"))),
        (TokenType::Comma, Value::Str(String::from(","))),
        (TokenType::Ident, Value::Str(String::from("ten"))),
        (TokenType::Rparen, Value::Str(String::from(")"))),
        (TokenType::Semicolon, Value::Str(String::from(";"))),
        (TokenType::Bang, Value::Str(String::from("!"))),
        (TokenType::Minus, Value::Str(String::from("-"))),
        (TokenType::Slash, Value::Str(String::from("/"))),
        (TokenType::Asterisk, Value::Str(String::from("*"))),
        (TokenType::Int, Value::Str(String::from("5"))),
        (TokenType::Semicolon, Value::Str(String::from(";"))),
        (TokenType::Int, Value::Str(String::from("5"))),
        (TokenType::Lt, Value::Str(String::from("<"))),
        (TokenType::Int, Value::Str(String::from("10"))),
        (TokenType::Gt, Value::Str(String::from(">"))),
        (TokenType::Int, Value::Str(String::from("5"))),
        (TokenType::Semicolon, Value::Str(String::from(";"))),
        (TokenType::If, Value::Str(String::from("if"))),
        (TokenType::Lparen, Value::Str(String::from("("))),
        (TokenType::Int, Value::Str(String::from("5"))),
        (TokenType::Lt, Value::Str(String::from("<"))),
        (TokenType::Int, Value::Str(String::from("10"))),
        (TokenType::Rparen, Value::Str(String::from(")"))),
        (TokenType::Lbrace, Value::Str(String::from("{"))),
        (TokenType::Return, Value::Str(String::from("return"))),
        (TokenType::True, Value::Str(String::from("true"))),
        (TokenType::Semicolon, Value::Str(String::from(";"))),
        (TokenType::Rbrace, Value::Str(String::from("}"))),
        (TokenType::Else, Value::Str(String::from("else"))),
        (TokenType::Lbrace, Value::Str(String::from("{"))),
        (TokenType::Return, Value::Str(String::from("return"))),
        (TokenType::False, Value::Str(String::from("false"))),
        (TokenType::Semicolon, Value::Str(String::from(";"))),
        (TokenType::Rbrace, Value::Str(String::from("}"))),
        (TokenType::Int, Value::Str(String::from("10"))),
        (TokenType::Eq, Value::Str(String::from("=="))),
        (TokenType::Int, Value::Str(String::from("10"))),
        (TokenType::Semicolon, Value::Str(String::from(";"))),
        (TokenType::Int, Value::Str(String::from("10"))),
        (TokenType::NotEq, Value::Str(String::from("!="))),
        (TokenType::Int, Value::Str(String::from("9"))),
        (TokenType::Semicolon, Value::Str(String::from(";"))),
        (TokenType::Eof, Value::Str(String::from("\0"))),
    ];

    let l = &Lexer::new(input);
    for e in &tests {
        let t = l.next_token();
        assert_eq!(t.token_type, e.0);
        assert_eq!(t.value.unwrap(), e.1);
    }
}

use std::char;
use std::cell::Cell;


#[derive(Debug)]
pub enum Value {
    Int(String),
    Str(String),
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    MINUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
}

impl TokenType {
    pub fn from_string(s: String) -> TokenType {
        match s.trim().as_ref() {
            "\n" => TokenType::EOF,
            "=" => TokenType::ASSIGN,
            "(" => TokenType::LPAREN,
            ")" => TokenType::RPAREN,
            "let" => TokenType::LET,
            "+" => TokenType::PLUS,
            "-" => TokenType::MINUS,
            "," => TokenType::COMMA,
            ";" => TokenType::SEMICOLON,
            "{" => TokenType::LBRACE,
            "}" => TokenType::RBRACE,
            "func" => TokenType::FUNCTION,
            c => TokenType::ILLEGAL
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<Value>,
}

pub struct Lexer {
    input: String,
    position: Cell<usize>,
    read_position: Cell<usize>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        return Lexer {
            input: input,
            position: Cell::new(0),
            read_position: Cell::new(0),
        };
    }

    pub fn read_char(&self) -> char {
        if self.read_position.get() >= self.input.len() {
            return '0';
        } else {
            let c = self.input.chars().nth(self.read_position.get()).unwrap();
            self.position.set(self.read_position.get());
            self.read_position.set(self.read_position.get() + 1);
            return c;
        }
    }

    pub fn next_token(&self) -> Token {
        let c = self.read_char();
        let token_type = TokenType::from_string(c.to_string());
        return Token {
            token_type: token_type,
            value: Some(Value::Str(c.to_string()))
        };
    }

    pub fn peek_char(&self) -> char {
        if self.read_position.get() >= self.input.len() {
            return '0';
        } else {
            return self.input.chars().nth(self.read_position.get()).unwrap();
        }
    }
}


#[test]
fn read_char() {
    let input = String::from("=!");
    let l = &Lexer::new(input);
    assert_eq!('=', l.read_char());
    assert_eq!('!', l.read_char());
    assert_eq!('0', l.read_char());
}


#[test]
fn peek_char() {
    let input = String::from("!=");
    let l = &Lexer::new(input);
    assert_eq!('!', l.peek_char());
    assert_eq!('!', l.peek_char());
}

#[test]
fn next_token() {
    let input = String::from("=");
    let l = &Lexer::new(input);
    assert_eq!(l.next_token().token_type, TokenType::ASSIGN);
}

#[test]
fn from_string() {
    assert_eq!(TokenType::from_string(String::from("=")), TokenType::ASSIGN);
    assert_eq!(TokenType::from_string(String::from("(")), TokenType::LPAREN);
}

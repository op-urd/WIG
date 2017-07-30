use std::char;

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

pub enum Value {
    Int(String),
    Str(String),
}

pub struct Token {
    pub token_type: TokenType,
    pub value: Option<Value>,
}

impl TokenType {
    pub fn from_string (s: String) -> TokenType {
        match s.as_ref() {
            "\n" => TokenType::EOF,
            "let" => TokenType::LET,
            "+" => TokenType::PLUS,
            "-" => TokenType::MINUS,
            "," => TokenType::COMMA,
            ";" => TokenType::SEMICOLON,
            "(" => TokenType::LPAREN,
            ")" => TokenType::RPAREN,
            "{" => TokenType::LBRACE,
            "}" => TokenType::RBRACE,
            "func" => TokenType::FUNCTION,
            _ => TokenType::ILLEGAL
        }
    }
}

pub struct Parser<'a> {
    pub tokens: &'a [Token]
}

impl<'a> Parser<'a> {
    pub fn parse(&self) {
        return ();
    }
}

pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub read_position: usize,
    pub ch: char
}

impl Lexer {
    pub fn peek_char(&self) -> char {
        if self.read_position >=  self.input.len() {
            return '0';
        } else {
            return self.input.chars().nth(self.read_position).unwrap() ;
        }
    }
}

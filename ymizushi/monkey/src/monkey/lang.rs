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
    pub fn from_string(s: String) -> TokenType {
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

pub struct Lexer<'a> {
    pub input: String,
    pub position: usize,
    pub read_position: usize,
    pub ch: &'a mut char
}

impl<'a> Lexer<'a> {
    pub fn new(input: String, c: &'a mut char) -> Lexer<'a> {
        return Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: c
        };
    }

    //    pub fn read_char(&'a mut self)  {
    //        if self.read_position >= self.input.len() {
    //            self.ch = & '0';
    //        } else {
    //            self.ch = & self.input.chars().nth(self.read_position).unwrap();
    //            self.position = self.read_position;
    //            self.read_position += 1;
    //        }
    //    }

    pub fn next_token(&'a mut self) -> Token {
        let token_type = TokenType::from_string(self.ch.to_string());
        //        self.read_char();
        return Token {
            token_type: token_type,
            value: Some(Value::Str(self.ch.to_string()))
        };
    }

    pub fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return '0';
        } else {
            return self.input.chars().nth(self.read_position).unwrap();
        }
    }
}


#[test]
fn next_token() {
    let input = String::from("=+(){},;");
    let tests = [
        (TokenType::ASSIGN, "="),
        (TokenType::PLUS, "+"),
        (TokenType::LPAREN, "("),
        (TokenType::RPAREN, ")"),
        (TokenType::LBRACE, "{"),
        (TokenType::RBRACE, "}"),
        (TokenType::COMMA, ","),
        (TokenType::SEMICOLON, ";"),
        (TokenType::EOF, ""),
    ];
}

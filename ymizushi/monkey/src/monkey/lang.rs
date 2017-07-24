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

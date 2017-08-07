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
    EQ,
    BANG,
    NOT_EQ,
    PLUS,
    MINUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    SLASH,
    ASTERISK,
    LT,
    GT,
    FUNCTION,
    LET,
}

pub struct Token {
    pub token_type: TokenType,
    pub value: Option<Value>,
}

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
        return l;
    }

    pub fn from_string(&self, s: String) -> TokenType {
        let result = match s.trim().as_ref() {
            "\n" => TokenType::EOF,
            "=" => {
                if self.peek_char() == '=' {
                    self.read_char();
                    TokenType::EQ
                } else {
                    TokenType::ASSIGN
                }
            }
            "(" => TokenType::LPAREN,
            ")" => TokenType::RPAREN,
            "let" => TokenType::LET,
            "+" => TokenType::PLUS,
            "-" => TokenType::MINUS,
            "," => TokenType::COMMA,
            ";" => TokenType::SEMICOLON,
            "{" => TokenType::LBRACE,
            "}" => TokenType::RBRACE,
            "/" => TokenType::SLASH,
            "," => TokenType::COMMA,
            "0" => {
                TokenType::EOF
            },
            "!" => {
                if self.peek_char() == '=' {
                    let ch = self.ch.get(); // chに退避させて次の文字を読む
                    self.read_char();
                    TokenType::NOT_EQ
                } else {
                    TokenType::BANG
                }
            }
            "func" => {
                TokenType::FUNCTION
            },
            c => {
                TokenType::ILLEGAL
            }
        };
        self.read_char();
        return result;
    }

    pub fn read_char(&self) {
        if self.read_position.get() >= self.input.len() {
            self.ch.set('0');
        } else {
            let c = self.input.chars().nth(self.read_position.get()).unwrap();
            self.ch.set(c);
        }
        self.position.set(self.read_position.get());
        self.read_position.set(self.read_position.get() + 1);
    }

    pub fn next_token(&self) -> Token {
        self.skip_white_space();
        let token_type = self.from_string(self.ch.get().to_string());
        return Token {
            token_type: token_type,
            value: Some(Value::Str(self.ch.get().to_string()))
        };
    }

    pub fn peek_char(&self) -> char {
        if self.read_position.get() >= self.input.len() {
            return '0';
        } else {
            return self.input.chars().nth(self.read_position.get()).unwrap();
        }
    }

    pub unsafe fn read_identifier(&self) -> String {
        let start_position = self.position.get();
        let ch = self.ch.get();
        while is_letter(ch) {
            self.read_char();
        }
        let end_position = self.position.get();
        let s = self.input.slice_unchecked(start_position, end_position);
        return s.to_string();
    }

    pub unsafe fn read_number(&self) -> String {
        let start_position = self.position.get();
        let ch = self.ch.get();
        while is_digit(ch) {
            self.read_char();
        }
        let end_position = self.position.get();
        let s = self.input.slice_unchecked(start_position, end_position);
        return s.to_string();
    }

    pub fn skip_white_space(&self) {
        let ch = self.ch.get();
        while ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
            self.read_char();
        }
    }
}

fn is_letter(ch: char) -> bool {
    return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_';
}

fn is_digit(ch: char) -> bool {
    return '0' <= ch && ch <= '9';
}




#[test]
fn peek_char() {
    let input = String::from("!=");
    let l = &Lexer::new(input);
    assert_eq!('=', l.peek_char());
    assert_eq!('=', l.peek_char());
}

#[test]
fn next_token() {
    let input = String::from("=()");
    let l = &Lexer::new(input);
    assert_eq!(l.next_token().token_type, TokenType::ASSIGN);
    assert_eq!(l.next_token().token_type, TokenType::LPAREN);
    assert_eq!(l.next_token().token_type, TokenType::RPAREN);
}

#[test]
fn from_string() {
    let input = String::from("=()");
    let l = &Lexer::new(input);
    assert_eq!(l.from_string(String::from("=")), TokenType::ASSIGN);
    assert_eq!(l.from_string(String::from("(")), TokenType::LPAREN);
}
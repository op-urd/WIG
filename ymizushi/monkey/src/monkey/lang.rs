use std::char;
use std::cell::Cell;
use std::fmt;
use std::collections::HashMap;


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
        let ch = self.ch.get().to_string();
        self.get_token(ch)
    }

    pub fn get_token(&self, s: String) -> Token {
        let token = match s.trim().as_ref() {
            "=" => {
                if self.peek_char() == '=' {
                    let ch = self.ch.get();
                    self.read_char();
                    Token {
                        token_type: TokenType::EQ,
                        value: Some(Value::Str(String::from("=="))) // TODO: 文字列を連結するように修正する
                    }
                } else {
                    Token {
                        token_type: TokenType::ASSIGN,
                        value: Some(Value::Str(String::from("=")))
                    }
                }
            }
            "+" => {
                Token {
                    token_type: TokenType::PLUS,
                    value: Some(Value::Str(String::from("+")))
                }
            }
            "-" => {
                Token {
                    token_type: TokenType::MINUS,
                    value: Some(Value::Str(String::from("-")))
                }
            }
            "!" => {
                if self.peek_char() == '=' {
//                    let ch = self.ch.get(); // chに退避させて次の文字を読む
                    self.read_char();
                    Token {
                        token_type: TokenType::NotEq,
                        value: Some(Value::Str(String::from("!=")))
                    }
                } else {
                    Token {
                        token_type: TokenType::BANG,
                        value: Some(Value::Str(String::from("!")))
                    }
                }
            }
            "/" => {
                Token {
                    token_type: TokenType::SLASH,
                    value: Some(Value::Str(String::from("/")))
                }
            }
            "*" => {
                Token {
                    token_type: TokenType::ASTERISK,
                    value: Some(Value::Str(String::from("*")))
                }
            }
            "<" => {
                Token {
                    token_type: TokenType::LT,
                    value: Some(Value::Str(String::from("<")))
                }
            }
            ">" => {
                Token {
                    token_type: TokenType::GT,
                    value: Some(Value::Str(String::from(">")))
                }
            }
            ";" => {
                Token {
                    token_type: TokenType::SEMICOLON,
                    value: Some(Value::Str(String::from(";")))
                }
            }
            "(" => {
                Token {
                    token_type: TokenType::LPAREN,
                    value: Some(Value::Str(String::from("(")))
                }
            }
            ")" => {
                Token {
                    token_type: TokenType::RPAREN,
                    value: Some(Value::Str(String::from(")")))
                }
            }
            "," => {
                Token {
                    token_type: TokenType::COMMA,
                    value: Some(Value::Str(String::from(",")))
                }
            }
            "{" => {
                Token {
                    token_type: TokenType::LBRACE,
                    value: Some(Value::Str(String::from("{")))
                }
            }
            "}" => {
                Token {
                    token_type: TokenType::RBRACE,
                    value: Some(Value::Str(String::from("}")))
                }
            }
            "\0" => {
                Token {
                    token_type: TokenType::EOF,
                    value: Some(Value::Str(String::from("\0")))
                }
            }
            c => {
                if is_letter(c.chars().nth(0).unwrap()) {
                    Token {
                        token_type: TokenType::ILLEGAL, // TODO: lookup_identを実装する
                        value: Some(Value::Str(unsafe {self.read_identifier()}))
                    }
                } else if is_digit(c.chars().nth(0).unwrap()) {
                    Token {
                        token_type: TokenType::INT,
                        value: Some(Value::Str(unsafe { self.read_number()}))
                    }
                } else {
                    Token {
                        token_type: TokenType::ILLEGAL,
                        value: None
                    }
                }
            }
        };
        self.read_char();
        return token;
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

    pub fn peek_char(&self) -> char {
        if self.read_position.get() >= self.input.len() {
            return '0';
        } else {
            return self.input.chars().nth(self.read_position.get()).unwrap();
        }
    }
}

fn is_letter(ch: char) -> bool {
    return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_';
}

fn is_digit(ch: char) -> bool {
    return '0' <= ch && ch <= '9';
}



#[derive(Debug)]
pub enum Value {
    Int(String),
    Str(String),
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    Ident, // TODO: 使う
    INT,

    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    EQ,
    NotEq,

    LT,
    GT,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return
}

//fn lookup_ident(ident: String) -> TokenType {
//    let mut book_reviews = HashMap::new();
//    book_reviews.insert("Adventures of Huckleberry Finn","");
//}

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



// Test
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


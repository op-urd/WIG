use monkey::ast::{Program};
use std::cell::Cell;
use monkey::lang::{Lexer, Token, TokenType};


struct Parser {
    l: Lexer,
    cur_token: Cell<Token>,
    peek_token: Cell<Token>
}


impl Parser {
    pub fn new(l: Lexer) -> Parser {
        // TODO: null許容する
        let p = Parser {
            l:l,
            cur_token: Cell::new(Token{token_type: TokenType::Semicolon, value: None}),
            peek_token: Cell::new(Token{token_type: TokenType::Semicolon, value: None})
        };
        p.next_token();
        p.next_token();
        p
    }

    pub fn next_token(&self) {
        self.cur_token.set(self.peek_token.get());
        self.peek_token.set(self.cur_token.get());
    }

    pub fn parse_program(&self) -> Program {
        // TODO: 実装する
        return None
    }
}



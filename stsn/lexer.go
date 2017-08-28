package main

type Lexer struct {
	input string

	position     int // current position in input
	readPosition int
	ch           byte
}

func (l *Lexer) NextToken() Token {
	return newToken(ASSIGN)
}

func newToken(tokenType TokenType) Token {
	return Token{Type: tokenType}
}

func NewLexer(input string) *Lexer {
	return &Lexer{input: input}
}

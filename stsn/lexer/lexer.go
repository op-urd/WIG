package lexer

import "github.com/satoshun/wig/token"

type Lexer struct {
	input string

	position     int // current position in input
	readPosition int
	ch           byte
}

func (l *Lexer) NextToken() token.Token {
	return newToken(token.ASSIGN)
}

func newToken(tokenType token.TokenType) token.Token {
	return token.Token{Type: tokenType}
}

func New(input string) *Lexer {
	return &Lexer{input: input}
}

package main

import (
	"fmt"
	"testing"
)

func TestNextToken(t *testing.T) {
	input := `=+(),;`
	tests := []struct {
		expectedType    TokenType
		expectedLiteral string
	}{
		{ASSIGN, "="},
	}
	l := NewLexer(input)
	for i, _ := range tests {
		t := l.NextToken()
		fmt.Printf("%d %s", i, t)
	}

}

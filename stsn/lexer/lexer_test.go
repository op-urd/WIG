package lexer

import (
	"fmt"
	"testing"

	"github.com/satoshun/wig/token"
)

func TestNextToken(t *testing.T) {
	input := `=+(),;`
	tests := []struct {
		expectedType    token.TokenType
		expectedLiteral string
	}{
		{token.ASSIGN, "="},
	}
	l := New(input)
	for i, _ := range tests {
		t := l.NextToken()
		fmt.Printf("%d %s", i, t)
	}

}

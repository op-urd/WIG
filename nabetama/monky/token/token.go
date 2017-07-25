package token

type TokenType string

type Token struct {
	Type    TokenType
	Literal string
}

// TokenTypes
const (
	ILLEGAL = "ILLEGAL" // illegal characters
	EOF     = "EOF"     // end of the file

	IDENT   = "IDENT"   // add, foobar, x
	INT     = "INT"     // 1,2,3...

	// Operators
	ASSIGN = "="
	PLUS   = "+"

	// Delimiters
	COMMA     = ","
	SEMICOLON = ";"

	LPAREN    = "("
	RPAREN    = ")"
	LBRACE    = "{"
	RBRACE    = "}"

	// Functions
	FUNCTION = "FUNCTION"
	LET      = "LET"
)


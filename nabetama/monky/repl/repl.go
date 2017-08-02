package repl

import (
	"io"
	"bufio"
	"fmt"
	"github.com/op-urd/WIG/nabetama/monky/lexar"
	"github.com/op-urd/WIG/nabetama/monky/token"

)

const (
	PROMPT = ">> "
)

func Start(in io.Reader, out io.Writer) {
	scanner := bufio.NewScanner(in)

	for {
		fmt.Printf(PROMPT)
		scanned := scanner.Scan()
		if !scanned {
			return
		}

		line := scanner.Text()
		l := lexar.New(line)

		for tok := l.NextToken(); tok.Type != token.EOF; tok = l.NextToken(){
			fmt.Printf("%+v\n", tok)
		}
	}
}

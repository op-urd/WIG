package main

import (
	"os/user"
	"os"
	"fmt"
	"github.com/op-urd/WIG/nabetama/monky/repl"
)

func main() {
	user, err := user.Current()
	if err != nil {
		os.Exit(1)
	}

	fmt.Printf("Hello %s! This is the Monkey programming language!\n",
		user.Username)
	fmt.Printf("Feel free to type in commands\n")
	repl.Start(os.Stdin, os.Stdout)
}

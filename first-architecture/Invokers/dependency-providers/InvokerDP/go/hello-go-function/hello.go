package main

import (
	"fmt"
	"os"
)

func main() {
	// Check if an argument is provided
	if len(os.Args) < 2 {
		fmt.Println("Usage: go run main.go <your_argument>")
		return
	}
	// Read the first argument (os.Args[1])
	arg := os.Args[1]
	hello(arg)
}

func hello(arg string) {
	os.Stdout.WriteString("hello " + arg)
}

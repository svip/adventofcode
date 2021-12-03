package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

type command struct {
	command string
	amount  int
}

func main() {
	body, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	reader := bufio.NewReader(bytes.NewReader(body))

	var commands []command
	for {
		text, _ := reader.ReadString('\n')
		text = strings.Trim(text, " \n")
		if text == "" {
			break
		}
		s := strings.Split(text, " ")
		n, _ := strconv.Atoi(s[1])
		commands = append(commands, command{
			command: s[0],
			amount:  n,
		})
	}

	pos := 0
	depth := 0
	aim := 0

	for _, a := range commands {
		switch a.command {
		case "forward":
			pos += a.amount
			depth += aim * a.amount
		case "down":
			aim += a.amount
		case "up":
			aim -= a.amount
		}
	}
	fmt.Println(pos, depth, pos*depth)
}

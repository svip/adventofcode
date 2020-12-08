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

func main() {
	body, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	reader := bufio.NewReader(bytes.NewReader(body))

	type instruction struct {
		op    string
		value int
	}

	var program []instruction
	for {
		text, err := reader.ReadString('\n')
		if err != nil {
			break
		}
		text = strings.Trim(text, " \n")

		s := strings.Split(text, " ")
		op := s[0]
		v, _ := strconv.Atoi(s[1])

		program = append(program, instruction{op: op, value: v})
	}

	accumulator := 0
	p := 0

	instructionRunMap := make(map[int]int)

	for {
		i := program[p]
		instructionRunMap[p] = instructionRunMap[p] + 1
		if instructionRunMap[p] >= 2 {
			break
		}
		switch i.op {
		case "acc":
			accumulator += i.value
		case "jmp":
			p += i.value - 1
		case "nop":
			break
		}
		p++
		if p >= len(program) {
			break
		}
	}

	fmt.Println(accumulator)
}

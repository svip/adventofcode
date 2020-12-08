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

type instruction struct {
	op    string
	value int
}

func runProgram(program []instruction) (accumulator int, success bool) {
	accumulator = 0
	p := 0

	instructionRunMap := make(map[int]int)

	for {
		i := program[p]
		instructionRunMap[p] = instructionRunMap[p] + 1
		if instructionRunMap[p] >= 2 {
			return accumulator, false
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
		if p < 0 {
			return accumulator, false
		}
		if p >= len(program) {
			break
		}
	}
	return accumulator, true
}

func main() {
	body, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	reader := bufio.NewReader(bytes.NewReader(body))

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

	for i, ins := range program {
		if ins.op == "jmp" || ins.op == "nop" {
			org := ins.op
			if ins.op == "jmp" {
				program[i].op = "nop"
			} else {
				program[i].op = "jmp"
			}
			if accumulator, success := runProgram(program); success {
				fmt.Println(accumulator)
				break
			}
			program[i].op = org // set it back
		}
	}
}

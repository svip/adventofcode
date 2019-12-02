package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	var positions []int
	for scanner.Scan() {
		numbers := strings.Split(scanner.Text(), ",")
		positions = make([]int, len(numbers))
		for i, s := range numbers {
			n, err := strconv.Atoi(s)
			if err != nil {
				panic(err)
			}
			positions[i] = n
		}
	}
	// start up
	positions[1] = 12
	positions[2] = 2
	p := 0 // position
main:
	for {
		opcode := positions[p]
		switch opcode {
		case 1,2:
			a_pos, b_pos, c_pos := positions[p+1], positions[p+2], positions[p+3]
			a, b := positions[a_pos], positions[b_pos]
			var c int
			if opcode == 1 { // addition
				c = a+b
			} else if opcode == 2 { // multiplication
				c = a*b
			}
			positions[c_pos] = c
			p += 4
		case 99:
			break main
		default:
			panic(fmt.Sprintf("Unknown opcode: %d", opcode))
		}
	}
	fmt.Println(positions[0])
}

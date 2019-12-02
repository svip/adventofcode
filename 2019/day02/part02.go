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
	var noun, verb int
main:
	for i := 0; i <= 99; i++ {
		for j := 0; j <= 99; j++ {
			p := 0 // instruction pointer
			memory := make([]int, len(positions))
			copy(memory, positions)
			memory[1] = i
			memory[2] = j
		program:
			for {
				opcode := memory[p]
				switch opcode {
				case 1, 2:
					a_pos, b_pos, c_pos := memory[p+1], memory[p+2], memory[p+3]
					a, b := memory[a_pos], memory[b_pos]
					var c int
					if opcode == 1 { // addition
						c = a + b
					} else if opcode == 2 { // multiplication
						c = a * b
					}
					memory[c_pos] = c
					p += 4
				case 99:
					break program
				default:
					break program // wasn't that
				}
			}
			if memory[0] == 19690720 {
				noun, verb = memory[1], memory[2]
				break main
			}
		}
	}
	fmt.Println(100*noun + verb)
}

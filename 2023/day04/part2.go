package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func getInput() []string {
	var input []string
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		input = append(input, scanner.Text())
	}
	if err := scanner.Err(); err != nil {
		panic(err)
	}
	return input
}

func main() {
	input := getInput()

	copies := make(map[int]int)
	for no, line := range input {
		copies[no] += 1
		ss := strings.Split(line, ": ")
		card := ss[1]
		sets := strings.Split(card, " | ")
		var winning []int
		for _, s := range strings.Split(sets[0], " ") {
			n, _ := strconv.Atoi(s)
			if n > 0 {
				winning = append(winning, n)
			}
		}
		var has []int
		for _, s := range strings.Split(sets[1], " ") {
			n, _ := strconv.Atoi(s)
			if n > 0 {
				has = append(has, n)
			}
		}
		for j := 0; j < copies[no]; j++ {
			i := 1
			for _, n := range has {
				for _, w := range winning {
					if n == w {
						copies[no+i] += 1
						i++
						break
					}
				}
			}
		}
	}
	total := 0
	for i := 0; i < len(input); i++ {
		c := copies[i]
		total += c
	}
	fmt.Println(total)
}

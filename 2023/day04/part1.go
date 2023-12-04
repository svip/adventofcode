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

	total := 0
	for _, line := range input {
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
		p := 0
		for _, n := range has {
			for _, w := range winning {
				if n == w {
					if p == 0 {
						p = 1
					} else {
						p = p * 2
					}
					break
				}
			}
		}
		total += p
	}
	fmt.Println(total)
}

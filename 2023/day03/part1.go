package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
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

func neighbourIsPart(y int, x0 int, x1 int, text []string) bool {
	match := regexp.MustCompile("[^.0-9]")
	var coordsToCheck [][2]int
	for i := x0 - 1; i <= x1+1; i++ {
		for j := y - 1; j <= y+1; j++ {
			if j == y && i >= x0 && i <= x1 {
				continue
			}
			coordsToCheck = append(coordsToCheck, [2]int{i, j})
		}
	}
	for _, coords := range coordsToCheck {
		if coords[0] < 0 || coords[1] < 0 || coords[0] >= len(text[0]) || coords[1] >= len(text) {
			continue
		}
		if match.MatchString(string(text[coords[1]][coords[0]])) {
			return true
		}
	}
	return false
}

func main() {
	input := getInput()

	total := 0
	match := regexp.MustCompile("[0-9]")
	for y, line := range input {
		atNumber := false
		var currentNumber string
		currentStart := -1
		for x, c := range line {
			if match.MatchString(string(c)) {
				if atNumber {
					currentNumber += string(c)
				} else {
					atNumber = true
					currentStart = x
					currentNumber = string(c)
				}
			} else {
				if atNumber {
					atNumber = false
					if neighbourIsPart(y, currentStart, x-1, input) {
						n, _ := strconv.Atoi(currentNumber)
						total += n
					}
					currentNumber = ""
				}
			}
		}
		if atNumber {
			atNumber = false
			if neighbourIsPart(y, currentStart, len(line)-1, input) {
				n, _ := strconv.Atoi(currentNumber)
				total += n
			}
			currentNumber = ""
		}
	}
	fmt.Println(total)
}

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

func convertDigit(s string) int {
	switch s {
	case "one":
		return 1
	case "two":
		return 2
	case "three":
		return 3
	case "four":
		return 4
	case "five":
		return 5
	case "six":
		return 6
	case "seven":
		return 7
	case "eight":
		return 8
	case "nine":
		return 9
	default:
		n, _ := strconv.Atoi(s)
		return n
	}
}

func main() {
	input := getInput()

	total := 0
	r := regexp.MustCompile("^(one|two|three|four|five|six|seven|eight|nine|[0-9])")
	for _, line := range input {
		var tokens []string
		for i := 0; i < len(line); i++ {
			if r.MatchString(line[i:]) {
				tokens = append(tokens, r.FindString(line[i:]))
			}
		}
		if len(tokens) > 0 {
			d := convertDigit(tokens[0])*10 + convertDigit(tokens[len(tokens)-1])
			total += d
		}
	}
	fmt.Println(total)
}

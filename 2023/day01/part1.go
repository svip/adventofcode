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

func main() {
	input := getInput()

	total := 0
	r := regexp.MustCompile("[0-9]")
	for _, line := range input {
		f := ""
		for _, c := range line {
			if r.MatchString(string(c)) {
				f = string(c)
				break
			}
		}
		l := ""
		for i := len(line) - 1; i >= 0; i-- {
			if r.MatchString(string(line[i])) {
				l = string(line[i])
				break
			}
		}
		d := f + l
		n, _ := strconv.Atoi(d)
		total += n
	}
	fmt.Println(total)
}

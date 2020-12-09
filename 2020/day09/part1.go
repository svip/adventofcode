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

func isValid(numbers []int, n int) bool {
	for i, a := range numbers {
		for j, b := range numbers {
			if i == j {
				continue
			}
			if a+b == n {
				return true
			}
		}
	}
	return false
}

func main() {
	body, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	reader := bufio.NewReader(bytes.NewReader(body))

	var numbers []int
	for {
		text, err := reader.ReadString('\n')
		if err != nil {
			break
		}
		text = strings.Trim(text, " \n")

		n, _ := strconv.Atoi(text)

		numbers = append(numbers, n)
	}

	preamble := 25

	for i, n := range numbers {
		if i < preamble {
			continue
		}
		if !isValid(numbers[i-preamble:i], n) {
			fmt.Println(n)
			break
		}
	}
}

package main

import (
	"bufio"
	"bytes"
	"io/ioutil"
	"os"
	"strconv"

	"fmt"
	"strings"
)

func main() {
	body, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	reader := bufio.NewReader(bytes.NewReader(body))

	var numbers []int
	for {
		text, _ := reader.ReadString('\n')
		text = strings.Trim(text, " \n")
		if text == "" {
			break
		}
		n, _ := strconv.Atoi(text)
		if n != 0 {
			numbers = append(numbers, n)
		}
	}

	previous := 0
	increases := 0
	for i, a := range numbers {
		if i == 0 {
			previous = a
			continue
		}
		if a > previous {
			increases++
		}
		previous = a
	}
	fmt.Println(increases)
}

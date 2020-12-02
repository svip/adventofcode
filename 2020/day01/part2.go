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

loop:
	for i, a := range numbers {
		for j, b := range numbers {
			if i == j {
				continue
			}
			for k, c := range numbers {
				if i == k || j == k {
					continue
				}
				if a+b+c == 2020 {
					fmt.Printf("%d * %d * %d = %d\n", a, b, c, a*b*c)
					break loop
				}
			}
		}
	}
}

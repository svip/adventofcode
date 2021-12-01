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

	windows := make([]int, 4)
	increases := 0
	for i, a := range numbers {
		switch i % 4 {
		case 0:
			windows[0] = a
			windows[2] += a
			windows[3] += a
		case 1:
			windows[1] = a
			windows[3] += a
			windows[0] += a
		case 2:
			windows[2] = a
			windows[0] += a
			windows[1] += a
		case 3:
			windows[3] = a
			windows[1] += a
			windows[2] += a
		}
		if i < 3 {
			continue
		}
		diff := 0
		id := i%4 - 2
		if id < 0 {
			id += 4
		}
		if id == 0 {
			diff = windows[0] - windows[3]
		} else {
			diff = windows[id] - windows[id-1]
		}
		if diff > 0 {
			increases++
		}
	}
	fmt.Println(increases)
}

package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

func main() {
	body, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	reader := bufio.NewReader(bytes.NewReader(body))

	var field [][]bool // true = tree
	for {
		text, _ := reader.ReadString('\n')
		text = strings.Trim(text, " \n")
		if text == "" {
			break
		}
		var row []bool
		for _, r := range text {
			if r == '#' {
				row = append(row, true)
			} else {
				row = append(row, false)
			}
		}
		field = append(field, row)
	}

	x, y := 0, 0
	trees := 0

	for {
		x, y = x+3, y+1
		if y >= len(field) {
			break
		}
		if x >= len(field[y]) {
			x = x - len(field[y])
		}
		if field[y][x] {
			trees++
		}
	}

	fmt.Println(trees)
}

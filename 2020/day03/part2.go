package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

func slope(field [][]bool, a int, b int) int {
	x, y := 0, 0
	trees := 0

	for {
		x, y = x+a, y+b
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

	return trees
}

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

	a := slope(field, 1, 1)
	b := slope(field, 3, 1)
	c := slope(field, 5, 1)
	d := slope(field, 7, 1)
	e := slope(field, 1, 2)

	fmt.Println(a * b * c * d * e)
}

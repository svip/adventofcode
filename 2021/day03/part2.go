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

type position struct {
	zeros int
	ones  int
}

func main() {
	body, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	reader := bufio.NewReader(bytes.NewReader(body))

	var lines []string
	for {
		text, _ := reader.ReadString('\n')
		text = strings.Trim(text, " \n")
		if text == "" {
			break
		}
		lines = append(lines, text)
	}

	oxygenLines := lines
	co2Lines := lines
	for pos := 0; pos < len(lines[0]); pos++ {
		if len(oxygenLines) > 1 {
			position := position{0, 0}
			for _, line := range oxygenLines {
				n, _ := strconv.Atoi(string(line[pos]))
				if n == 0 {
					position.zeros++
				} else {
					position.ones++
				}
			}
			char := '0'
			if position.ones >= position.zeros {
				char = '1'
			}
			var newLines []string
			for _, line := range oxygenLines {
				if line[pos] == byte(char) {
					newLines = append(newLines, line)
				}
			}
			oxygenLines = newLines
		}
		if len(co2Lines) > 1 {
			position := position{0, 0}
			for _, line := range co2Lines {
				n, _ := strconv.Atoi(string(line[pos]))
				if n == 0 {
					position.zeros++
				} else {
					position.ones++
				}
			}
			char := '0'
			if position.ones < position.zeros {
				char = '1'
			}
			var newLines []string
			for _, line := range co2Lines {
				if line[pos] == byte(char) {
					newLines = append(newLines, line)
				}
			}
			co2Lines = newLines
		}

		if len(oxygenLines) == 1 && len(co2Lines) == 1 {
			break
		}
	}

	oxygen, _ := strconv.ParseInt(oxygenLines[0], 2, 64)
	co2, _ := strconv.ParseInt(co2Lines[0], 2, 64)

	fmt.Println(oxygen, co2, oxygen*co2)
}

package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io/ioutil"
	"math"
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

	var ors []int
	var andNots []int

	memory := make(map[int]int)

	for {
		text, err := reader.ReadString('\n')
		if err != nil {
			break
		}
		text = strings.Trim(text, " \n")

		ss := strings.Split(text, " = ")

		switch ss[0] {
		case "mask":
			ors = make([]int, 0, 36)
			andNots = make([]int, 0, 36)
			for i, r := range ss[1] {
				if r == 'X' {
					continue
				}
				v := int(math.Pow(float64(2), float64(35-i)))
				if r == '1' {
					ors = append(ors, v)
				} else {
					andNots = append(andNots, v)
				}
			}
		default:
			pos, _ := strconv.Atoi(ss[0][4 : len(ss[0])-1])
			value, _ := strconv.Atoi(ss[1])
			for _, or := range ors {
				value = value | or
			}
			for _, andNot := range andNots {
				value = value &^ andNot
			}
			memory[pos] = value
		}
	}

	sum := 0
	for _, v := range memory {
		sum += v
	}
	fmt.Println(sum)
}

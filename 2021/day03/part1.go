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

	var positions []position
	for {
		text, _ := reader.ReadString('\n')
		text = strings.Trim(text, " \n")
		if text == "" {
			break
		}
		for i, r := range text {
			if len(positions) <= i {
				positions = append(positions, position{0, 0})
			}
			n, _ := strconv.Atoi(string(r))
			if n == 0 {
				positions[i].zeros++
			} else {
				positions[i].ones++
			}
		}
	}

	var gamma, epsilon string
	for _, pos := range positions {
		if pos.zeros > pos.ones {
			gamma = gamma + "0"
			epsilon = epsilon + "1"
		} else {
			gamma = gamma + "1"
			epsilon = epsilon + "0"
		}
	}

	gammaD, _ := strconv.ParseInt(gamma, 2, 64)
	epsilonD, _ := strconv.ParseInt(epsilon, 2, 64)

	fmt.Println(gammaD, epsilonD, gammaD*epsilonD)
}

package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

type group struct {
	times   int
	answers map[rune]int
}

func newGroup() group {
	return group{
		times:   0,
		answers: make(map[rune]int),
	}
}

func (g group) count() int {
	c := 0
	for _, amount := range g.answers {
		if amount == g.times {
			c++
		}
	}
	return c
}

func main() {
	body, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	reader := bufio.NewReader(bytes.NewReader(body))

	var groups []group
	currentGroup := newGroup()
	for {
		text, err := reader.ReadString('\n')
		if err != nil {
			break
		}
		text = strings.Trim(text, " \n")

		if text == "" {
			groups = append(groups, currentGroup)
			currentGroup = newGroup()
			continue
		}

		currentGroup.times++

		for _, r := range text {
			currentGroup.answers[r] = currentGroup.answers[r] + 1
		}
	}
	groups = append(groups, currentGroup)

	sum := 0
	for _, group := range groups {
		sum += group.count()
	}

	fmt.Println(sum)
}

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

	var groups []map[rune]bool
	currentGroup := make(map[rune]bool)
	for {
		text, err := reader.ReadString('\n')
		if err != nil {
			break
		}
		text = strings.Trim(text, " \n")

		if text == "" {
			groups = append(groups, currentGroup)
			currentGroup = make(map[rune]bool)
		}

		for _, r := range text {
			currentGroup[r] = true
		}
	}
	groups = append(groups, currentGroup)

	sum := 0
	for _, group := range groups {
		sum += len(group)
	}

	fmt.Println(sum)
}

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

	type password struct {
		rule struct {
			letter string
			min    int
			max    int
		}
		password string
	}
	var passwords []password
	for {
		text, _ := reader.ReadString('\n')
		text = strings.Trim(text, " \n")
		if text == "" {
			break
		}
		s := strings.Split(text, ":")
		rule := strings.Split(s[0], " ")
		minMax := strings.Split(rule[0], "-")
		min, _ := strconv.Atoi(minMax[0])
		max, _ := strconv.Atoi(minMax[1])
		pw := password{password: s[1]}
		pw.rule.letter = rule[1]
		pw.rule.min = min
		pw.rule.max = max
		passwords = append(passwords, pw)
	}

	valids := 0
	for _, pw := range passwords {
		count := strings.Count(pw.password, pw.rule.letter)
		if count >= pw.rule.min && count <= pw.rule.max {
			valids++
		}
	}

	fmt.Println(valids)
}

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
			posA   int
			posB   int
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
		pos := strings.Split(rule[0], "-")
		posA, _ := strconv.Atoi(pos[0])
		posB, _ := strconv.Atoi(pos[1])
		pw := password{password: strings.Trim(s[1], " ")}
		pw.rule.letter = rule[1]
		pw.rule.posA = posA
		pw.rule.posB = posB
		passwords = append(passwords, pw)
	}

	valids := 0
	for _, pw := range passwords {
		a := string(pw.password[pw.rule.posA-1]) == pw.rule.letter
		b := string(pw.password[pw.rule.posB-1]) == pw.rule.letter
		if (a || b) && !(a && b) {
			valids++
		}
	}

	fmt.Println(valids)
}

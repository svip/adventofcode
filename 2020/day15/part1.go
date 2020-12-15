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

	var turns []int
	spokenLast := make(map[int]int)
	spokenPrevious := make(map[int]int)

	for {
		text, err := reader.ReadString('\n')
		if err != nil {
			break
		}
		text = strings.Trim(text, " \n")

		ss := strings.Split(text, ",")

		for i, s := range ss {
			n, _ := strconv.Atoi(s)
			turns = append(turns, n)
			if _, ok := spokenLast[n]; ok {
				spokenPrevious[n] = spokenLast[n]
			}
			spokenLast[n] = i + 1
		}
	}

	for i := len(turns); i < 2020; i++ {
		last := turns[i-1]
		this := 0
		if _, ok := spokenPrevious[last]; ok {
			this = spokenLast[last] - spokenPrevious[last]
			spokenPrevious[last] = spokenLast[last]
		}
		turns = append(turns, this)
		if _, ok := spokenLast[this]; ok {
			spokenPrevious[this] = spokenLast[this]
		}
		spokenLast[this] = i + 1
	}

	fmt.Println(turns[len(turns)-1])
}

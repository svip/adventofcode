package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io/ioutil"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	body, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	reader := bufio.NewReader(bytes.NewReader(body))

	var jolts []int
	for {
		text, err := reader.ReadString('\n')
		if err != nil {
			break
		}
		text = strings.Trim(text, " \n")

		n, _ := strconv.Atoi(text)

		jolts = append(jolts, n)
	}

	sort.Slice(jolts, func(i, j int) bool { return jolts[i] < jolts[j] })

	differences := make(map[int]int)

	at := 0

	for _, j := range jolts {
		diff := j - at
		differences[diff] = differences[diff] + 1
		at += diff
	}
	differences[3] = differences[3] + 1

	fmt.Printf("%d * %d = %d\n", differences[1], differences[3], differences[1]*differences[3])
}

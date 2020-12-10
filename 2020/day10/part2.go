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

	// for convinence, we add 0 and our device to the list
	jolts = append(jolts, 0)
	sort.Slice(jolts, func(i, j int) bool { return jolts[i] < jolts[j] })
	jolts = append(jolts, jolts[len(jolts)-1]+3)

	converters := make(map[int]int)

	converters[jolts[len(jolts)-1]] = 1

	for i := len(jolts) - 2; i >= 0; i-- {
		jolt := jolts[i]
		converters[jolt] = converters[jolt+1] + converters[jolt+2] + converters[jolt+3]
	}
	fmt.Println(converters[0])
}

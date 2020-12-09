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

type sum struct {
	sum    int
	values []int
}

func findSum(numbers []int, value int) sum {
	sums := make([]sum, len(numbers))
	for i, n := range numbers {
		sums[i] = sum{
			sum:    n,
			values: []int{n},
		}
		for j := 0; j < i; j++ {
			s := sums[j]
			s.sum += n
			s.values = append(s.values, n)
			if s.sum == value {
				return s
			}
			sums[j] = s
		}
	}
	return sum{}
}

func main() {
	body, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	reader := bufio.NewReader(bytes.NewReader(body))

	var numbers []int
	for {
		text, err := reader.ReadString('\n')
		if err != nil {
			break
		}
		text = strings.Trim(text, " \n")

		n, _ := strconv.Atoi(text)

		numbers = append(numbers, n)
	}

	s := findSum(numbers, 0)

	smallest := s.sum
	highest := 0
	for _, n := range s.values {
		if smallest > n {
			smallest = n
		}
		if highest < n {
			highest = n
		}
	}
	fmt.Println(smallest + highest)
}

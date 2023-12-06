package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"regexp"
)

func getInput() []string {
	var input []string
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		input = append(input, scanner.Text())
	}
	if err := scanner.Err(); err != nil {
		panic(err)
	}
	return input
}

func main() {
	input := getInput()
	
	type race struct {
		duration int
		record int
	}
	
	var races []race
	
	c := regexp.MustCompile(" +")
	r := regexp.MustCompile("[0-9]+")
	
	for _, line := range input {
		line = c.ReplaceAllString(line, "")
		n := r.FindAllString(line, -1)
		doTime := strings.HasPrefix(line, "Time:")
		doDistance := strings.HasPrefix(line, "Distance:")
		if doTime || doDistance {
			if len(races) < len(n) {
				races = make([]race, len(n))
			}
			for i := range n {
				nn, _ := strconv.Atoi(n[i])
				if doTime {
					races[i].duration = nn
				} else {
					races[i].record = nn				
				}
			}
		}
	}
	
	total := 0
	for _, r := range races {
		s := r.duration / 2
		options := 0
		eachRow := func(i int) bool  {
			travelled := (r.duration - i) * i
			if travelled <= r.record {
				return false
			}
			options++
			//fmt.Println(r.duration, r.record, i)
			return true 
		}
		for i := s; i >= 0; i-- {
			if !eachRow(i) {
				break
			}
		}
		for i := s+1; i <= r.duration; i++ {
			if !eachRow(i) {
				break
			}
		}
		fmt.Println(r.duration, r.record, "-->", options)
		if total == 0 {
			total = options
		} else {
			total = total * options
		}
	}
	fmt.Println(total)
}

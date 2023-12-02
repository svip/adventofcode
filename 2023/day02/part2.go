package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"strconv"
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

	total := 0
	for _, line := range input {
		ss := strings.Split(line, ": ")
		
		lowestCubes := make(map[string]int)
		lowestCubes["red"] = 0
		lowestCubes["green"] = 0
		lowestCubes["blue"] = 0
		
		sets := strings.Split(ss[1], "; ")
		for _, set := range sets {
			cubes := strings.Split(set, ", ")
			for _, cube := range cubes {
				c := strings.Split(cube, " ")
				n, _ := strconv.Atoi(c[0])
				if lowestCubes[c[1]] < n {
					lowestCubes[c[1]] = n
				}
			}
		}
		power := lowestCubes["red"] * lowestCubes["green"] * lowestCubes["blue"]
		total += power
	}
	fmt.Println(total)
}

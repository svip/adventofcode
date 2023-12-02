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
		game := strings.Split(ss[0], " ")
		gameID, _ := strconv.Atoi(game[1])
		
		valid := true
		
		sets := strings.Split(ss[1], "; ")
		for _, set := range sets {
			cubes := strings.Split(set, ", ")
			cubeMap := make(map[string]int)
			for _, cube := range cubes {
				c := strings.Split(cube, " ")
				n, _ := strconv.Atoi(c[0])
				cubeMap[c[1]] += n
			}
			if cubeMap["red"] > 12 || cubeMap["green"] > 13 || cubeMap["blue"] > 14 {
				valid = false
				break
			}
		}
		if !valid {
			continue
		}
		total += gameID
	}
	fmt.Println(total)
}

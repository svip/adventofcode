package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
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

type coord struct {
	x int
	y int
}

func findGears(y int, x0 int, x1 int, text []string) []coord {
	var coordsToCheck [][2]int
	for i := x0 - 1; i <= x1+1; i++ {
		for j := y - 1; j <= y+1; j++ {
			if j == y && i >= x0 && i <= x1 {
				continue
			}
			coordsToCheck = append(coordsToCheck, [2]int{i, j})
		}
	}
	var gears []coord
	for _, coords := range coordsToCheck {
		if coords[0] < 0 || coords[1] < 0 || coords[0] >= len(text[0]) || coords[1] >= len(text) {
			continue
		}
		if text[coords[1]][coords[0]] == '*' {
			gears = append(gears, coord{x: coords[0], y: coords[1]})
		}
	}
	return gears
}

func main() {
	input := getInput()

	match := regexp.MustCompile("[0-9]")
	gears := make(map[coord][]int)
	for y, line := range input {
		atNumber := false
		var currentNumber string
		currentStart := -1
		for x, c := range line {
			if match.MatchString(string(c)) {
				if atNumber {
					currentNumber += string(c)
				} else {
					atNumber = true
					currentStart = x
					currentNumber = string(c)
				}
			} else {
				if atNumber {
					atNumber = false
					for _, coord := range findGears(y, currentStart, x-1, input) {
						n, _ := strconv.Atoi(currentNumber)
						gears[coord] = append(gears[coord], n)
					}
					currentNumber = ""
				}
			}
		}
		if atNumber {
			atNumber = false
			for _, coord := range findGears(y, currentStart, len(line)-1, input) {
				n, _ := strconv.Atoi(currentNumber)
				gears[coord] = append(gears[coord], n)
			}
			currentNumber = ""
		}
	}
	total := 0
	for _, gearLists := range gears {
		if len(gearLists) != 2 {
			continue
		}
		total += gearLists[0] * gearLists[1]
	}
	fmt.Println(total)
}

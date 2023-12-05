package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
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

	type seedMap struct {
		from     int
		to       int
		increase int
	}

	var seeds []int
	var maps [][]seedMap
	var currentMaps []seedMap
	for _, line := range input {
		if strings.HasPrefix(line, "seeds: ") {
			for _, s := range strings.Split(strings.Split(line, ": ")[1], " ") {
				n, _ := strconv.Atoi(s)
				seeds = append(seeds, n)
			}
		} else {
			if strings.Count(line, " ") == 2 {
				var src, dst, rgn int
				for i, s := range strings.Split(line, " ") {
					n, _ := strconv.Atoi(s)
					switch i {
					case 0:
						dst = n
					case 1:
						src = n
					case 2:
						rgn = n
					}
				}
				currentMaps = append(currentMaps, seedMap{
					from:     src,
					to:       src + rgn - 1,
					increase: dst - src,
				})
			} else if strings.Trim(line, " ") == "" {
				if len(currentMaps) > 0 {
					maps = append(maps, currentMaps)
				}
			} else if strings.HasSuffix(line, "map:") {
				currentMaps = nil
			}
		}
	}
	maps = append(maps, currentMaps)
	for _, m := range maps {
		for i := range seeds {
			seed := seeds[i]
			//fmt.Print(mNo, seed, " ")
			for _, subM := range m {
				if seed >= subM.from && seed <= subM.to {
					seeds[i] += subM.increase
				}
			}
			//fmt.Println(seeds[i])
		}
	}
	lowest := 100_000_000_000_000
	for _, s := range seeds {
		if s < lowest {
			lowest = s
		}
	}
	fmt.Println(lowest)
}

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

	var maps [][]seedMap
	var currentMaps []seedMap
	var seedLine string
	for _, line := range input {
		if strings.HasPrefix(line, "seeds: ") {
			seedLine = line
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

	ss := strings.Split(strings.Split(seedLine, ": ")[1], " ")
	lowest := 100_000_000_000_000
	for i := 0; i < len(ss); i += 2 {
		s, _ := strconv.Atoi(ss[i])
		r, _ := strconv.Atoi(ss[i+1])
		for j := s; j < s+r; j++ {
			seed := j
			for _, m := range maps {
				for _, subM := range m {
					if seed >= subM.from && seed <= subM.to {
						seed += subM.increase
						break
					}
				}
			}
			if seed < lowest {
				lowest = seed
			}
		}
	}
	fmt.Println(lowest)
}

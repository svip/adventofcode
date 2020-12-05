package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io/ioutil"
	"math"
	"os"
	"sort"
	"strings"
)

func main() {
	body, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	reader := bufio.NewReader(bytes.NewReader(body))

	var seats []int
	for {
		text, err := reader.ReadString('\n')
		if err != nil {
			break
		}
		text = strings.Trim(text, " \n")

		rMin, rMax := 0, 127
		cMin, cMax := 0, 7
		for i, r := range text {
			if i < 7 {
				diff := int(math.Ceil((float64(rMax) - float64(rMin)) / 2.0))
				switch r {
				case 'F':
					rMax = rMax - diff
				case 'B':
					rMin = rMin + diff
				}
			} else {
				diff := int(math.Ceil((float64(cMax) - float64(cMin)) / 2.0))
				switch r {
				case 'L':
					cMax = cMax - diff
				case 'R':
					cMin = cMin + diff
				}
			}
		}

		id := rMin*8 + cMin
		seats = append(seats, id)
	}
	sort.Slice(seats, func(i, j int) bool { return seats[i] < seats[j] })
	previous := seats[0]
	for i, id := range seats {
		if i == 0 {
			continue // ignore
		}
		if id-2 == previous {
			// we found it
			fmt.Println(id - 1)
			break
		}
		previous = id
	}
}

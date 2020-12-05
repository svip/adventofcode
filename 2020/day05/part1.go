package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io/ioutil"
	"math"
	"os"
	"strings"
)

func main() {
	body, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	reader := bufio.NewReader(bytes.NewReader(body))

	highestID := 0
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
		if id > highestID {
			highestID = id
		}
	}
	fmt.Println(highestID)
}

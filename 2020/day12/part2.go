package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io/ioutil"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	body, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	reader := bufio.NewReader(bytes.NewReader(body))

	x, y := 0, 0
	wX, wY := 10, -1

	for {
		text, err := reader.ReadString('\n')
		if err != nil {
			break
		}
		text = strings.Trim(text, " \n")

		i := text[0]
		v, _ := strconv.Atoi(text[1:])

		switch i {
		case 'N':
			wY -= v
		case 'S':
			wY += v
		case 'E':
			wX += v
		case 'W':
			wX -= v
		case 'L':
			switch v {
			case 90:
				wX, wY = wY, wX*-1
			case 180:
				wX, wY = wX*-1, wY*-1
			case 270:
				wX, wY = wY*-1, wX
			}
		case 'R':
			switch v {
			case 90:
				wX, wY = wY*-1, wX
			case 180:
				wX, wY = wX*-1, wY*-1
			case 270:
				wX, wY = wY, wX*-1
			}
		case 'F':
			x, y = x+wX*v, y+wY*v
		}
	}

	abs := func(i int) int {
		return int(math.Abs(float64(i)))
	}

	fmt.Printf("%d + %d = %d\n", abs(x), abs(y), abs(x)+abs(y))
}

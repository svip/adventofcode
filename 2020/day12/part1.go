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
	dir := 90 // N = 0, E = 90, S = 180, W = 270

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
			y -= v
		case 'S':
			y += v
		case 'E':
			x += v
		case 'W':
			x -= v
		case 'L':
			dir -= v
			if dir < 0 {
				dir += 360
			}
		case 'R':
			dir += v
			if dir >= 360 {
				dir -= 360
			}
		case 'F':
			switch dir {
			case 0:
				y -= v
			case 90:
				x += v
			case 180:
				y += v
			case 270:
				x -= v
			}
		}
	}

	abs := func(i int) int {
		return int(math.Abs(float64(i)))
	}

	fmt.Printf("%d + %d = %d\n", abs(x), abs(y), abs(x)+abs(y))
}

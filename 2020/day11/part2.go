package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

type tile int

const (
	floor = iota + 1
	empty
	occupied
)

/*
	1 2 3
	4   6
	7 8 9
*/
func dirToCoord(dir int) (x, y int) {
	switch dir {
	case 1:
		return -1, -1
	case 2:
		return 0, -1
	case 3:
		return 1, -1
	case 4:
		return -1, 0
	case 6:
		return 1, 0
	case 7:
		return -1, 1
	case 8:
		return 0, 1
	case 9:
		return 1, 1
	default:
		return 0, 0
	}
}

func visible(tiles [][]tile, atX, atY int) []tile {
	var visible []tile
	for _, dir := range []int{1, 2, 3, 4, 6, 7, 8, 9} {
		wX, wY := dirToCoord(dir)
		x, y := atX+wX, atY+wY
		for y >= 0 && y < len(tiles) && x >= 0 && x < len(tiles[y]) {
			if tiles[y][x] != floor {
				visible = append(visible, tiles[y][x])
				break
			}
			x += wX
			y += wY
		}
	}
	return visible
}

func main() {
	body, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	reader := bufio.NewReader(bytes.NewReader(body))

	var tiles [][]tile

	for {
		text, err := reader.ReadString('\n')
		if err != nil {
			break
		}
		text = strings.Trim(text, " \n")

		var row []tile
		for _, r := range text {
			if r == 'L' {
				row = append(row, empty)
			} else {
				row = append(row, floor)
			}
		}

		tiles = append(tiles, row)
	}

	for {
		madeChange := false
		nextMap := make([][]tile, len(tiles))
		for y := range tiles {
			nextRow := make([]tile, len(tiles[y]))
		rowLoop:
			for x := range tiles[y] {
				t := tiles[y][x]
				switch t {
				case empty:
					for _, n := range visible(tiles, x, y) {
						if n == occupied {
							// nothing happens, continue on the row
							nextRow[x] = t
							continue rowLoop
						}
					}
					// otherwise, it becomes occupied
					madeChange = true
					nextRow[x] = occupied
				case occupied:
					c := 0
					for _, n := range visible(tiles, x, y) {
						if n == occupied {
							c++
							if c >= 5 {
								// five occupied around it?  It becomes empty
								madeChange = true
								nextRow[x] = empty
								continue rowLoop
							}
						}
					}
					// otherwise, nothing happens
					nextRow[x] = t
				default:
					nextRow[x] = t
				}
			}
			nextMap[y] = nextRow
		}
		tiles = nextMap
		// check if the maps have changed
		if !madeChange {
			break
		}
	}

	occupiedSeats := 0
	for y := range tiles {
		for x := range tiles[y] {
			if tiles[y][x] == occupied {
				occupiedSeats++
			}
		}
	}

	fmt.Println(occupiedSeats)
}

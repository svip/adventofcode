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

func neighbours(tiles [][]tile, atX, atY int) []tile {
	var neighbours []tile
	for x := atX - 1; x <= atX+1; x++ {
		for y := atY - 1; y <= atY+1; y++ {
			if x == atX && y == atY {
				continue
			}
			if y >= 0 && y < len(tiles) && x >= 0 && x < len(tiles[y]) {
				neighbours = append(neighbours, tiles[y][x])
			}
		}
	}
	return neighbours
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
					for _, n := range neighbours(tiles, x, y) {
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
					for _, n := range neighbours(tiles, x, y) {
						if n == occupied {
							c++
							if c >= 4 {
								// four occupied around it?  It becomes empty
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

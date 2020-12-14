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

func getAddresses(address int, found map[int]bool, floats []int, pos int) []int {
	var addresses []int
	if pos > 0 {
		addresses = []int{address}
	}
	for i := pos; i < len(floats); i++ {
		float := floats[i]
		if !found[address|float] {
			found[address|float] = true
			addresses = append(addresses, getAddresses(address|float, found, floats, pos+1)...)
		}
		if !found[address&^float] {
			found[address&^float] = true
			addresses = append(addresses, getAddresses(address&^float, found, floats, pos+1)...)
		}
	}
	return addresses
}

func main() {
	body, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	reader := bufio.NewReader(bytes.NewReader(body))

	var ors []int
	var floats []int

	memory := make(map[int]int)

	for {
		text, err := reader.ReadString('\n')
		if err != nil {
			break
		}
		text = strings.Trim(text, " \n")

		ss := strings.Split(text, " = ")

		switch ss[0] {
		case "mask":
			ors = make([]int, 0, 36)
			floats = make([]int, 0, 36)
			for i, r := range ss[1] {
				v := int(math.Pow(float64(2), float64(35-i)))
				switch r {
				case '1':
					ors = append(ors, v)
				case 'X':
					floats = append(floats, v)
				}
			}
		default:
			address, _ := strconv.Atoi(ss[0][4 : len(ss[0])-1])
			value, _ := strconv.Atoi(ss[1])
			for _, or := range ors {
				address = address | or
			}
			addresses := getAddresses(address, make(map[int]bool), floats, 0)
			for _, address := range addresses {
				memory[address] = value
			}
		}
	}

	sum := 0
	for _, v := range memory {
		sum += v
	}
	fmt.Println(sum)
}

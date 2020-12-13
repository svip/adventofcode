package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io/ioutil"
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

	var estimate int
	var buses []int

	i := 0
	for {
		text, err := reader.ReadString('\n')
		if err != nil {
			break
		}
		text = strings.Trim(text, " \n")

		switch i {
		case 0:
			estimate, _ = strconv.Atoi(text)
		case 1:
			ss := strings.Split(text, ",")
			for _, s := range ss {
				if s == "x" {
					continue
				}
				bus, _ := strconv.Atoi(s)
				buses = append(buses, bus)
			}
		}
		i++
	}

	t := estimate
loop:
	for {
		for _, bus := range buses {
			if t%bus == 0 {
				wait := t - estimate
				fmt.Printf("%d * %d = %d\n", wait, bus, wait*bus)
				break loop
			}
		}
		t++
	}
}

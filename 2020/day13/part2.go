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

	type aBus struct {
		t  int
		id int
	}

	var buses []aBus

	i := 0
	for {
		text, err := reader.ReadString('\n')
		if err != nil {
			break
		}
		text = strings.Trim(text, " \n")

		switch i {
		case 1:
			ss := strings.Split(text, ",")
			for t, s := range ss {
				if s == "x" {
					continue
				}
				bus, _ := strconv.Atoi(s)
				buses = append(buses, aBus{t, bus})
			}
		}
		i++
	}

	t := buses[0].id
	skip := buses[0].id
	busesIn := map[int]bool{
		buses[0].t: true,
	}
	for {
		valid := true
		for _, bus := range buses {
			if (t+bus.t)%bus.id != 0 {
				valid = false
				break
			}
			if _, ok := busesIn[bus.t]; !ok {
				busesIn[bus.t] = true
				skip *= bus.id
			}
		}
		if valid {
			fmt.Println(t)
			break
		}
		t += skip
	}
}

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

type bagContent struct {
	bag    string
	amount int
}

type bag struct {
	bag      string
	contains []bagContent
}

func countBags(bags map[string]bag, bag string) int {
	count := 0
	details := bags[bag]
	for _, content := range details.contains {
		amount := content.amount
		each := countBags(bags, content.bag)
		count += amount + amount*each
	}
	return count
}

func main() {
	body, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	reader := bufio.NewReader(bytes.NewReader(body))

	bags := make(map[string]bag)
	for {
		text, err := reader.ReadString('\n')
		if err != nil {
			break
		}
		text = strings.Trim(text, " \n")
		rules := strings.Split(text, " contain ")

		b := strings.TrimSuffix(rules[0], " bags")
		oneBag := bag{
			bag: b,
		}

		if rules[1] != "no other bags." {
			contains := strings.Split(rules[1], ", ")
			for _, contain := range contains {
				line := strings.Split(contain, " ")
				amount, _ := strconv.Atoi(line[0])
				bag := fmt.Sprintf("%s %s", line[1], line[2])
				oneBag.contains = append(oneBag.contains, bagContent{
					bag:    bag,
					amount: amount,
				})
			}
		}

		bags[b] = oneBag
	}

	count := countBags(bags, "shiny gold")

	fmt.Println(count)
}

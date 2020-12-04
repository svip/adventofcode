package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

type passport struct {
	byr string
	iyr string
	eyr string
	hgt string
	hcl string
	ecl string
	pid string
	cid string
}

func (p passport) valid() bool {
	return len(p.byr) > 0 && len(p.iyr) > 0 && len(p.eyr) > 0 && len(p.hgt) > 0 && len(p.hcl) > 0 && len(p.ecl) > 0 && len(p.pid) > 0
}

func main() {
	body, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	reader := bufio.NewReader(bytes.NewReader(body))

	var passports []passport
	var currentPassport passport
	for {
		text, err := reader.ReadString('\n')
		if err != nil {
			break
		}
		text = strings.Trim(text, " \n")
		if text == "" {
			passports = append(passports, currentPassport)
			currentPassport = passport{}
		}
		ss := strings.Split(text, " ")
		for _, s := range ss {
			e := strings.Split(s, ":")
			switch e[0] {
			case "byr":
				currentPassport.byr = e[1]
			case "iyr":
				currentPassport.iyr = e[1]
			case "eyr":
				currentPassport.eyr = e[1]
			case "hgt":
				currentPassport.hgt = e[1]
			case "hcl":
				currentPassport.hcl = e[1]
			case "ecl":
				currentPassport.ecl = e[1]
			case "pid":
				currentPassport.pid = e[1]
			case "cid":
				currentPassport.cid = e[1]
			}
		}
	}
	passports = append(passports, currentPassport)

	valids := 0
	for _, p := range passports {
		if p.valid() {
			valids++
		}
	}

	fmt.Println(valids)
}

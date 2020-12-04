package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io/ioutil"
	"os"
	"regexp"
	"strconv"
	"strings"
)

var hairColour *regexp.Regexp
var passportID *regexp.Regexp

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
	valid := len(p.byr) == 4 && len(p.iyr) == 4 && len(p.eyr) == 4 && len(p.hgt) >= 4 && len(p.hcl) == 7 && len(p.ecl) == 3 && len(p.pid) == 9
	if !valid {
		return false
	}
	byr, _ := strconv.Atoi(p.byr)
	if byr < 1920 || byr > 2002 {
		return false
	}
	iyr, _ := strconv.Atoi(p.iyr)
	if iyr < 2010 || iyr > 2020 {
		return false
	}
	eyr, _ := strconv.Atoi(p.eyr)
	if eyr < 2020 || eyr > 2030 {
		return false
	}
	unit := p.hgt[len(p.hgt)-2:]
	if unit != "in" && unit != "cm" {
		return false
	}
	hgt, _ := strconv.Atoi(strings.Replace(p.hgt, unit, "", 1))
	switch unit {
	case "in":
		if hgt < 59 || hgt > 76 {
			return false
		}
	case "cm":
		if hgt < 150 || hgt > 193 {
			return false
		}
	default:
		return false
	}
	if p.hcl[0] != '#' {
		return false
	}
	if !hairColour.MatchString(p.hcl[1:]) {
		return false
	}
	switch p.ecl {
	case "amb", "blu", "brn", "gry", "grn", "hzl", "oth":
		break
	default:
		return false
	}
	if !passportID.MatchString(p.pid) {
		return false
	}

	return true
}

func main() {
	hairColour = regexp.MustCompile("[0-9a-f]{6}")
	passportID = regexp.MustCompile("[0-9]{9}")

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

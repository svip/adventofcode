package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	totalFuel := 0
	for scanner.Scan() {
		mass, err := strconv.Atoi(scanner.Text())
		if err != nil {
			panic(err)
		}
		fuel := int(float64(mass)/3.0) - 2
		fuelFuel := int(float64(fuel)/3.0) - 2
		for fuelFuel > 0 {
			fuel += fuelFuel
			fuelFuel = int(float64(fuelFuel)/3.0) - 2
		}
		totalFuel += fuel
	}
	fmt.Println(totalFuel)
}

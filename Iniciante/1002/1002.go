package main

import "fmt"

func main() {
	const PI float64 = 3.14159

	var a float64 = 0.0

	fmt.Scan(&a)

	fmt.Printf("A=%.4f\n", a*a*PI)
}

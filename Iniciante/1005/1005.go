package main

import "fmt"

func main() {
	var a, b float64 = 0.0, 0.0

	fmt.Scan(&a)
	fmt.Scan(&b)

	result := ((a * 3.5) + b*7.5) / 11

	fmt.Printf("MEDIA = %.5f\n", result)
}

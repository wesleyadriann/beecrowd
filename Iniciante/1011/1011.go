package main

import "fmt"

func main() {
	const PI float64 = 3.14159

	var r float64

	fmt.Scan(&r)
	fmt.Printf("VOLUME = %.3f\n", (4.0/3.0)*PI*(r*r*r))
}

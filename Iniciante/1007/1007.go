package main

import "fmt"

func main() {
	var a, b, c, d int

	fmt.Scan(&a, &b, &c, &d)

	result := a*b - c*d

	fmt.Printf("DIFERENCA = %v\n", result)
}

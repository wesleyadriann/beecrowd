package main

import "fmt"

func main() {
	var a, b int = 0, 0

	fmt.Scan(&a)
	fmt.Scan(&b)

	fmt.Println("PROD =", a*b)
}

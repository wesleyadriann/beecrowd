package main

import "fmt"

func main() {
	var n, x, y float64

	fmt.Scan(&n, &x, &y)

	fmt.Printf("%.2f\n", n/(x+y))
}

package main

import "fmt"

func main() {
	var name string
	var salary, sales float64

	fmt.Scan(&name, &salary, &sales)

	fmt.Printf("TOTAL = R$ %.2f\n", sales*0.15+salary)
}

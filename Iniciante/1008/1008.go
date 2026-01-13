package main

import "fmt"

func main() {
	var n, s int
	var v float64

	fmt.Scan(&n, &s, &v)

	fmt.Println("NUMBER =", n)
	fmt.Printf("SALARY = U$ %.2f\n", float64(s)*v)
}

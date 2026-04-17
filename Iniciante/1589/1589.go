package main

import "fmt"

func main() {
	t, r1, r2 := 0, 0, 0

	fmt.Scan(&t)

	for i := 0; i < t; i++ {
		fmt.Scanf("%d %d", &r1, &r2)

		fmt.Println(r1 + r2)
	}
}

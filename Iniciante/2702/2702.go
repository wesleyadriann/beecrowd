package main

import "fmt"

func main() {
	var ca, ba, pa, cr, br, pr int
	result := 0

	fmt.Scanf("%d %d %d", &ca, &ba, &pa)
	fmt.Scanf("%d %d %d", &cr, &br, &pr)

	if cr > ca {
		result += cr - ca
	}

	if br > ba {
		result += br - ba
	}

	if pr > pa {
		result += pr - pa
	}

	fmt.Printf("%d\n", result)
}

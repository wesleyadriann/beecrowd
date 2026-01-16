package main

import "fmt"

func main() {
	var code string
	var qtd_1, qtd_2 int
	var value_1, value_2 float64

	fmt.Scanf("%s %d %f", &code, &qtd_1, &value_1)
	fmt.Scanf("%s %d %f", &code, &qtd_2, &value_2)

	fmt.Printf("VALOR A PAGAR: R$ %.2f\n", float64(qtd_1)*value_1+float64(qtd_2)*value_2)
}

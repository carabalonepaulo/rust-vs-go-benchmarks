package main

import (
	"fmt"
	"time"
)

func main() {
	// Marca o tempo inicial
	start := time.Now()

	// Variável para armazenar o resultado da soma
	var result int

	// Três loops aninhados para realizar os cálculos
	for i := 0; i < 2000; i++ {
		for j := 0; j < 2000; j++ {
			for k := 0; k < 2000; k++ {
				result += i * j * k
			}
		}
	}

	// Medição de tempo após o cálculo
	duration := time.Since(start)

	// Exibindo o resultado e o tempo de execução
	fmt.Printf("Resultado: %d\n", result)
	fmt.Printf("Tempo total: %.2fs\n", duration.Seconds())
}

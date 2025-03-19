package main

import (
	"fmt"
	"time"
)

func main() {
	start := time.Now()

	var result int
	for i := 0; i < 2000; i++ {
		for j := 0; j < 2000; j++ {
			for k := 0; k < 2000; k++ {
				result += i * j * k
			}
		}
	}

	duration := time.Since(start)
	fmt.Printf("Resultado: %d\n", result)
	fmt.Printf("Tempo total: %.2fs\n", duration.Seconds())
}

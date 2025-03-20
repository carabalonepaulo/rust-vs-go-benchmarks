package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"time"

	"github.com/jackc/pgx/v5"
)

func main() {
	startTime := time.Now()

	connectionURL := os.Getenv("TEST_DATABASE_URL")
	if connectionURL == "" {
		log.Fatal("TEST_DATABASE_URL not set")
	}

	connectionURL += "?sslmode=disable"

	db, err := pgx.Connect(context.Background(), connectionURL)
	if err != nil {
		log.Fatalf("Connection error: %v", err)
	}
	defer db.Close(context.Background())

	rowsResult, err := db.Query(context.Background(), "SELECT * FROM test_table")

	defer rowsResult.Close()

	if err != nil {
		log.Fatalf("Failed to scan row: %v", err)
	}

	for rowsResult.Next() {
		var id int
		var name string
		err = rowsResult.Scan(&id, &name)
		if err != nil {
			log.Fatalf("Failed to scan row: %v", err)
		}
		// fmt.Println("Row:", id, name)
	}

	fmt.Printf("Tempo total: %.2f segundos\n", time.Since(startTime).Seconds())
}

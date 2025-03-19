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

	startTime := time.Now()

	_, err = db.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS test_table (
			id SERIAL PRIMARY KEY,
			name TEXT NOT NULL
		)
	`)
	if err != nil {
		log.Fatalf("Failed to create table: %v", err)
	}

	rows := make([][]interface{}, 100000)
	for i := 0; i < 100000; i++ {
		rows[i] = []interface{}{fmt.Sprintf("Item %d", i)}
	}

	_, err = db.CopyFrom(
		context.Background(),
		pgx.Identifier{"test_table"},
		[]string{"name"},
		pgx.CopyFromRows(rows),
	)
	if err != nil {
		log.Fatalf("Failed to batch insert data: %v", err)
	}

	rowsResult, err := db.Query(context.Background(), "SELECT * FROM test_table")
	if err != nil {
		log.Fatalf("Failed to select data: %v", err)
	}
	defer rowsResult.Close()

	for rowsResult.Next() {
		var id int
		var name string
		err = rowsResult.Scan(&id, &name)
		if err != nil {
			log.Fatalf("Failed to scan row: %v", err)
		}
		fmt.Println("Row:", id, name)
	}

	if err = rowsResult.Err(); err != nil {
		log.Fatalf("Error iterating rows: %v", err)
	}

	_, err = db.Exec(context.Background(), "DELETE FROM test_table")
	if err != nil {
		log.Fatalf("Failed to delete data: %v", err)
	}

	_, err = db.Exec(context.Background(), "DROP TABLE IF EXISTS test_table CASCADE")
	if err != nil {
		log.Fatalf("Failed to drop table: %v", err)
	}

	fmt.Printf("Tempo total: %.2f segundos\n", time.Since(startTime).Seconds())
}

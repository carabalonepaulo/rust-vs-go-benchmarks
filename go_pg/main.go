package main

import (
	"database/sql"
	"fmt"
	"log"
	"os"
	"time"

	_ "github.com/lib/pq"
)

func main() {
	connectionURL := os.Getenv("TEST_DATABASE_URL")
	if connectionURL == "" {
		log.Fatal("TEST_DATABASE_URL not set")
	}

	connectionURL = connectionURL + "?sslmode=disable"

	db, err := sql.Open("postgres", connectionURL)
	if err != nil {
		log.Fatal("Connection error: ", err)
	}
	defer db.Close()

	startTime := time.Now()

	_, err = db.Exec(`
		CREATE TABLE test_table (
			id SERIAL PRIMARY KEY,
			name TEXT NOT NULL
		)
	`)
	if err != nil {
		log.Fatal("Failed to create table")
	}

	for i := 0; i < 100000; i++ {
		_, err := db.Exec("INSERT INTO test_table (name) VALUES ($1)", fmt.Sprintf("Item %d", i))
		if err != nil {
			log.Fatal("Failed to insert data")
		}
	}

	_, err = db.Exec("SELECT * FROM test_table")
	if err != nil {
		log.Fatal("Failed to select data")
	}

	_, err = db.Exec("DELETE FROM test_table")
	if err != nil {
		log.Fatal("Failed to delete data")
	}

	_, err = db.Exec("DROP TABLE test_table")
	if err != nil {
		log.Fatal("Failed to drop table")
	}

	duration := time.Since(startTime)

	fmt.Printf("Tempo total: %.2fs\n", duration.Seconds())
}

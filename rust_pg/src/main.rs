use postgres::{Client, NoTls};
use std::env;

use std::time::Instant;

fn main() {
    let connection_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL not set");
    let connection_url_str = connection_url.as_str();

    let mut client = Client::connect(connection_url_str, NoTls).expect("Connection error");

    let start_time = Instant::now();

    client
        .execute(
            "CREATE TABLE test_table (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL
        )",
            &[],
        )
        .expect("Failed to create table");

    for i in 0..100000 {
        client
            .execute(
                "INSERT INTO test_table (name) VALUES ($1)",
                &[&format!("Item {}", i)],
            )
            .expect("Failed to insert data");
    }

    client
        .execute("SELECT * FROM test_table", &[])
        .expect("Failed to select data");

    client
        .execute("DELETE FROM test_table", &[])
        .expect("Failed to delete data");

    client
        .execute("DROP TABLE test_table", &[])
        .expect("Failed to drop table");

    let duration = start_time.elapsed();

    println!("Tempo total: {:.9}", duration.as_secs_f64());
}

use tokio::time::Instant;
use tokio_postgres::{Error, NoTls};

use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let connection_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL not set");

    let (client, connection) = tokio_postgres::connect(&connection_url, NoTls).await?;

    tokio::spawn(connection);

    let start_time = Instant::now();

    client
        .execute(
            "
            CREATE TABLE IF NOT EXISTS test_table (
                id SERIAL PRIMARY KEY,
                name TEXT NOT NULL
            )
            ",
            &[],
        )
        .await?;

    let mut insert_stmt = String::new();
    for i in 0..5_000_000 {
        insert_stmt.push_str(&format!("('{}'),", format_args!("Item {}", i)));
    }

    insert_stmt.pop();
    client
        .execute(
            &format!("INSERT INTO test_table (name) VALUES {}", insert_stmt),
            &[],
        )
        .await?;

    let rows = client.query("SELECT * FROM test_table", &[]).await?;
    for row in rows {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        // println!("Row: {} - {}", id, name);
    }

    client.execute("DELETE FROM test_table", &[]).await?;
    client
        .execute("DROP TABLE IF EXISTS test_table", &[])
        .await?;

    let duration = start_time.elapsed();
    println!("Tempo total: {:.2?} segundos", duration.as_secs_f64());

    Ok(())
}

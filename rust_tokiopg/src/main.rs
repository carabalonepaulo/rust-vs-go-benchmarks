use tokio::time::Instant;
use tokio_postgres::{Error, NoTls};

use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let start_time = Instant::now();

    let connection_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL not set");

    let (client, connection) = tokio_postgres::connect(&connection_url, NoTls).await?;

    tokio::spawn(connection);

    let rows = client.query("SELECT * FROM test_table", &[]).await?;
    for row in rows {
        let _id: i32 = row.get(0);
        let _name: String = row.get(1);
        // println!("Row: {} - {}", id, name);
    }

    let duration = start_time.elapsed();
    println!("Tempo total: {:.2?} segundos", duration.as_secs_f64());

    Ok(())
}

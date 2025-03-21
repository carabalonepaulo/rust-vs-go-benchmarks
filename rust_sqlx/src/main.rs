use sqlx::{Row, postgres::PgPoolOptions};
use std::env;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let connection_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL not set");
    let pool = PgPoolOptions::new().connect(&connection_url).await.unwrap();

    let rows = sqlx::query("SELECT * FROM test_table")
        .fetch_all(&pool)
        .await?;

    rows.iter().for_each(|row| {
        let _id: i32 = row.try_get(0).unwrap();
        let _name: String = row.try_get(1).unwrap();
    });

    let duration = start_time.elapsed();
    println!("Tempo total: {:.2?} segundos", duration.as_secs_f64());

    Ok(())
}

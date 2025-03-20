use sqlx::{Row, postgres::PgPoolOptions};
use std::env;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL not set");
    let pool = PgPoolOptions::new().connect(&connection_url).await.unwrap();
    let start_time = Instant::now();

    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS test_table (
                id SERIAL PRIMARY KEY,
                name TEXT NOT NULL
            )
            "#,
    )
    .execute(&pool)
    .await?;

    let rows: Vec<String> = (0..5_000_000)
        .map(|i| format!("('{}')", format_args!("Item {}", i)))
        .collect();

    let insert_stmt: String = rows.join(", ");
    sqlx::query(&format!(
        "INSERT INTO test_table (name) VALUES {}",
        insert_stmt
    ))
    .execute(&pool)
    .await?;

    let rows = sqlx::query("SELECT id, name FROM test_table")
        .fetch_all(&pool)
        .await?;

    rows.iter().for_each(|row| {
        let _id: i32 = row.try_get(0).unwrap();
        let _name: String = row.try_get(1).unwrap();
    });

    sqlx::query("DELETE FROM test_table").execute(&pool).await?;
    sqlx::query("DROP TABLE IF EXISTS test_table")
        .execute(&pool)
        .await?;

    let duration = start_time.elapsed();
    println!("Tempo total: {:.2?} segundos", duration.as_secs_f64());

    Ok(())
}

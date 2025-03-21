use sqlx::postgres::PgPoolOptions;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let connection_url = std::env::var("TEST_DATABASE_URL").expect("no db url");
    let pool = PgPoolOptions::new().connect(&connection_url).await?;

    let start_time = Instant::now();

    sqlx::query("CREATE TABLE test_table (id SERIAL PRIMARY KEY, name TEXT NOT NULL)")
        .execute(&pool)
        .await?;

    let mut set = tokio::task::JoinSet::new();

    for i in 0..100000 {
        let pool = pool.clone();
        set.spawn(async move {
            sqlx::query("INSERT INTO test_table (name) VALUES ($1)")
                .bind(format!("Item {}", i))
                .execute(&pool)
                .await
        });
    }

    set.join_all().await;

    sqlx::query("SELECT * FROM test_table")
        .execute(&pool)
        .await?;

    sqlx::query("DELETE FROM test_table").execute(&pool).await?;
    sqlx::query("DROP TABLE test_table").execute(&pool).await?;

    println!("Tempo total: {:.9}", start_time.elapsed().as_secs_f64());
    Ok(())
}

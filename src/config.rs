use dotenvy::dotenv;
use sqlx::SqlitePool;

#[derive(Debug, Clone)]
pub struct AppContext {
  pub db: SqlitePool,
}

impl AppContext {
  pub async fn new() -> Self {
    let database = DbConfig::establish_connection().await;

    AppContext {
      db: database.unwrap(),
    }
  }
}

pub struct DbConfig;

impl DbConfig {
  pub async fn establish_connection() -> Result<sqlx::SqlitePool, sqlx::Error> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    sqlx::sqlite::SqlitePoolOptions::new()
      .max_connections(5)
      .connect(&database_url)
      .await
  }
}

#[actix_web::test]
async fn test_db_connection() {
  let result = DbConfig::establish_connection().await;
  assert!(result.is_ok());
}

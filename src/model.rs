use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{prelude::FromRow, sqlite::SqliteQueryResult, Sqlite, SqlitePool};

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Todo {
  pub id: String,
  pub title: String,
  pub is_complete: bool,
  pub created_at: NaiveDateTime,
  pub updated_at: Option<NaiveDateTime>,
}

impl Todo {
  pub fn new(title: String) -> Self {
    Self {
      id: cuid2::create_id(),
      title,
      is_complete: false,
      created_at: chrono::Local::now().naive_local(),
      updated_at: None,
    }
  }

  pub async fn get_todos(db: &SqlitePool) -> Result<Vec<Todo>, sqlx::Error> {
    let sql = "SELECT * FROM todos ORDER BY created_at DESC";
    sqlx::query_as::<Sqlite, Todo>(sql).fetch_all(db).await
  }

  pub async fn get_todo(db: &SqlitePool, id: String) -> Result<Todo, sqlx::Error> {
    let sql = "SELECT * FROM todos WHERE id = ?";
    sqlx::query_as::<Sqlite, Todo>(sql)
      .bind(id)
      .fetch_one(db)
      .await
  }

  pub async fn create_todo(
    db: &SqlitePool,
    title: String,
  ) -> Result<SqliteQueryResult, sqlx::Error> {
    let sql =
      "INSERT INTO todos (id, title, is_complete, created_at, updated_at) VALUES (?,?,?,?,?)";
    let todo = Self::new(title);
    sqlx::query(sql)
      .bind(todo.id)
      .bind(todo.title)
      .bind(todo.is_complete)
      .bind(todo.created_at)
      .bind(todo.updated_at)
      .execute(db)
      .await
  }

  pub async fn toggle_todo(db: &SqlitePool, id: String) -> Result<SqliteQueryResult, sqlx::Error> {
    let sql = "UPDATE todos SET is_complete = NOT is_complete WHERE id = ?";
    sqlx::query(sql).bind(id).execute(db).await
  }

  pub async fn delete_todo(db: &SqlitePool, id: String) -> Result<SqliteQueryResult, sqlx::Error> {
    let sql = "DELETE FROM todos WHERE id = ?";
    sqlx::query(sql).bind(id).execute(db).await
  }
}

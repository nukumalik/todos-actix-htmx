use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use std::fs;

use crate::{config::AppContext, model::Todo};

#[actix_web::get("/api/v1/todos")]
pub async fn get_todos(ctx: web::Data<AppContext>) -> impl Responder {
  let todos = Todo::get_todos(&ctx.db).await.unwrap_or(vec![]);
  let template = fs::read_to_string("templates/partials/todo_item.html").unwrap();
  let body = todos
    .iter()
    .map(|todo| {
      template
        .replace("{{todo.id}}", &todo.id)
        .replace("{{todo.title}}", &todo.title)
    })
    .collect::<String>();
  HttpResponse::Ok().content_type("text/html").body(body)
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTodo {
  pub title: String,
}

#[actix_web::post("/api/v1/todos")]
pub async fn create_todo(
  ctx: web::Data<AppContext>,
  body: web::Form<CreateTodo>,
) -> impl Responder {
  let _ = Todo::create_todo(&ctx.db, body.title.to_owned()).await;

  let todos = Todo::get_todos(&ctx.db).await.unwrap_or(vec![]);
  let template = fs::read_to_string("templates/partials/todo_item.html").unwrap();
  let body = todos
    .iter()
    .map(|todo| {
      template
        .replace("{{todo.id}}", &todo.id)
        .replace("{{todo.title}}", &todo.title)
    })
    .collect::<String>();

  HttpResponse::Created().content_type("text/html").body(body)
}

#[actix_web::put("/api/v1/todos/{id}/toggle")]
pub async fn toggle_todo(ctx: web::Data<AppContext>, id: web::Path<String>) -> impl Responder {
  let _ = Todo::toggle_todo(&ctx.db, id.to_owned()).await;
  let todo = Todo::get_todo(&ctx.db, id.to_owned()).await.unwrap();
  let is_complete = if todo.is_complete {
    "text-decoration-line-through"
  } else {
    ""
  };
  let body = format!(
    "<span id='title-{}' class='{}'>{}</span>",
    todo.id, is_complete, todo.title
  );
  HttpResponse::Ok().content_type("text/html").body(body)
}

#[actix_web::delete("/api/v1/todos/{id}")]
pub async fn delete_todo(ctx: web::Data<AppContext>, id: web::Path<String>) -> impl Responder {
  let _ = Todo::delete_todo(&ctx.db, id.to_owned()).await;
  HttpResponse::Ok().content_type("text/html").body("")
}

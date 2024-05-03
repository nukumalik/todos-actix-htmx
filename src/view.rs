use actix_web::{web, Responder};
use actix_web_lab::respond::Html;
use askama::Template;

use crate::{config::AppContext, model::Todo};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
  message: String,
  todos: Vec<Todo>,
}

#[actix_web::get("/")]
pub async fn index(ctx: web::Data<AppContext>) -> impl Responder {
  let todos = Todo::get_todos(&ctx.db).await.unwrap_or(vec![]);
  let template = IndexTemplate {
    message: "".to_string(),
    todos,
  };
  let body = template.render().unwrap();
  Html(body)
}

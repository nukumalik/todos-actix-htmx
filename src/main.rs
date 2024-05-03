use actix_web::{web, App, HttpServer};

pub mod api;
pub mod config;
pub mod model;
pub mod view;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let context = config::AppContext::new().await;

  let server = HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(context.clone()))
      .service(view::index)
      .service(api::get_todos)
      .service(api::create_todo)
      .service(api::toggle_todo)
      .service(api::delete_todo)
  });
  println!("🚀 Listening on http://localhost:3000");
  server.bind("localhost:3000").unwrap().run().await
}

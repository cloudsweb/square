use std::ops::DerefMut;

use crate::{db, db::Pool};
use actix_web::{get, post, web, App, HttpServer, Responder};

#[post("/users/create")]
async fn create_user(conn: web::Data<Pool>) -> impl Responder {
  let mut conn = conn.get().expect("database error");
  let result = db::UserCreate {
    alias: "test".to_string(), name: "test name".to_string(), 
    description: None,
    avatar: None,
  }.exec(conn.deref_mut());
  match result {
    Ok(()) => "succeed".to_string(),
    Err(e) => format!("error: {:?}", e),
  }
}


#[get("/{id}/{title}")]
async fn index(params: web::Path<(u32, String)>) -> impl Responder {
  let (id, title) = params.into_inner();
  format!("{}, author: {}", title, id)
}

// #[actix_web::main]
pub async fn run(bind_addr: &str, conn: Pool) -> std::io::Result<()> {
  let data = web::Data::new(conn);
  HttpServer::new(move ||
    App::new()
      .service(create_user)
      .service(index)
      .app_data(data.clone())
  ).bind(bind_addr)?.run().await
}

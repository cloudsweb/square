use crate::{db, db::Pool};
use actix_web::{App, HttpServer, Responder, get, http::StatusCode, post, web};

#[derive(Debug, serde::Deserialize)]
pub struct UserCreateInfo {
  pub alias: String,
  pub name: String,
  pub password: String,
  pub email: Option<String>,
  pub description: Option<String>,
  pub avatar: Option<String>, // TODO: upload?
}

#[derive(Debug, serde::Deserialize)]
pub struct UserLoginInfo {
  pub alias: String,
  pub password: String,
}

#[post("/users/create")]
async fn create_user(conn: web::Data<Pool>, info: web::Json<UserCreateInfo>) -> impl Responder {
  let mut conn = conn.get().expect("database error");
  println!("create_user {:?}", info);

  let result: anyhow::Result<_> = conn.build_transaction().run(|conn| {
    let id = db::UserCreate {
      alias: info.alias.clone(), name: info.name.clone(), 
      description: info.description.clone(),
      avatar: info.avatar.clone(),
    }.exec(conn)?;
    db::UserPassword::new(id, info.password.clone(), conn).exec(conn)?;
    Ok(id)
  });
  match result {
    Ok(id) => (format!("success: {}", id), StatusCode::OK),
    Err(e) => (format!("error: {:?}", e), StatusCode::BAD_REQUEST), // TODO: 400 bad requests
  }
}

#[post("/users/login")]
async fn create_login(conn: web::Data<Pool>, info: web::Json<UserLoginInfo>) -> impl Responder {
  let mut conn = conn.get().expect("database error");

  let result: anyhow::Result<_> = conn.build_transaction().run(|conn| {
    let id = db::UserCreate::find_id(&info.alias, conn)?;
    db::UserPassword::check(id, &info.password, conn)
  });
  match result {
    Ok(true) => (format!("success: {}", info.alias), StatusCode::OK),
    Ok(false) => (format!("error: wrong password"), StatusCode::FORBIDDEN),
    Err(e) => (format!("error: {:?}", e), StatusCode::BAD_REQUEST), // TODO: 400 bad requests
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

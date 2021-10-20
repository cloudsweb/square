use crate::{db, db::Pool};
// use actix_web::{App, HttpServer, Responder, get, http::StatusCode, post, web};
use axum::{AddExtensionLayer, Json, Router, extract::{Extension, Path as UrlPath}, handler::{get, post}, http::StatusCode, response::IntoResponse};

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

// #[post("/users/create")]
async fn create_user(conn: Extension<Pool>, Json(info): Json<UserCreateInfo>) -> impl IntoResponse {
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
    Ok(id) => (StatusCode::OK, format!("success: {}", id)),
    Err(e) => (StatusCode::BAD_REQUEST, format!("error: {:?}", e)), // TODO: 400 bad requests
  }
}

// #[post("/users/login")]
async fn login_user(conn: Extension<Pool>, Json(info): Json<UserLoginInfo>) -> impl IntoResponse {
  let mut conn = conn.get().expect("database error");

  let result: anyhow::Result<_> = conn.build_transaction().run(|conn| {
    let id = db::UserCreate::find_id(&info.alias, conn)?;
    db::UserPassword::check(id, &info.password, conn)
  });
  match result {
    Ok(true) => (StatusCode::OK, format!("success: {}", info.alias)),
    Ok(false) => (StatusCode::FORBIDDEN, format!("error: wrong password")),
    Err(e) => (StatusCode::BAD_REQUEST, format!("error: {:?}", e)), // TODO: 400 bad requests
  }
}

// #[get("/{id}/{title}")]
async fn index(UrlPath((id, title)): UrlPath<(u32, String)>) -> impl IntoResponse {
  format!("{}, author: {}", title, id)
}

// #[actix_web::main]
pub async fn run(bind_addr: &str, conn: Pool) -> std::io::Result<()> {
  let bind_addr = bind_addr.parse().map_err(|_| std::io::ErrorKind::InvalidInput)?;
  // build our application with a route
  let app = Router::new()
    // `GET /` goes to `root`
    .route("/users/create", post(create_user))
    // `POST /users` goes to `create_user`
    .route("/users/login", post(login_user))
    .route("/:id/:title", get(index))
    .layer(AddExtensionLayer::new(conn));
  // tracing::debug!("listening on {}", addr);
  axum::Server::bind(&bind_addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
  Ok(())
}

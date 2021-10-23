use crate::{auth::Claims, db, db::Pool};
// use actix_web::{App, HttpServer, Responder, get, http::StatusCode, post, web};
use axum::{AddExtensionLayer, Json, Router, extract::{Extension, Path as UrlPath}, handler::{get, post}, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub enum JsonResponse {
  Ok(serde_json::Value),
  Error {
    status: StatusCode, code: i32, msg: String,
  },
}

impl JsonResponse {
  pub fn error<S: ToString>(status: StatusCode, code: i32, msg: S) -> Self {
    JsonResponse::Error { status, code, msg: msg.to_string() }
  }

  pub fn to_json(self) -> (StatusCode, Json<serde_json::Value>) {
    match self {
      JsonResponse::Ok(v) => (StatusCode::OK, Json(v)),
      JsonResponse::Error{ status, code, msg } => (status, Json(json!({
        "code": code, "msg": msg
      }))),
    }
  }
}

impl IntoResponse for JsonResponse {
  type Body = axum::body::Full<axum::body::Bytes>;
  type BodyError = std::convert::Infallible;

  fn into_response(self) -> axum::http::Response<Self::Body> {
    self.to_json().into_response()
  }
}

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
async fn create_user(conn: Extension<Pool>, Json(info): Json<UserCreateInfo>) -> JsonResponse {
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
    Ok(id) => JsonResponse::Ok(json!({"id": id})),
    Err(e) => JsonResponse::error(StatusCode::BAD_REQUEST, 1, format!("{:?}", e)),
  }
}

// #[post("/users/login")]
async fn login_user(conn: Extension<Pool>, Json(info): Json<UserLoginInfo>) -> JsonResponse {
  let mut conn = conn.get().expect("database error");

  let result: anyhow::Result<_> = conn.build_transaction().run(|conn| {
    let id = db::UserCreate::find_id(&info.alias, conn)?;
    db::UserPassword::check(id, &info.password, conn)
  });
  let token = "";
  match result {
    Ok(true) => JsonResponse::Ok(json!({ "token": token })),
    Ok(false) => JsonResponse::error(StatusCode::FORBIDDEN, 1, "wrong password"),
    Err(e) => JsonResponse::error(StatusCode::BAD_REQUEST, 1, format!("{:?}", e)),
  }
}

// #[get("/{id}/{title}")]
async fn index(UrlPath((id, title)): UrlPath<(u32, String)>, claims: Option<Claims>) -> impl IntoResponse {
  info!("claims: {:?}", claims);
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

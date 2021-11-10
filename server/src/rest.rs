use crate::{auth::{self, Claims}, db, db::Pool};
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
async fn user_create(conn: Extension<Pool>, Json(info): Json<UserCreateInfo>) -> JsonResponse {
  let mut conn = conn.get().expect("database error");
  println!("user_create {:?}", info);

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
async fn user_login(conn: Extension<Pool>, Json(info): Json<UserLoginInfo>) -> JsonResponse {
  let mut conn = conn.get().expect("database error");

  let result: anyhow::Result<_> = conn.build_transaction().run(|conn| {
    let id = db::UserInfo::find_id(&info.alias, conn)?;
    let correct = db::UserPassword::check(id, &info.password, conn)?;
    Ok((id, correct))
  });
  match result {
    Ok((id, true)) => {
      match auth::Claims::new(id, 3600).issue() {
        Ok(token) => JsonResponse::Ok(json!({ "token": token })),
        Err(e) => JsonResponse::error(StatusCode::INTERNAL_SERVER_ERROR, 1, format!("{:?}", e)),
      }
    },
    Ok((_, false)) => JsonResponse::error(StatusCode::FORBIDDEN, 1, "wrong password"),
    Err(e) => JsonResponse::error(StatusCode::BAD_REQUEST, 1, format!("{:?}", e)),
  }
}

async fn user_info(conn: Extension<Pool>, UrlPath((id,)): UrlPath<(u64,)>, claims: Option<Claims>) -> JsonResponse {
  let mut conn = conn.get().expect("database error");

  let is_me = claims.as_ref().map(|i| i.sub == format!("#{}", id)) == Some(true);
  info!("claims({}): {:?}", is_me, claims);
  let result: anyhow::Result<_> = conn.build_transaction().run(|conn| {
    Ok(db::UserInfo::get(id as i64, conn)?)
  });
  match result {
    Ok(result) => {
      JsonResponse::Ok(json!({ "alias": result.alias, "nickname": result.name }))
    },
    Err(e) => JsonResponse::error(StatusCode::BAD_REQUEST, 1, format!("{:?}", e)),
  }
}

// #[get("/{id}/{title}")]
async fn index(UrlPath((id, title)): UrlPath<(u32, String)>, claims: Option<Claims>) -> impl IntoResponse {
  info!("claims: {:?}", claims);
  format!("{}, author: {}", title, id)
}

// #[get("/{id}/{title}")]
async fn new_index(conn: Extension<Pool>, UrlPath((alias, title)): UrlPath<(String, String)>, claims: Option<Claims>, content: String) -> impl IntoResponse {
  let mut conn = conn.get().expect("database error");

  info!("claims: {:?}", claims);
  let id = alias.parse::<i64>().ok();
  let result: anyhow::Result<_> = conn.build_transaction().run(|conn| {
    let id = match id {
      Some(id) => id,
      None => db::UserInfo::find_id(&alias, conn)?,
    };
    let user = db::UserInfo::get(id, conn)?;
    let post_id = db::PostCreate {
      author_id: id,
      author_name: user.name,
      title, content,
    }.exec(conn)?;
    Ok(post_id)
  });
  match result {
    Ok(result) => {
      JsonResponse::Ok(json!({ "id": result.to_string() }))
    },
    Err(e) => JsonResponse::error(StatusCode::BAD_REQUEST, 1, format!("{:?}", e)),
  }
}

// #[actix_web::main]
pub async fn run(bind_addr: &str, conn: Pool) -> std::io::Result<()> {
  let bind_addr = bind_addr.parse().map_err(|_| std::io::ErrorKind::InvalidInput)?;
  // build our application with a route
  let app = Router::new()
    // `GET /` goes to `root`
    .route("/users/create", post(user_create))
    // `POST /users` goes to `create_user`
    .route("/users/login", post(user_login))
    .route("/users/:id/info", get(user_info))
    .route("/:id/:title", get(index))
    .route("/:id/:title", post(new_index))
    .layer(AddExtensionLayer::new(conn));
  // tracing::debug!("listening on {}", addr);
  axum::Server::bind(&bind_addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
  Ok(())
}

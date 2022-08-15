use crate::common::*;
// use actix_web::{App, HttpServer, Responder, get, http::StatusCode, post, web};
use axum::{
  Json, Router,
  extract::{Path as UrlPath, OriginalUri},
  routing::{get, post},
};
use tower_http::{trace::TraceLayer};
use serde_json::json;

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

  let result: db::Result<_> = conn.build_transaction().run(|conn| {
    let id = db::UserCreate {
      alias: info.alias.clone(), name: info.name.clone(),
      description: info.description.clone(),
      avatar: info.avatar.clone(),
    }.exec(conn)?;
    db::UserPassword::new(id, info.password.clone(), conn).exec(conn)?;
    Ok(id)
  });
  match result {
    Ok(id) => json!({"id": id}).into(),
    Err(e) => return Error::from(e).into(),
  }
}

// #[post("/users/login")]
async fn user_login(conn: Extension<Pool>, Json(info): Json<UserLoginInfo>) -> JsonResponse {
  let mut conn = conn.get().expect("database error");

  match Claims::login(&info.alias, &info.password, &mut conn).and_then(|claim| claim.issue()) {
    Ok(token) => json!({"token": token}).into(),
    Err(e) => return Error::from(e).into(),
  }
}

async fn user_info(conn: Extension<Pool>, UrlPath((id,)): UrlPath<(u64,)>, claims: Option<Claims>) -> JsonResponse {
  let mut conn = conn.get().expect("database error");

  let is_me = claims.as_ref().map(|i| i.check_id(id)) == Some(true);
  info!("claims({}): {:?}", is_me, claims);
  let result: db::Result<_> = conn.build_transaction().run(|conn| {
    Ok(db::UserInfo::get(id, conn)?)
  });
  match result {
    Ok(Some(result)) => {
      json!({ "alias": result.alias, "nickname": result.name }).into()
    },
    Ok(None) => return Error::UserNotFound(id).into(),
    Err(e) => return Error::from(e).into(),
  }
}

// #[get("/{id}/{title}")]
async fn index(UrlPath((id, title)): UrlPath<(u64, String)>, claims: Option<Claims>) -> impl IntoResponse {
  format!("{}, author: {}", title, id)
}

// #[get("/{id}/{title}")]
async fn new_index(conn: Extension<Pool>, UrlPath((alias, title)): UrlPath<(String, String)>, uri: OriginalUri, claims: Option<Claims>, content: String) -> JsonResponse {
  let mut conn = conn.get().expect("database error");

  let user = match claims.and_then(|i| i.info) {
    Some(user) => user,
    None => return Error::LoginRequired(uri.0.to_string()).into(),
  };
  if user.alias != alias {
    return Error::NotPermitted { alias: user.alias.clone(), owned: Some(alias.clone()), target: uri.0.to_string() }.into()
  }
  let result: db::Result<_> = conn.build_transaction().run(|conn| {
    let post_id = db::PostCreate {
      author_id: user.id as i64,
      author_name: user.name,
      title, content,
    }.exec(conn)?;
    Ok(post_id)
  });
  match result {
    Ok(result) => json!({ "id": result.to_string() }).into(),
    Err(e) => return Error::from(e).into(),
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
    .layer(TraceLayer::new_for_http())
    .layer(Extension(conn));
  info!("listening on {}", bind_addr);
  axum::Server::bind(&bind_addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
  Ok(())
}

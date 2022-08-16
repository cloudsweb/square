pub use crate::db::{self, Pool};
pub use crate::auth::{self, Claims};
pub use crate::session::{self, SessionData, MutSessionData};
pub use thiserror::Error as ThisError;
pub use serde::{Serialize, Deserialize};
pub use axum::{
  async_trait,
  extract::{Extension, FromRequest, RequestParts},
  http::StatusCode,
  response::{IntoResponse, Response},
};
pub type UserId = u64;

use serde_json::json;
use axum::Json;

pub enum ArgUserId {
  Id(UserId),
  Alias(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ErrorCode {
  Auth = 30000,
  UserNotFound,
  Internal,
  Database,
  Unauthenticated, // 401 unauthorized
  Unauthorized, // 403 forbidden
}

#[derive(Debug, ThisError)]
pub enum Error {
  #[error("auth error: {0}")]
  Auth(#[from] auth::Error),
  #[error("user {0} not found")]
  UserNotFound(UserId),
  #[error("db error: {0}")]
  Db(#[from] db::Error),
  #[error("login required for {0}")]
  LoginRequired(String),
  #[error("user {alias} cannot access {}{target}", owned.clone().unwrap_or_default())]
  NotPermitted { alias: String, owned: Option<String>, target: String }
}

pub trait IntoErrorResp: std::error::Error + Sized {
  fn error_code(&self) -> (StatusCode, ErrorCode);
  fn into_error_resp(self) -> (StatusCode, ErrorCode, String) {
    let (status, code) = self.error_code();
    (status, code, self.to_string())
  }
}

impl IntoErrorResp for Error {
  fn error_code(&self) -> (StatusCode, ErrorCode) {
    match self {
      Error::Auth(err) => err.error_code(),
      Error::UserNotFound(_) => (StatusCode::NOT_FOUND, ErrorCode::UserNotFound),
      Error::Db(_) => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Database),
      Error::LoginRequired(_) => (StatusCode::UNAUTHORIZED, ErrorCode::Unauthenticated),
      Error::NotPermitted{..} => (StatusCode::FORBIDDEN, ErrorCode::Unauthorized),
    }
  }
}

impl IntoResponse for Error {
  fn into_response(self) -> Response {
    // TODO: force json?
    let (s, _, m) = self.into_error_resp();
    (s, m).into_response()
  }
}

pub enum JsonResponse {
  Ok(serde_json::Value),
  Error {
    status: StatusCode, code: ErrorCode, msg: String,
  },
}

impl From<serde_json::Value> for JsonResponse {
  fn from(value: serde_json::Value) -> Self {
    JsonResponse::Ok(value)
  }
}

impl From<Error> for JsonResponse {
  fn from(err: Error) -> Self {
    let (status, code, msg) = err.into_error_resp();
    JsonResponse::Error { status, code, msg }
  }
}

impl JsonResponse {
  pub fn error<S: ToString>(status: StatusCode, code: ErrorCode, msg: S) -> Self {
    JsonResponse::Error { status, code, msg: msg.to_string() }
  }

  pub fn to_json(self) -> (StatusCode, Json<serde_json::Value>) {
    match self {
      JsonResponse::Ok(v) => (StatusCode::OK, Json(v)),
      JsonResponse::Error{ status, code, msg } => (status, Json(json!({
        "code": code as i32, "msg": msg
      }))),
    }
  }
}

impl IntoResponse for JsonResponse {
  fn into_response(self) -> Response {
    self.to_json().into_response()
  }
}

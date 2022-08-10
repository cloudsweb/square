use std::time::SystemTime;

use axum::{extract::{FromRequest, RequestParts, TypedHeader}, http::StatusCode};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::ErrorKind};
use serde::{Deserialize, Serialize};

const KEY: &str = "";

#[derive(Debug)]
pub enum Error {
  JWT(jsonwebtoken::errors::ErrorKind), // no clone for this
  Authorization,
}

impl From<ErrorKind> for Error {
  fn from(e: ErrorKind) -> Self {
    Error::JWT(e)
  }
}

impl axum::response::IntoResponse for Error {
  fn into_response(self) -> axum::response::Response {
    match self {
      Error::JWT(e) => (StatusCode::BAD_REQUEST, format!("invalid token: {:?}", e)),
      Error::Authorization => (StatusCode::FORBIDDEN, format!("no permission to resource")),
    }.into_response()
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub sub: String,
  pub exp: u64,
}

impl Claims {
  pub fn new(id: i64, duration: u64) -> Self {
    let exp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
      .map(|i| i.as_secs()+duration).unwrap_or_default();
    Self {
      sub: format!("#{}", id), exp
    }
  }

  pub fn issue(&self) -> anyhow::Result<String> {
    let s = encode(&Header::default(), self, &EncodingKey::from_secret(KEY.as_bytes()))?;
    Ok(s)
  }
}

#[async_trait]
impl<B> FromRequest<B> for Claims
where
  B: Send,
{
  type Rejection = Error;

  async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
    // Extract the token from the authorization header
    let TypedHeader(headers::Authorization(bearer)) =
      TypedHeader::<headers::Authorization<headers::authorization::Bearer>>::from_request(req)
        .await.map_err(|_| ErrorKind::InvalidToken)?;
    // Decode the user data
    let token_data = decode(bearer.token(), &DecodingKey::from_secret(KEY.as_bytes()), &Validation::default())
      .map_err(|_| ErrorKind::InvalidToken)?;

    Ok(token_data.claims)
  }
}

use crate::{common::*, db::UserInfo};
use std::time::SystemTime;

use axum::extract::TypedHeader;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::ErrorKind};

const KEY: &str = "";

pub type Result<T, E=Error> = std::result::Result<T, E>;
#[derive(Debug, Clone, ThisError)]
pub enum Error {
  #[error("auth internal error: {0}")]
  Internal(&'static str),
  #[error("login failed: {0}")]
  Login(&'static str),
  #[error("signing failed: {0:?}")]
  Signing(jsonwebtoken::errors::Error),
  #[error("invalid token: {0:?}")]
  Authentication(jsonwebtoken::errors::ErrorKind), // no clone for this
  #[error("no permision to resource")]
  Authorization,
}

impl From<ErrorKind> for Error {
  fn from(e: ErrorKind) -> Self {
    Error::Authentication(e)
  }
}

impl IntoErrorResp for Error {
  fn error_code(&self) -> (StatusCode, ErrorCode) {
    match self {
      Error::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Auth),
      _ => (StatusCode::FORBIDDEN, ErrorCode::Auth)
    }
  }
}

impl IntoResponse for Error {
  fn into_response(self) -> Response {
    let (s, _, m) = self.into_error_resp();
    (s, m).into_response()
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub sub: UserId,
  pub exp: u64,
  #[serde(skip)]
  pub info: Option<UserInfo>,
}

impl Claims {
  pub fn new(id: UserId, duration: u64) -> Self {
    let exp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
      .map(|i| i.as_secs()+duration).unwrap_or_default();
    Self {
      sub: id, exp, info: None,
    }
  }

  pub fn issue(&self) -> Result<String, Error> {
    let s = encode(&Header::default(), self, &EncodingKey::from_secret(KEY.as_bytes())).map_err(Error::Signing)?;
    Ok(s)
  }

  pub fn check_id(&self, id: UserId) -> bool {
    self.sub == id
  }

  pub fn login(alias: &str, password: &str, conn: &mut db::Conn) -> Result<Self> {
    let result: db::Result<_> = conn.build_transaction().run(|conn| {
      let id = db::UserInfo::find_id(alias, conn)?;
      if id.is_none() {
        return Ok((None, false))
      }
      let correct = db::UserPassword::check(id.expect("none checked"), &password, conn)?;
      Ok((id, correct))
    });
    match result {
      Ok((Some(id), true)) => Ok(Self::new(id, 3600)),
      Ok((None, _)) => Err(Error::Login("wrong alias")),
      Ok((_, false)) => Err(Error::Login("wrong password")),
      Err(_) => Err(Error::Internal("db check password")),
    }
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
      TypedHeader::<headers::Authorization<headers::authorization::Bearer>>::from_request(req).await.map_err(|_| ErrorKind::InvalidToken)?;
    let Extension(pool) = Extension::<Pool>::from_request(req).await.map_err(|_| Error::Internal("pool not presence"))?;
    // Decode the user data
    let mut token_data = decode::<Claims>(bearer.token(), &DecodingKey::from_secret(KEY.as_bytes()), &Validation::default())
      .map_err(|err| err.into_kind())?;

    let mut conn = pool.get().map_err(|_| Error::Internal("pool get conn"))?;
    token_data.claims.info = UserInfo::get(token_data.claims.sub, &mut conn).map_err(|_| Error::Internal("user_info get"))?;
    if token_data.claims.info.is_none() {
      return Err(Error::Internal("user not found"))?;
    }

    debug!("claims: {:?}", token_data.claims);
    Ok(token_data.claims)
  }
}

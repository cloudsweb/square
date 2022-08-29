pub use crate::common::*;

use axum_sessions::{extractors::{ReadableSession, WritableSession}, async_session::Session};

#[derive(Debug, Clone, Copy, ThisError)]
pub enum Error {
  #[error("session not setup")]
  SessionSetup,
  #[error("{0} not presence")]
  Presence(&'static str),
  #[error("db not setup")]
  DbSetup,
  #[error("db exec failed")]
  DbExec,
  #[error("user not found")]
  DbResult,
}

impl IntoResponse for Error {
  fn into_response(self) -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionData {
  pub id: u64,
  pub alias: String,
  #[serde(skip)]
  pub info: Option<db::UserInfo>,
}

impl SessionData {
  pub async fn from(session: &Session, pool: Pool) -> Result<Self, Error> {
    debug!("session len: {}, id: {:?}, alias: {:?}", session.len(), session.get_raw("id"), session.get_raw("alias"));
    let id = session.get::<u64>("id").ok_or_else(|| Error::Presence("id"))?;
    let mut conn = pool.get().map_err(|_| Error::DbSetup)?;
    let info = db::UserInfo::get(id, &mut conn).map_err(|_| Error::DbExec)?.ok_or_else(|| Error::DbResult)?;
    Ok(Self {
      id,
      alias: session.get_raw("alias").ok_or_else(|| Error::Presence("alias"))?,
      info: Some(info),
    })
  }
}

#[derive(Debug, Serialize)]
pub struct MutSessionData {
  cache: SessionData,
  #[serde(skip)]
  writer: WritableSession,
}

impl MutSessionData {
  pub fn id(&self) -> u64 {
    self.cache.id
  }

  pub fn alias(&self) -> &str {
    &self.cache.alias
  }

  pub fn set_id(&mut self, id: u64) {
    self.writer.insert_raw("id", id.to_string());
    self.cache.id = id
  }

  pub fn set_alias(&mut self, alias: &str) {
    self.writer.insert_raw("alias", alias.to_string());
    self.cache.alias = alias.to_string();
  }

  pub fn save(&mut self) {
    debug!("save {} {}", self.cache.id, self.cache.alias);
    self.writer.insert_raw("id", self.cache.id.to_string());
    self.writer.insert_raw("alias", self.cache.alias.to_string());
  }

  pub fn from(data: db::UserInfo, writer: WritableSession) -> Self {
    Self {
      cache: SessionData { id: data.id as _, alias: data.alias.clone(), info: Some(data) },
      writer
    }
  }
}

#[async_trait]
impl<B: Send> FromRequest<B> for SessionData {
  type Rejection = Error;

  async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
    let session = ReadableSession::from_request(req).await.map_err(|_| Error::SessionSetup)?;
    let Extension(pool) = Extension::<Pool>::from_request(req).await.map_err(|_| Error::DbSetup)?;
    SessionData::from(&session, pool).await
  }
}

#[async_trait]
impl<B: Send> FromRequest<B> for MutSessionData {
  type Rejection = Error;

  async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
    let session = WritableSession::from_request(req).await.map_err(|_| Error::SessionSetup)?;
    let Extension(pool) = Extension::<Pool>::from_request(req).await.map_err(|_| Error::DbSetup)?;
    let cache = SessionData::from(&session, pool).await?;
    Ok(MutSessionData {
      cache, writer: session
    })
  }
}

use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub sub: String,
  pub exp: usize,
}

impl Claims {
  pub fn new(id: i64, exp: usize) -> Self {
    Self {
      sub: format!("#{}", id), exp
    }
  }

  pub fn issue(&self) -> anyhow::Result<String> {
    let s = encode(&Header::default(), self, &EncodingKey::from_secret(b""))?;
    Ok(s)
  }
}

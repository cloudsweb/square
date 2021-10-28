use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use rand::Rng;
use sha2::Digest;
use crate::diesel::ExpressionMethods;

use crate::schema::{users, secrets, posts};

pub type Conn = diesel::PgConnection;
pub type Pool<Conn=diesel::PgConnection> = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<Conn>>;

pub fn connect(url: &str) -> anyhow::Result<Pool> {
  let manager = diesel::r2d2::ConnectionManager::<diesel::PgConnection>::new(url);
  Ok(diesel::r2d2::Pool::builder().build(manager)?)
}

#[derive(Queryable, PartialEq, Debug, Selectable)]
#[table_name = "users"]
pub struct UserInfo {
  pub id: i64,
  pub alias: String,
  pub name: String,
}

impl UserInfo {
  pub fn find_id(id: i64, conn: &mut Conn) -> anyhow::Result<Self> {
    let result = users::table.select(Self::as_select()).filter(users::id.eq(id)).get_result(conn)?;
    Ok(result)
  }
}

#[derive(PartialEq, Debug, Insertable)]
#[table_name = "users"]
pub struct UserCreate {
  pub alias: String,
  pub name: String,
  pub description: Option<String>,
  pub avatar: Option<String>, // TODO: upload?
}

impl UserCreate {
  pub fn exec(self, conn: &mut Conn) -> anyhow::Result<i64> {
    let id = diesel::insert_into(users::table).values(&self).returning(users::id).get_result(conn)?;
    Ok(id)
  }

  pub fn find_id(alias: &str, conn: &mut Conn) -> anyhow::Result<i64> {
    let id = users::table.select(users::id).filter(users::alias.eq(alias)).get_result(conn)?;
    Ok(id)
  }
}

#[derive(PartialEq, Debug, Queryable, Selectable, Insertable)]
#[table_name = "secrets"]
pub struct UserPassword {
  pub id: i64,
  pub current: String,
  pub salt: String,
}

impl UserPassword {
  pub fn gen_salt() -> String {
    let mut rng = rand::thread_rng();
    let bytes: [u8; 16] = rng.gen();
    Self::_hash(&bytes)
  }
  fn _hash(x: &[u8]) -> String {
    let s = sha2::Sha256::digest(x);
    format!("{:x}", s)
  }
  pub fn hash_salt<S1: AsRef<str>, S2: AsRef<str>>(password: S1, salt: S2) -> String {
    let result = Self::_hash(format!("{}${}", password.as_ref(), salt.as_ref()).as_bytes());
    debug!("hash_salt: {}${} => {}", password.as_ref(), salt.as_ref(), result);
    result
  }

  pub fn new(id: i64, password: String, conn: &mut Conn) -> Self {
    let result = secrets::table.select(UserPassword::as_select()).filter(secrets::id.eq(id)).get_result(conn);
    let salt = match result {
      Ok(Self { current, .. }) => current,
      _ => Self::gen_salt(),
    };
    let current = Self::hash_salt(password, &salt);
    Self {
      id, salt, current
    }
  }

  pub fn exec(self, conn: &mut Conn) -> anyhow::Result<()> {
    diesel::insert_into(secrets::table).values(&self).execute(conn)?;
    Ok(())
  }

  pub fn check(id: i64, password: &str, conn: &mut Conn) -> anyhow::Result<bool> {
    let profile: UserPassword = secrets::table.select(UserPassword::as_select()).filter(secrets::id.eq(id)).get_result(conn)?;
    Ok(Self::hash_salt(password, profile.salt) == profile.current)
  }
}

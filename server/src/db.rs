use diesel::RunQueryDsl;

use crate::schema::{users, posts};

pub type Conn = diesel::PgConnection;
pub type Pool<Conn=diesel::PgConnection> = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<Conn>>;

pub fn connect(url: &str) -> anyhow::Result<Pool> {
  let manager = diesel::r2d2::ConnectionManager::<diesel::PgConnection>::new(url);
  Ok(diesel::r2d2::Pool::builder().build(manager)?)
}

#[derive(Queryable, PartialEq, Debug, Selectable)]
pub struct User {
  pub id: i64,
}

#[derive(Queryable, PartialEq, Debug, Selectable, Insertable)]
#[table_name = "users"]
pub struct UserCreate {
  pub alias: String,
  pub name: String,
  pub description: Option<String>,
  pub avatar: Option<String>, // TODO: upload?
}

impl UserCreate {
  pub fn exec(self, conn: &mut Conn) -> anyhow::Result<()> {
    diesel::insert_into(users::table).values(&self).execute(conn)?;
    Ok(())
  }
}

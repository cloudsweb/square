#[macro_use] extern crate log;
#[macro_use] extern crate diesel;

pub type Conn = diesel::pg::PgConnection;

use diesel::prelude::*;
mod schema;
mod db;

const DATABASE_URL: &'static str = "postgresql://localhost:7039/posts";

fn main() {
  flexi_logger::Logger::try_with_env_or_str("info").unwrap().start().unwrap();
  info!("Hello, world!");
  let mut conn = Conn::establish(DATABASE_URL).unwrap();
  let users = schema::users::table.select(db::User::as_select()).load(&mut conn).unwrap();
  info!("users: {:?}", users);
}

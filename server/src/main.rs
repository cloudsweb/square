#[macro_use] extern crate log;
#[macro_use] extern crate async_trait;
#[macro_use] extern crate diesel;

// use diesel::prelude::*;
mod schema;
mod db;
mod auth;
mod rest;

const DATABASE_URL: &'static str = "postgresql://localhost:7039/posts";
const WEB_URL: &'static str = "127.0.0.1:7079";

fn main() {
  flexi_logger::Logger::try_with_env_or_str("info").unwrap().start().unwrap();
  info!("Hello, world!");
  let conn = db::connect(DATABASE_URL).expect("connect database");
  // let users = schema::users::table.select(db::User::as_select()).load(&mut conn).unwrap();
  // info!("users: {:?}", users);
  let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
  rt.block_on(async {
    rest::run(WEB_URL, conn.clone()).await
  }).expect("run error");
}

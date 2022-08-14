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
  #[cfg(debug_assertions)]
  flexi_logger::Logger::try_with_env_or_str("debug").unwrap().start().unwrap();
  #[cfg(not(debug_assertions))]
  flexi_logger::Logger::try_with_env_or_str("info").unwrap().start().unwrap();
  dotenv::dotenv().ok();
  let web_url = std::env::var("SQUARE_API_URL").unwrap_or_else(|_| WEB_URL.to_string());
  let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| DATABASE_URL.to_string());
  info!("Hello, world!");
  let conn = db::connect(&database_url).expect("connect database");
  // let users = schema::users::table.select(db::User::as_select()).load(&mut conn).unwrap();
  // info!("users: {:?}", users);
  let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
  rt.block_on(async {
    rest::run(&web_url, conn.clone()).await
  }).expect("run error");
}

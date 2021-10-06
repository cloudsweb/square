#[macro_use] extern crate log;
#[macro_use] extern crate diesel;

// use diesel::prelude::*;
mod schema;
mod db;
mod rest;

const DATABASE_URL: &'static str = "postgresql://localhost:7039/posts";
const WEB_URL: &'static str = "localhost:7079";

fn main() {
  flexi_logger::Logger::try_with_env_or_str("info").unwrap().start().unwrap();
  info!("Hello, world!");
  let conn = db::connect(DATABASE_URL).expect("connect database");
  // let users = schema::users::table.select(db::User::as_select()).load(&mut conn).unwrap();
  // info!("users: {:?}", users);
  actix_web::rt::System::new().block_on(async {
    rest::run(WEB_URL, conn.clone()).await
  }).expect("run error");
}

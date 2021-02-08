#[macro_use] extern crate log;

fn main() {
  flexi_logger::Logger::with_env_or_str("info").start().unwrap();
  info!("Hello, world!");
}

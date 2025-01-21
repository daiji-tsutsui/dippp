// Exercise according to
//   Dependency Injection Principles, Practices, and Patterns
// by Steven van Deursen and Mark Seemann

extern crate log;
extern crate env_logger as logger;

use log::info;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    logger::init();

    info!("Start");
}

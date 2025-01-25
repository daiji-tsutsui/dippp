// Exercise according to
//   Dependency Injection Principles, Practices, and Patterns
// by Steven van Deursen and Mark Seemann

extern crate env_logger as logger;
extern crate log;

mod model;
mod db_context;

use dotenv::dotenv;
#[allow(unused_imports)]
use log::{debug, info};

fn main() {
    dotenv().ok();
    logger::init();

    let context = db_context::CommerceContext::new();

    let product = context.fetch_one("name", "Black Thunder").unwrap();
    info!("fetched: {:#?}", product);

    let products = context.fetch("is_featured", "true");
    info!("fetched: {:#?}", products);
}

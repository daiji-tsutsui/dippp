// Exercise according to
//   Dependency Injection Principles, Practices, and Patterns
// by Steven van Deursen and Mark Seemann

extern crate env_logger as logger;
extern crate log;

mod model;

use dotenv::dotenv;
#[allow(unused_imports)]
use log::{debug, info};
use model::product;

fn main() {
    dotenv().ok();
    logger::init();

    let product = product::Product::fetch_one("name", "Black Thunder").unwrap();
    info!("fetched: {:#?}", product);

    let products = product::Product::fetch("is_featured", "true");
    info!("fetched: {:#?}", products);
}

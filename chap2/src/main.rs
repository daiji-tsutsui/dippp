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

    let mut product = product::Product::fetch_one("is_featured", "true").unwrap();
    info!("fetched: {:#?}", product);
    product.name = String::from("changed");
    info!("changed: {:#?}", product);

    let product2 = product::Product::fetch_one("is_featured", "true").unwrap();
    info!("fetched2: {:#?}", product2);

    let new_product = product::Product::new();
    info!("new: {:#?}", new_product);
}

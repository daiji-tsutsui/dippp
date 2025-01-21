// Exercise according to
//   Dependency Injection Principles, Practices, and Patterns
// by Steven van Deursen and Mark Seemann

extern crate log;
extern crate env_logger as logger;

mod model;

#[allow(unused_imports)]
use log::{ info, debug };
use dotenv::dotenv;
use model::product;

fn main() {
    dotenv().ok();
    logger::init();

    let mut product = product::Product::new();
    debug!("product: {:#?}", product);
    product.name = String::from("changed");
    debug!("modified product: {:#?}", product);
}

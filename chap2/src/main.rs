// Exercise according to
//   Dependency Injection Principles, Practices, and Patterns
// by Steven van Deursen and Mark Seemann

extern crate env_logger as logger;
extern crate getset;
extern crate log;

mod model;

use dotenv::dotenv;
#[allow(unused_imports)]
use log::{ debug, info };
use model::product;

fn main() {
    dotenv().ok();
    logger::init();

    let mut product = product::Product::fetch_one(
        "is_featured",
        model::DbValue::Bool(true),
    ).unwrap();
    debug!("fetched: {:#?}", product);
    product.set_name(String::from("test"));
    debug!("changed: {:#?}", product);

    let product2 = product::Product::fetch_one(
        "is_featured",
        model::DbValue::Bool(true),
    ).unwrap();
    debug!("fetched2: {:#?}", product2);

    let new_product = product::Product::new();
    debug!("new: {:#?}", new_product);
}

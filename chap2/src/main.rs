// Exercise according to
//   Dependency Injection Principles, Practices, and Patterns
// by Steven van Deursen and Mark Seemann

extern crate log;
extern crate env_logger as logger;
extern crate getset;

mod model;

#[allow(unused_imports)]
use log::{ info, debug };
use dotenv::dotenv;
use model::product;

fn main() {
    dotenv().ok();
    logger::init();

    let mut product = product::Product::fetch_one(
        "is_featured",
        model::DbValue::Bool(true),
    );
    debug!("fetched: {:#?}", product);
    product.set_name(String::from("test"));
    debug!("changed: {:#?}", product);

    let product2 = product::Product::fetch_one(
        "is_featured",
        model::DbValue::Bool(true),
    );
    debug!("fetched2: {:#?}", product2);
}

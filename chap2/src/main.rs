// Exercise according to
//   Dependency Injection Principles, Practices, and Patterns
// by Steven van Deursen and Mark Seemann

extern crate log;
extern crate env_logger as logger;
extern crate getset;
extern crate uuid;

mod product;

#[allow(unused_imports)]
use log::{ info, debug };
use dotenv::dotenv;
use product::Product;

fn main() {
    dotenv().ok();
    logger::init();

    let mut product = Product::new();

    debug!("name: {}", product.name());
    product.set_name(String::from("test"));
    debug!("name: {}", product.name());
    debug!("desc: {}", product.desc());
    debug!("id: {}", product.id());
}

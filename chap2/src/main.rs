// Exercise according to
//   Dependency Injection Principles, Practices, and Patterns
// by Steven van Deursen and Mark Seemann

extern crate env_logger as logger;
extern crate log;

mod controller;
mod db_context;
mod logic;
mod model;
mod view;

use dotenv::dotenv;
#[allow(unused_imports)]
use log::{debug, info};
use logic::product_service;

fn main() {
    dotenv().ok();
    logger::init();

    let service = product_service::ProductService::new();

    let is_customer_preferred1 = true;
    let products1 = service.get_featured_products(is_customer_preferred1);
    info!("fetched 1: {:#?}", products1);

    let is_customer_preferred2 = false;
    let products2 = service.get_featured_products(is_customer_preferred2);
    info!("fetched 2: {:#?}", products2);
}

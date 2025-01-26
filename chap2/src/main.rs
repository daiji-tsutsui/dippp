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

use controller::home::HomeController;
use dotenv::dotenv;
#[allow(unused_imports)]
use log::{debug, info};

fn main() {
    dotenv().ok();
    logger::init();

    let mut web = HomeController::new();
    web.login("PreferredCustomer");

    let view_result = web.index();
    info!("response: {:#?}", view_result);
}

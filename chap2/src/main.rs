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
use controller::home::HomeController;

fn main() {
    dotenv().ok();
    logger::init();

    let web = HomeController::new("PreferredCustomer");

    let view_result = web.index();
    info!("response: {:#?}", view_result);
}

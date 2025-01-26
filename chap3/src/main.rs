// Exercise according to
//   Dependency Injection Principles, Practices, and Patterns
// by Steven van Deursen and Mark Seemann

extern crate env_logger as logger;
extern crate log;

mod controller;
mod logic;
mod model;
mod view;
mod view_model;

use dotenv::dotenv;
#[allow(unused_imports)]
use log::{debug, info};

use crate::view::ViewResult;

fn main() {
    dotenv().ok();
    logger::init();

    let service = logic::product_service::ProductService::new();
    let web = controller::home::HomeController::new(service);

    let view = web.index();
    info!("Response HTML: {}", view.get_html());
}

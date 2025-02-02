// Exercise according to
//   Dependency Injection Principles, Practices, and Patterns
// by Steven van Deursen and Mark Seemann

extern crate env_logger as logger;
extern crate log;

mod db;
mod domain;
mod ui;

use dotenv::dotenv;
#[allow(unused_imports)]
use log::{debug, info};

use ui::view::ViewResult;

fn main() {
    dotenv().ok();
    logger::init();

    let mut web = ui::controller::home::HomeController::new();
    web.login("PreferredCustomer");

    let view_result = web.index();
    info!("Response HTML: {}", view_result.get_html());
}

// Exercise according to
//   Dependency Injection Principles, Practices, and Patterns
// by Steven van Deursen and Mark Seemann

extern crate env_logger as logger;
extern crate log;

mod controller;
mod db;
mod logic;
mod model;
mod view;
mod view_model;
mod web_context;

use dotenv::dotenv;
#[allow(unused_imports)]
use log::{debug, info};

use crate::view::ViewResult;

fn main() {
    dotenv().ok();
    logger::init();

    // オブジェクトグラフを見るため、あえてすべて入れ子にする
    let web = controller::home::HomeController::new(
        logic::product_service::ProductService::new(
            db::repository::sql_product_repository::SqlProductRepository::new(
                db::db_context::commerce_context::CommerceContext::new(),
            ),
            web_context::UserContextAdapter::new(),
        )
    );

    let view = web.index();
    info!("Response HTML: {}", view.get_html());
}

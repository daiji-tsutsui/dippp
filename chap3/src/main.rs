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

use crate::ui::view::ViewResult;

fn main() {
    dotenv().ok();
    logger::init();

    // オブジェクトグラフを見るため、あえてすべて入れ子にする
    let web = ui::controller::home::HomeController::new(
        domain::logic::product_service::ProductService::new(
            db::repository::sql_product_repository::SqlProductRepository::new(
                db::db_context::commerce_context::CommerceContext::new(),
            ),
            ui::web_context::UserContextAdapter::new(),
        )
    );

    let view = web.index();
    info!("Response HTML: {}", view.get_html());
}

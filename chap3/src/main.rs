// Exercise according to
//   Dependency Injection Principles, Practices, and Patterns
// by Steven van Deursen and Mark Seemann

extern crate env_logger as logger;
extern crate log;

mod model;
mod view;
mod view_model;

use dotenv::dotenv;
#[allow(unused_imports)]
use log::{debug, info};
use crate::view::ViewResult;
use crate::view::product_view;
use crate::view_model::{featured_products_view_model, product_view_model};
use crate::model::product;

fn main() {
    dotenv().ok();
    logger::init();

    let view = product_view::ProductView {
        model: featured_products_view_model::FeaturedProductsViewModel {
            products: vec![
                product_view_model::ProductViewModel {
                    data: product::Product::new(),
                },
                product_view_model::ProductViewModel {
                    data: product::Product::new(),
                },
            ],
        },
    };
    info!("Response HTML: {}", view.get_html());
}

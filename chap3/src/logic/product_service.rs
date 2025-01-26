use crate::logic::i_product_service::IProductService;
use crate::model::product;

pub struct ProductService {}

impl ProductService {
    pub fn new() -> Self {
        Self {}
    }
}

impl IProductService for ProductService {
    fn get_featured_products(&self) -> Vec<product::DiscountedProduct> {
        vec![
            product::DiscountedProduct {
                name: String::from("test1"),
                unit_price: 10.0,
            },
            product::DiscountedProduct {
                name: String::from("test2"),
                unit_price: 38.95,
            },
        ]
    }
}

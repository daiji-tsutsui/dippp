use crate::logic::i_product_service::IProductService;
use crate::model::product;

pub struct ProductService {}

impl ProductService {
    pub fn new() -> Self {
        Self {}
    }
}

impl IProductService for ProductService {
    fn get_featured_products(&self) -> Vec<product::Product> {
        vec![
            product::Product::new(),
            product::Product::new(),
        ]
    }
}

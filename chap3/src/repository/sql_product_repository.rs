// use crate::db_context::DbContext;
use crate::model::product;
use super::i_product_repository::IProductRepository;

pub struct SqlProductRepository {
    // context: DbContext,
}

impl IProductRepository for SqlProductRepository {
    fn get_featured_products(&self) -> Vec<product::Product> {
        vec![
            product::Product::new(),
        ]
    }
}

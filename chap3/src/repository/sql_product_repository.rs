use crate::db_context::DbContext;
use crate::model::product;
use super::i_product_repository::IProductRepository;

pub struct SqlProductRepository {
    context: DbContext,
}

impl SqlProductRepository {
    pub fn new(context: DbContext) -> Self {
        Self { context }
    }
}

impl IProductRepository for SqlProductRepository {
    fn get_featured_products(&self) -> Vec<product::Product> {
        self.context.fetch_products("is_featured", "true")
    }
}

use crate::db_context::commerce_context;
use crate::model::i_product_repository::IProductRepository;
use crate::model::product;

pub struct SqlProductRepository {
    context: commerce_context::CommerceContext,
}

impl SqlProductRepository {
    pub fn new(context: commerce_context::CommerceContext) -> Self {
        Self { context }
    }
}

impl IProductRepository for SqlProductRepository {
    fn get_featured_products(&self) -> Vec<product::Product> {
        self.context.fetch_products("is_featured", "true")
    }
}

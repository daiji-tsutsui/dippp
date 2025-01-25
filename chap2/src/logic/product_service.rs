use crate::db_context;
use crate::model::product;

pub struct ProductService {
    db_context: db_context::CommerceContext,
}

impl ProductService {
    pub fn new() -> Self {
        Self {
            db_context: db_context::CommerceContext::new(),
        }
    }

    pub fn get_featured_products(&self, is_customer_preferred: bool) -> Vec<product::Product> {
        let discount: f32 = match is_customer_preferred {
            true => 0.95,
            false => 1.0,
        };
        let products = self.db_context.fetch("is_featured", "true");
        products.iter().map(|p| Self::build_discounted_one(p, discount)).collect()
    }

    fn build_discounted_one(product: &product::Product, discount: f32) -> product::Product {
        let mut cloned = product.clone();
        let discounted_price: f32 = (product.unit_price as f32) * discount;
        cloned.unit_price = discounted_price.round() as i32;
        cloned
    }
}

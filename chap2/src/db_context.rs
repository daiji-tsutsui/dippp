use crate::model::product;

pub struct CommerceContext {}

impl CommerceContext {
    pub fn new() -> Self {
        Self {}
    }

    pub fn fetch(&self, field: &str, value: &str) -> Vec<product::Product> {
        product::Product::fetch(field, value)
    }

    pub fn fetch_one(&self, field: &str, value: &str) -> Option<product::Product> {
        product::Product::fetch_one(field, value)
    }
}

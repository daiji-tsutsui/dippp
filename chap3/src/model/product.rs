use super::*;

#[allow(dead_code)]
#[derive(Clone, Default, Debug)]
// Only necessary columns for our domain logic
pub struct Product {
    pub name: String,
    pub unit_price: f32,
    pub is_featured: bool,
}

impl Product {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Model for Product {}

#[derive(Clone, Debug)]
pub struct DiscountedProduct {
    pub name: String,
    pub unit_price: f32,
}

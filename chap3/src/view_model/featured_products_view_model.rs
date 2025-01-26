use super::product_view_model;
use crate::model::product;

#[derive(Debug)]
pub struct FeaturedProductsViewModel {
    pub products: Vec<product_view_model::ProductViewModel>,
}

impl FeaturedProductsViewModel {
    pub fn new(products: Vec<product::DiscountedProduct>) -> Self {
        Self {
            products: products.iter().cloned().map(|p| product_view_model::ProductViewModel::new(p)).collect(),
        }
    }
}
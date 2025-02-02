use crate::domain::model::product;

pub trait IProductService {
    fn get_featured_products(&self) -> Vec<product::DiscountedProduct>;
}

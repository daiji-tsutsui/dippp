use crate::domain::model::product;

pub trait IProductRepository {
    fn get_featured_products(&self) -> Vec<product::Product>;
}

use super::product_view_model;

#[derive(Debug)]
pub struct FeaturedProductsViewModel {
    pub products: Vec<product_view_model::ProductViewModel>,
}

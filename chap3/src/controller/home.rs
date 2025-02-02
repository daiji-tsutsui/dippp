use crate::domain::logic::i_product_service::IProductService;
use crate::view::product_view;
use crate::view_model::featured_products_view_model;

pub struct HomeController<T: IProductService> {
    service: T,
}

impl<T: IProductService> HomeController<T> {
    pub fn new(service: T) -> Self {
        Self { service }
    }

    pub fn index(&self) -> product_view::ProductView {
        let products = self.service.get_featured_products();

        product_view::ProductView {
            model: featured_products_view_model::FeaturedProductsViewModel::new(products),
        }
    }
}

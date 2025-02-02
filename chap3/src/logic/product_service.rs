use crate::logic::i_product_service::IProductService;
use crate::model::product;
use crate::model::i_user_context::IUserContext;
use crate::repository::i_product_repository::IProductRepository;

pub struct ProductService<T: IProductRepository, S: IUserContext> {
    repository: T,
    user_context: S,
}

impl<T: IProductRepository, S: IUserContext> ProductService<T, S> {
    pub fn new(repository: T, user_context: S) -> Self {
        Self {
            repository,
            user_context,
        }
    }
}

impl<T: IProductRepository, S: IUserContext> IProductService for ProductService<T, S> {
    fn get_featured_products(&self) -> Vec<product::DiscountedProduct> {
        let products = self.repository.get_featured_products();
        products
            .iter()
            .cloned()
            .map(|p| p.apply_discount_for(&self.user_context))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::i_user_context;

    #[test]
    fn test_get_featured_products_1() {
        let repository = MockProductRepository {};
        let user_context = MockUserContext {
            test_discount_flag: true,
        };
        let service = ProductService::new(repository, user_context);

        let result = service.get_featured_products();
        assert_eq!(result[0].unit_price, 9.5);
        assert_eq!(result[1].unit_price, 37.0025);
    }

    #[test]
    fn test_get_featured_products_2() {
        let repository = MockProductRepository {};
        let user_context = MockUserContext {
            test_discount_flag: false,
        };
        let service = ProductService::new(repository, user_context);

        let result = service.get_featured_products();
        assert_eq!(result[0].unit_price, 10.0);
        assert_eq!(result[1].unit_price, 38.95);
    }

    struct MockProductRepository {}

    impl IProductRepository for MockProductRepository {
        fn get_featured_products(&self) -> Vec<product::Product> {
            vec![
                product::Product {
                    name: String::from("test1"),
                    unit_price: 10.0,
                    is_featured: true,
                },
                product::Product {
                    name: String::from("test2"),
                    unit_price: 38.95,
                    is_featured: true,
                },
            ]
        }
    }

    struct MockUserContext {
        test_discount_flag: bool,
    }

    impl IUserContext for MockUserContext {
        fn is_in_role(&self, _role: i_user_context::Role) -> bool {
            self.test_discount_flag
        }
    }
}

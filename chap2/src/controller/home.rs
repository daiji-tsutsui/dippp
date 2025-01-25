use crate::logic::product_service;
use crate::model::product;
use crate::view;

pub struct HomeController {
    user: UserHttpContext,
}

impl HomeController {
    pub fn new(user_role: &str) -> Self {
        Self {
            user: UserHttpContext {
                role: String::from(user_role),
            },
        }
    }

    pub fn index(&self) -> view::ViewResult<product::Product> {
        let is_preferred_customer = self.user.is_in_role("PreferredCustomer");

        let service = product_service::ProductService::new();
        let products = service.get_featured_products(is_preferred_customer);

        view::ViewResult {
            view_data: products,
        }
    }
}

struct UserHttpContext {
    role: String,
}

impl UserHttpContext {
    fn is_in_role(&self, target_role: &str) -> bool {
        self.role == String::from(target_role)
    }
}

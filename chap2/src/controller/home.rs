use crate::controller;
use crate::logic::product_service;
use crate::model::product;
use crate::view;

pub struct HomeController {
    session: controller::HttpSession,
}

impl HomeController {
    pub fn new() -> Self {
        Self {
            session: controller::HttpSession {
                user: controller::UserHttpContext {
                    role: String::from("LoggedOut"),
                },
            },
        }
    }

    pub fn login(&mut self, user_role: &str) {
        self.session.user.role = String::from(user_role);
    }

    pub fn index(&self) -> view::ViewResult<product::Product> {
        let is_preferred_customer = self.session.user.is_in_role("PreferredCustomer");

        let service = product_service::ProductService::new();
        let products = service.get_featured_products(is_preferred_customer);

        view::ViewResult {
            view_data: products,
        }
    }
}

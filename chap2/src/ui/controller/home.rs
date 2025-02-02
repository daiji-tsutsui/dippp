use crate::ui::controller;
use crate::domain::logic::product_service;
use crate::ui::view;
use crate::ui::view::product_view;

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

    pub fn index(&self) -> product_view::ProductView {
        let is_preferred_customer = self.session.user.is_in_role("PreferredCustomer");

        let service = product_service::ProductService::new();
        let products = service.get_featured_products(is_preferred_customer);

        product_view::ProductView {
            view_data: view::ViewData { data: products },
        }
    }
}

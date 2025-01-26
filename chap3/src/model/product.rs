use super::*;
use super::i_user_context::IUserContext;

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

    pub fn apply_discount_for<T: IUserContext>(&self, user: &T) -> DiscountedProduct {
        let preferred: bool = user.is_in_role("PreferredCustomer");
        let discount: f32 = if preferred { 0.95 } else { 1.0 };
        DiscountedProduct {
            name: self.name.clone(),
            unit_price: self.unit_price * discount,
        }
    }
}

impl Model for Product {}

#[derive(Clone, Debug)]
pub struct DiscountedProduct {
    pub name: String,
    pub unit_price: f32,
}

#[cfg(test)]
include!("./product_test.rs");

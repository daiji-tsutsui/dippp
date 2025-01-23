use super::{ Model, DbValue };
use getset::{ Getters, Setters };

#[derive(Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Product {
    id: i32,
    name: String,
    desc: String,
    unit_price: i32,
    is_featured: bool,
}

impl Product {
    pub fn new() -> Self {
        Self {
            id: 1,
            name: String::from("name"),
            desc: String::from("desc"),
            unit_price: 100,
            is_featured: true,
        }
    }
}

impl Model for Product {}

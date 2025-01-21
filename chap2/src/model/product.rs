use super::{ Model };

#[derive(Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub unit_price: i32,
    pub is_featured: bool,
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

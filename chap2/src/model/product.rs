use super::Model;
use getset::{ Getters, Setters };
use uuid::{ uuid, Uuid };

#[derive(Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Product {
    id: Uuid,
    name: String,
    desc: String,
    unit_price: i32,
    is_featured: bool,
}

impl Product {
    pub fn new() -> Self {
        Self {
            id: uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"),
            name: String::from("name"),
            desc: String::from("desc"),
            unit_price: 100,
            is_featured: true,
        }
    }
}

impl Model for Product {}

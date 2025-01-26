use super::*;

#[allow(dead_code)]
#[derive(Clone, Default, Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub unit_price: f32,
    pub is_featured: bool,
}

impl Product {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Default::default()
    }
}

impl Model for Product {}

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

    pub fn fetch(field: &str, value: DbValue) -> Self {
        Self::new()
    }

    fn getter(&self, field: &str) -> DbValue {
        match field {
            "id" => DbValue::Int(self.id().clone()),
            "name" => DbValue::Str(self.name().clone()),
            "desc" => DbValue::Str(self.desc().clone()),
            "unit_price" => DbValue::Int(self.unit_price().clone()),
            "is_featured" => DbValue::Bool(self.is_featured().clone()),
            _ => panic!("Invalid field name!!"),
        }
    }
}

impl Model for Product {}

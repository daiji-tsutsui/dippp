use super::{ Model, DbValue };
use std::sync::{ Mutex, LazyLock };
use getset::{ Getters, Setters };

#[derive(Getters, Setters, Clone, Debug)]
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
            id: Self::current_id(),
            name: String::from(""),
            desc: String::from(""),
            unit_price: 0,
            is_featured: false,
        }
    }

    pub fn fetch_one(field: &str, value: DbValue) -> Self {
        let table = PRODUCT_TABLE.lock().unwrap();
        for record in table.iter() {
            if value == record.getter(field) {
                return record.clone();
            }
        }
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

    fn current_id() -> i32 {
        let table = PRODUCT_TABLE.lock().unwrap();
        table.len() as i32 + 1
    }
}

impl Model for Product {}

static PRODUCT_TABLE: LazyLock<Mutex<Vec<Product>>> = LazyLock::new(||
    Mutex::new(
        vec![
            Product {
                id: 1,
                name: String::from("Black Thunder"),
                desc: String::from("Chocolate Snack"),
                unit_price: 40,
                is_featured: false,
            },
            Product {
                id: 2,
                name: String::from("Orange"),
                desc: String::from("Organic"),
                unit_price: 100,
                is_featured: true,
            },
        ]
    )
);

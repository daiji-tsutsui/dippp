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
        let new_id = &PRODUCT_TABLE.lock().unwrap().current_id() + 1;
        Self {
            id: new_id,
            name: String::from(""),
            desc: String::from(""),
            unit_price: 0,
            is_featured: false,
        }
    }

    pub fn fetch_one(field: &str, value: DbValue) -> Self {
        let table_content = &PRODUCT_TABLE.lock().unwrap().table;
        for record in table_content.iter() {
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
}

impl Model for Product {}

struct ProductTable {
    pub table: Vec<Product>,
}

impl ProductTable {
    pub fn current_id(&self) -> i32 {
        self.table.len() as i32
    }
}

static PRODUCT_TABLE: LazyLock<Mutex<ProductTable>> = LazyLock::new(||
    Mutex::new(
        ProductTable {
            table: vec![
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
        }
    )
);

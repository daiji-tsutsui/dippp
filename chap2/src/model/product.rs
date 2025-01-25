use super::{ DbValue, Model };
use getset::{ Getters, Setters };
use std::sync::{ LazyLock, Mutex };

#[derive(Getters, Setters, Clone, Default, Debug)]
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
            ..Default::default()
        }
    }

    pub fn fetch(field: &str, value: DbValue) -> Vec<Self> {
        let table = &PRODUCT_TABLE.lock().unwrap().table;
        table
            .iter()
            .cloned()
            .filter(|record: &Self| record.getter(field) == value)
            .collect()
    }

    pub fn fetch_one(field: &str, value: DbValue) -> Option<Self> {
        let fetched = Self::fetch(field, value);
        match fetched.len() > 0 {
            true => Some(fetched[0].clone()),
            false => None,
        }
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

static PRODUCT_TABLE: LazyLock<Mutex<ProductTable>> = LazyLock::new(|| {
    Mutex::new(ProductTable {
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
        ],
    })
});

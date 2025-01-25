use super::{ Model };
use std::sync::{ LazyLock, Mutex };

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

struct ProductTable {
    pub data_json: String,
}

impl ProductTable {
    pub fn new(data_json: &str) -> Self {
        Self {
            data_json: data_json.to_string(),
        }
    }
}

static DEFAULT_PRODUCTS: &str = r#"[
    {
        "id": 1,
        "name": "Black Thunder",
        "desc": "Chocolate Snack",
        "unit_price": 40,
        "is_featured": false,
    },
    {
        "id": 2,
        "name": "Orange",
        "desc": "Organic",
        "unit_price": 100,
        "is_featured": true,
    }
]"#;
static PRODUCT_TABLE: LazyLock<Mutex<ProductTable>> = LazyLock::new(||
    Mutex::new(
        ProductTable::new(DEFAULT_PRODUCTS)
    )
);

use std::error::Error;
use std::sync::{LazyLock, Mutex};

use serde_json;

use super::Model;

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

    pub fn query_select(&self, field: &str, value: &str) -> Vec<Product> {
        let data: Vec<serde_json::Value> = self.get_data().unwrap();
        let filtered: Vec<serde_json::Value> =
            data.iter().cloned().filter(|record| record[field] == value).collect();
        filtered.iter().map(|record| build_from_json(record)).collect()
    }

    fn build_from_json(record: &&serde_json::Value) -> Product {
        Product {
            id: record["id"].as_i64().unwrap() as i32,
            name: record["name"].to_string(),
            desc: record["desc"].to_string(),
            unit_price: record["unit_price"].as_i64().unwrap() as i32,
            is_featured: record["is_featured"].as_bool().unwrap(),
        }
    }

    fn get_data(&self) -> Result<Vec<serde_json::Value>, Box<dyn Error>> {
        let v: serde_json::Value = serde_json::from_str(&self.data_json)?;
        let result = v["data"].as_array().unwrap();
        Ok(result.to_vec())
    }
}

static DEFAULT_PRODUCTS: &str = r#"
{
    "data": [
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
    ]
}
"#;
static PRODUCT_TABLE: LazyLock<Mutex<ProductTable>> =
    LazyLock::new(|| Mutex::new(ProductTable::new(DEFAULT_PRODUCTS)));

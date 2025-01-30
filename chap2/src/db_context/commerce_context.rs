use std::error::Error;
use std::sync::{LazyLock, Mutex};

use log::debug;
use serde_json;

use super::{DbContext, MockDbTable, MockDbTableData};
use crate::model::product::Product;

pub struct CommerceContext {}

impl CommerceContext {
    pub fn new() -> Self {
        Self {}
    }

    pub fn fetch_products(&self, field: &str, value: &str) -> Vec<Product> {
        let table = &PRODUCT_TABLE.lock().unwrap();
        table.query_select(field, value)
    }

    #[allow(dead_code)]
    pub fn fetch_product(&self, field: &str, value: &str) -> Option<Product> {
        let fetched = self.fetch_products(field, value);
        match fetched.len() > 0 {
            true => Some(fetched[0].clone()),
            false => None,
        }
    }
}

impl DbContext for CommerceContext {}

struct ProductTable {
    pub data: MockDbTableData,
}

impl ProductTable {
    pub fn new(data_json: &str) -> Self {
        Self {
            data: MockDbTableData::new(data_json),
        }
    }
}

impl MockDbTable<Product> for ProductTable {
    fn get_data(&self) -> Result<Vec<serde_json::Value>, Box<dyn Error>> {
        self.data.get_data()
    }

    fn build_from_json(record: &serde_json::Value) -> Product {
        debug!("Building: {:#?}", record);
        Product {
            id: record["id"].as_i64().unwrap() as i32,
            name: record["name"].as_str().unwrap().to_string(),
            desc: record["desc"].as_str().unwrap().to_string(),
            unit_price: record["unit_price"].as_f64().unwrap() as f32,
            is_featured: record["is_featured"].as_bool().unwrap(),
        }
    }
}

static DEFAULT_PRODUCTS: &str = r#"
{
    "data": [
        {
            "id": 1,
            "name": "Black Thunder",
            "desc": "Chocolate Snack",
            "unit_price": 40.0,
            "is_featured": false
        },
        {
            "id": 2,
            "name": "Orange",
            "desc": "Organic",
            "unit_price": 100.0,
            "is_featured": true
        }
    ]
}
"#;
static PRODUCT_TABLE: LazyLock<Mutex<ProductTable>> =
    LazyLock::new(|| Mutex::new(ProductTable::new(DEFAULT_PRODUCTS)));

#[cfg(test)]
include!("./commerce_context_test.rs");

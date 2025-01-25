use std::error::Error;
use std::sync::{LazyLock, Mutex};

use log::debug;
use serde_json;

use super::{MockDbTable, MockDbTableData, Model};

#[allow(dead_code)]
#[derive(Clone, Default, Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub unit_price: i32,
    pub is_featured: bool,
}

impl Product {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Default::default()
    }

    pub fn fetch(field: &str, value: &str) -> Vec<Self> {
        let table = &PRODUCT_TABLE.lock().unwrap();
        table.query_select(field, value)
    }

    #[allow(dead_code)]
    pub fn fetch_one(field: &str, value: &str) -> Option<Self> {
        let fetched = Self::fetch(field, value);
        match fetched.len() > 0 {
            true => Some(fetched[0].clone()),
            false => None,
        }
    }
}

impl Model for Product {}

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
            unit_price: record["unit_price"].as_i64().unwrap() as i32,
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
            "unit_price": 40,
            "is_featured": false
        },
        {
            "id": 2,
            "name": "Orange",
            "desc": "Organic",
            "unit_price": 100,
            "is_featured": true
        }
    ]
}
"#;
static PRODUCT_TABLE: LazyLock<Mutex<ProductTable>> =
    LazyLock::new(|| Mutex::new(ProductTable::new(DEFAULT_PRODUCTS)));

#[cfg(test)]
include!("./product_test.rs");

use std::error::Error;
use std::sync::{LazyLock, Mutex};

use log::debug;
use serde_json;

use crate::model::product::Product;

pub struct DbContext {}

impl DbContext {
    pub fn fetch_product(field: &str, value: &str) -> Vec<Product> {
        let table = &PRODUCT_TABLE.lock().unwrap();
        table.query_select(field, value)
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

struct ProductTable {
    pub data: MockDbTableData,
}

impl ProductTable {
    pub fn new(data_json: &str) -> Self {
        Self {
            data: MockDbTableData::new(data_json),
        }
    }

    fn get_data(&self) -> Result<Vec<serde_json::Value>, Box<dyn Error>> {
        self.data.get_data()
    }

    fn build_from_json(record: &serde_json::Value) -> Product {
        debug!("Building: {:#?}", record);
        Product {
            name: record["name"].as_str().unwrap().to_string(),
            unit_price: record["unit_price"].as_f64().unwrap() as f32,
            is_featured: record["is_featured"].as_bool().unwrap(),
        }
    }

    fn query_select(&self, field: &str, value: &str) -> Vec<Product> {
        let data: Vec<serde_json::Value> = self.get_data().unwrap();
        let filtered: Vec<serde_json::Value> = data
            .iter()
            .cloned()
            .filter(|record| {
                record[field].is_string() && value == record[field].as_str().unwrap()
                    || value == record[field].to_string()
            })
            .collect();
        filtered.iter().map(|record| Self::build_from_json(record)).collect()
    }
}

struct MockDbTableData {
    pub data_json: String,
}

impl MockDbTableData {
    pub fn new(data_json: &str) -> Self {
        Self {
            data_json: data_json.to_string(),
        }
    }

    pub fn get_data(&self) -> Result<Vec<serde_json::Value>, Box<dyn Error>> {
        let v: serde_json::Value = serde_json::from_str(&self.data_json)?;
        let result = v["data"].as_array().unwrap();
        Ok(result.to_vec())
    }
}

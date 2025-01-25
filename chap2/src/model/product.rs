use std::error::Error;
use std::sync::{LazyLock, Mutex};

use log::debug;
use serde_json;

use super::Model;

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
    pub fn new() -> Self {
        Default::default()
    }

    pub fn fetch(field: &str, value: &str) -> Vec<Self> {
        let table = &PRODUCT_TABLE.lock().unwrap();
        table.query_select(field, value)
    }

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
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let new_product = Product::new();
        assert_eq!(new_product.id, 0);
        assert_eq!(new_product.name, "");
        assert_eq!(new_product.unit_price, 0);
        assert_eq!(new_product.is_featured, false);
    }

    #[test]
    fn test_fetch() {
        let products = Product::fetch("name", "Black Thunder");
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].id, 1);
    }

    #[test]
    fn test_fetch_one_1() {
        let product = Product::fetch_one("is_featured", "true").unwrap();
        assert_eq!(product.id, 2);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_fetch_one_2() {
        let _product = Product::fetch_one("name", "Non-exists").unwrap();
    }

    #[test]
    fn test_fetch_one_does_not_change_master_record() {
        let mut product = Product::fetch_one("id", "1").unwrap();
        product.name = String::from("changed");
        assert_eq!(product.name, "changed");

        let product_again = Product::fetch_one("id", "1").unwrap();
        assert_eq!(product_again.name, "Black Thunder");
    }
}

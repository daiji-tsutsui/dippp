pub mod product;

use std::error::Error;

#[allow(dead_code)]
pub trait Model {}

pub trait MockDbTable<T> {
    fn get_data(&self) -> Result<Vec<serde_json::Value>, Box<dyn Error>>;
    fn build_from_json(record: &serde_json::Value) -> T;

    fn query_select(&self, field: &str, value: &str) -> Vec<T> {
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

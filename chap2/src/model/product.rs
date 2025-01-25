use super::{ DbValue, Model };
use std::sync::{ LazyLock, Mutex };

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

    pub fn fetch(field: &str, value: DbValue) -> Vec<Self> {
        let table = &PRODUCT_TABLE.lock().unwrap().table;
        table.iter()
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
            "id" => DbValue::Int(self.id.clone()),
            "name" => DbValue::Str(self.name.clone()),
            "desc" => DbValue::Str(self.desc.clone()),
            "unit_price" => DbValue::Int(self.unit_price.clone()),
            "is_featured" => DbValue::Bool(self.is_featured.clone()),
            _ => panic!("Invalid field name!!"),
        }
    }
}

#[allow(dead_code)]
impl Model for Product {}

struct ProductTable {
    pub table: Vec<Product>,
}

impl ProductTable {
    #[allow(dead_code)]
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ DbValue };

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
        let str_black_thunder = DbValue::Str(String::from("Black Thunder"));
        let products = Product::fetch("name", str_black_thunder);
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].id, 1);
    }

    #[test]
    fn test_fetch_one_1() {
        let product = Product::fetch_one("is_featured", DbValue::Bool(true)).unwrap();
        assert_eq!(product.id, 2);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_fetch_one_2() {
        let str_non_exists = DbValue::Str(String::from("Non-exists"));
        let _product = Product::fetch_one("name", str_non_exists).unwrap();
    }
}

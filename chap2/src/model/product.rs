use super::Model;

#[allow(dead_code)]
#[derive(Clone, Default, Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub unit_price: f32,
    pub is_featured: bool,
}

impl Product {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Default::default()
    }
}

impl Model for Product {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let new_product = Product::new();
        assert_eq!(new_product.id, 0);
        assert_eq!(new_product.name, "");
        assert_eq!(new_product.unit_price, 0.0);
        assert_eq!(new_product.is_featured, false);
    }
}

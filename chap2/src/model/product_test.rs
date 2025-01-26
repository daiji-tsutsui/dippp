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

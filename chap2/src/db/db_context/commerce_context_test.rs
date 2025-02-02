#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_products() {
        let context = CommerceContext::new();
        let products = context.fetch_products("name", "Black Thunder");
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].id, 1);
    }

    #[test]
    fn test_fetch_product_1() {
        let context = CommerceContext::new();
        let product = context.fetch_product("is_featured", "true").unwrap();
        assert_eq!(product.id, 2);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_fetch_product_2() {
        let context = CommerceContext::new();
        let _product = context.fetch_product("name", "Non-exists").unwrap();
    }

    #[test]
    fn test_fetch_product_does_not_change_master_record() {
        let context = CommerceContext::new();
        let mut product = context.fetch_product("id", "1").unwrap();
        product.name = String::from("changed");
        assert_eq!(product.name, "changed");

        let product_again = context.fetch_product("id", "1").unwrap();
        assert_eq!(product_again.name, "Black Thunder");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let new_product = Product::new();
        assert_eq!(new_product.name, "");
        assert_eq!(new_product.unit_price, 0.0);
        assert_eq!(new_product.is_featured, false);
    }

    #[test]
    fn test_apply_discount_for_1() {
        let product = Product {
            name: String::from("Test1"),
            unit_price: 10.0,
            is_featured: true,
        };
        let user_context = MockUserContext { test_discount_flag: true };

        let result = product.apply_discount_for(&user_context);
        assert_eq!(result.name, "Test1");
        assert_eq!(result.unit_price, 9.5);
    }

    #[test]
    fn test_apply_discount_for_2() {
        let product = Product {
            name: String::from("Test2"),
            unit_price: 10.0,
            is_featured: false,
        };
        let user_context = MockUserContext { test_discount_flag: false };

        let result = product.apply_discount_for(&user_context);
        assert_eq!(result.name, "Test2");
        assert_eq!(result.unit_price, 10.0);
    }

    struct MockUserContext {
        test_discount_flag: bool,
    }

    impl IUserContext for MockUserContext {
        fn is_in_role(&self, _role: &str) -> bool {
            self.test_discount_flag
        }
    }
}

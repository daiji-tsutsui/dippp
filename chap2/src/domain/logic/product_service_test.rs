#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_featured_products_1() {
        let service = ProductService::new();
        let is_customer_preferred = true;

        let products = service.get_featured_products(is_customer_preferred);
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].unit_price, 95.0);
    }

    #[test]
    fn test_get_featured_products_2() {
        let service = ProductService::new();
        let is_customer_preferred = false;

        let products = service.get_featured_products(is_customer_preferred);
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].unit_price, 100.0);
    }
}

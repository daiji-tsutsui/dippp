use crate::domain::model::product;

#[derive(Clone, Debug)]
pub struct ProductViewModel {
    pub data: product::DiscountedProduct,
}

impl ProductViewModel {
    pub fn new(data: product::DiscountedProduct) -> Self {
        Self { data }
    }

    pub fn summary_text(&self) -> String {
        format!("{} (${:.2})", self.data.name, self.data.unit_price)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summary_text() {
        let view_model = ProductViewModel {
            data: product::DiscountedProduct {
                name: String::from("test"),
                unit_price: 200.0,
            },
        };
        assert_eq!(view_model.summary_text(), "test ($200.00)");
    }
}

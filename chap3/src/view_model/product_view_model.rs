use crate::model::product;

#[derive(Clone, Debug)]
pub struct ProductViewModel {
    pub data: product::Product,
}

impl ProductViewModel {
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
            data: product::Product {
                name: String::from("test"),
                unit_price: 200.0,
                ..Default::default()
            },
        };
        assert_eq!(view_model.summary_text(), "test ($200.00)");
    }
}

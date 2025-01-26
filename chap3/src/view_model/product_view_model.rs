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

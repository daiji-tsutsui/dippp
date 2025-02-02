use crate::ui::view::*;
use crate::ui::view_model::featured_products_view_model;

#[derive(Debug)]
pub struct ProductView {
    pub model: featured_products_view_model::FeaturedProductsViewModel,
}

impl ViewResult for ProductView {
    fn get_html(&self) -> String {
        macro_rules! template {
            () => {
                "
<h2>Featured Products</h2>
<div>{list}</div>
"
            };
        }

        let products = self.model.products.clone();
        let list = products.iter().fold("\n".to_string(), |acc, p| {
            format!("{}\t<div>{}</div>\n", acc, p.summary_text())
        });
        let result = format!(template!(), list = list);
        result.to_string()
    }
}

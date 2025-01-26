use crate::model::product;
use crate::view;

#[derive(Debug)]
pub struct ProductView {
    pub view_data: view::ViewData<product::Product>,
}

impl view::ViewResult for ProductView {
    fn get_html(&self) -> String {
        macro_rules! template {
            () => {
                "
<h2>Featured Products</h2>
<div>{list}</div>
"
            };
        }

        let products = self.view_data.data.clone();
        let list = products.iter().fold("\n".to_string(), |acc, p| {
            format!("{}\t<div>{} ({})</div>\n", acc, p.name, p.unit_price)
        });
        let result = format!(template!(), list = list);
        result.to_string()
    }
}

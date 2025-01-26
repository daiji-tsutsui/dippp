use crate::model::product;
use crate::view;

#[derive(Debug)]
pub struct ProductView {
    pub view_data: view::ViewData<product::Product>,
}

impl view::ViewResult for ProductView {
    fn get_html() -> String {
        let result = "";
        result.to_string()
    }
}

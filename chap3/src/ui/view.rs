pub mod product_view;

pub trait ViewResult {
    fn get_html(&self) -> String;
}

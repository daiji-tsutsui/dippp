pub mod product_view;

#[allow(dead_code)]
pub trait ViewResult {
    fn get_html() -> String;
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ViewData<T> {
    pub data: Vec<T>,
}

pub mod product;

#[derive(PartialEq)]
pub enum DbValue {
    Str(String),
    Int(i32),
    Bool(bool),
}

pub trait Model {}

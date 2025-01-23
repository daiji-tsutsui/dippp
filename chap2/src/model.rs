pub mod product;

pub enum DbValue {
    Str(String),
    Int(i32),
    Bool(bool),
}

pub trait Model {}

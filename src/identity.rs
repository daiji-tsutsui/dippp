pub mod dummy;

pub trait Identity {
    fn is_authenticated(&self) -> bool;
}

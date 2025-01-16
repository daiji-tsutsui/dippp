pub mod console;
pub mod secure;

pub trait MessageWriter {
    fn write(&self, message: String);
}

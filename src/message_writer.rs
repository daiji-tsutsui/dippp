pub mod console;
pub mod secure;

pub trait MessageWriter {
    fn write(&mut self, message: String);
}

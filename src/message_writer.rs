pub mod console;
pub mod secure;

use mockall::*;
use mockall::predicate::*;

#[automock]
pub trait MessageWriter {
    fn write(&mut self, message: String);
}

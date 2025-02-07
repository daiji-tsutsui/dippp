pub mod console;
pub mod secure;

use mockall::predicate::*;
use mockall::*;

#[automock]
pub trait MessageWriter {
    fn write(&self, message: String);
}

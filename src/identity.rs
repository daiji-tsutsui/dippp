pub mod dummy;

use mockall::*;
use mockall::predicate::*;

#[automock]
pub trait Identity {
    fn is_authenticated(&self) -> bool;
}

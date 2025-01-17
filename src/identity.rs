pub mod dummy;

use mockall::predicate::*;
use mockall::*;

#[automock]
pub trait Identity {
    fn is_authenticated(&self) -> bool;
}

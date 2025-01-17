use super::{ Identity };
use log::{ debug };

pub struct Dummy { }

impl Dummy {
    pub fn new() -> Self {
        Self { }
    }
}

impl Identity for Dummy {
    fn is_authenticated(&self) -> bool {
        debug!("Authorized!");
        true
    }
}

use super::{ Identity };

pub struct Dummy { }

impl Dummy {
    pub fn new() -> Self {
        Self { }
    }
}

impl Identity for Dummy {
    fn is_authenticated(&self) -> bool {
        println!("[DEBUG] Dummy authorization!");
        true
    }
}

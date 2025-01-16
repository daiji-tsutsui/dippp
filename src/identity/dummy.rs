use super::{ Identity };

pub struct DummyIdentity { }

impl DummyIdentity {
    pub fn new() -> Self {
        Self { }
    }
}

impl Identity for DummyIdentity {
    fn is_authenticated(&self) -> bool {
        println!("[DEBUG] Dummy authorization!");
        true
    }
}

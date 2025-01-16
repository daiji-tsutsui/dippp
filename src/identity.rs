pub trait Identity {
    fn is_authenticated(&self) -> bool;
}

pub struct DummyIdentity { }

impl Identity for DummyIdentity {
    fn is_authenticated(&self) -> bool {
        println!("[DEBUG] Dummy authorization!");
        true
    }
}

impl DummyIdentity {
    pub fn new() -> Self {
        Self { }
    }
}

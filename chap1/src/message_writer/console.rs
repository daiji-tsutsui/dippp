use super::{ MessageWriter };

pub struct Console {}

impl Console {
    pub fn new() -> Self {
        Self {}
    }
}

impl MessageWriter for Console {
    fn write(&self, message: String) {
        println!("{}", message);
    }
}

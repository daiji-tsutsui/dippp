use super::{ MessageWriter };

pub struct ConsoleMessageWriter { }

impl ConsoleMessageWriter {
    pub fn new() -> Self {
        Self { }
    }
}

impl MessageWriter for ConsoleMessageWriter {
    fn write(&self, message: String) {
        println!("{}", message);
    }
}

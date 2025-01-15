pub trait MessageWriter {
    fn write(&self, message: String);
}

pub struct ConsoleMessageWriter { }

impl MessageWriter for ConsoleMessageWriter {
    fn write(&self, message: String) {
        println!("{}", message);
    }
}

impl ConsoleMessageWriter {
    pub fn new() -> Self {
        Self { }
    }
}

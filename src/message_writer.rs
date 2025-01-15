pub struct ConsoleMessageWriter { }

impl ConsoleMessageWriter {
    pub fn new() -> Self {
        Self { }
    }

    pub fn write(&self, message: String) {
        println!("{}", message);
    }
}

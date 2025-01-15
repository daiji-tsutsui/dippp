pub struct ConsoleMessageWriter { }

impl ConsoleMessageWriter {
    pub fn write(&self, message: String) {
        println!("{}", message);
    }
}

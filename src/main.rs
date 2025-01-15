struct ConsoleMessageWriter { }

impl ConsoleMessageWriter {
    fn write(&self, message: String) {
        println!("{}", message);
    }
}

fn main() {
    let writer = ConsoleMessageWriter { };
    let message = String::from("Hello, DI!");
    writer.write(message);
}

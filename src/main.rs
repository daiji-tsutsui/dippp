struct ConsoleMessageWriter { }

impl ConsoleMessageWriter {
    fn write(&self, message: String) {
        println!("{}", message);
    }
}

struct Salutation {
    writer: ConsoleMessageWriter,
}

impl Salutation {
    fn exclaim(&self) {
        let message = String::from("Hello, DI!");
        self.writer.write(message);
    }
}

fn main() {
    let writer = ConsoleMessageWriter { };
    let salute = Salutation { writer };
    salute.exclaim();
}

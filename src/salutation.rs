use crate::message_writer::{ ConsoleMessageWriter };

pub struct Salutation {
    writer: ConsoleMessageWriter,
}

impl Salutation {
    pub fn new(writer: ConsoleMessageWriter) -> Self {
        Self { writer }
    }

    pub fn exclaim(&self) {
        let message = String::from("Hello, DI!");
        self.writer.write(message);
    }
}

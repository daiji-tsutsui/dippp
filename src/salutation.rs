use crate::message_writer::{ ConsoleMessageWriter };

pub struct Salutation {
    pub writer: ConsoleMessageWriter,
}

impl Salutation {
    pub fn exclaim(&self) {
        let message = String::from("Hello, DI!");
        self.writer.write(message);
    }
}

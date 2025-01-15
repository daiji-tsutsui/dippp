use crate::message_writer::{ MessageWriter };

pub struct Salutation<T: MessageWriter> {
    writer: T,
}

impl<T: MessageWriter> Salutation<T> {
    pub fn new(writer: T) -> Self {
        Self { writer }
    }

    pub fn exclaim(&self) {
        let message = String::from("Hello, DI!");
        self.writer.write(message);
    }
}

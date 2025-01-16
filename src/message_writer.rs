use crate::identity::{ Identity };

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

pub struct SecureMessageWriter<T: MessageWriter, S: Identity> {
    writer: T,
    identity: S,
}

impl<T: MessageWriter, S: Identity> MessageWriter for SecureMessageWriter<T, S> {
    fn write(&self, message: String) {
        if self.identity.is_authenticated() {
            self.writer.write(message);
        }
    }
}

impl<T: MessageWriter, S: Identity> SecureMessageWriter<T, S> {
    pub fn new(writer: T, identity: S) -> Self {
        Self { writer, identity }
    }
}

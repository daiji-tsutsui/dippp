use super::{ MessageWriter };
use crate::identity::{ Identity };

pub struct Secure<T: MessageWriter, S: Identity> {
    writer: T,
    identity: S,
}

impl<T: MessageWriter, S: Identity> Secure<T, S> {
    pub fn new(writer: T, identity: S) -> Self {
        Self { writer, identity }
    }
}

impl<T: MessageWriter, S: Identity> MessageWriter for Secure<T, S> {
    fn write(&self, message: String) {
        if self.identity.is_authenticated() {
            self.writer.write(message);
        }
    }
}

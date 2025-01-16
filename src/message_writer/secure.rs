use super::{ MessageWriter };
use crate::identity::{ Identity };

pub struct SecureMessageWriter<T: MessageWriter, S: Identity> {
    writer: T,
    identity: S,
}

impl<T: MessageWriter, S: Identity> SecureMessageWriter<T, S> {
    pub fn new(writer: T, identity: S) -> Self {
        Self { writer, identity }
    }
}

impl<T: MessageWriter, S: Identity> MessageWriter for SecureMessageWriter<T, S> {
    fn write(&self, message: String) {
        if self.identity.is_authenticated() {
            self.writer.write(message);
        }
    }
}

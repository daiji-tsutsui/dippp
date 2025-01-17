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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::{ MockIdentity };
    use crate::message_writer::{ MockMessageWriter };

    #[test]
    fn test_write() {
        let mut mock_identity = MockIdentity::new();
        mock_identity
            .expect_is_authenticated()
            .with()
            .times(1)
            .return_const(true);

        let mut mock_writer = MockMessageWriter::new();
        mock_writer
            .expect_write()
            .times(1)
            .returning(|msg| assert_eq!(String::from("This is a test"), msg));

        let writer = Secure::new(mock_writer, mock_identity);
        writer.write(String::from("This is a test"));
    }
}

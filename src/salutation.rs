use crate::message_writer::{ MessageWriter };

pub struct Salutation<T: MessageWriter> {
    writer: T,
}

impl<T: MessageWriter> Salutation<T> {
    pub fn new(writer: T) -> Self {
        Self { writer }
    }

    pub fn exclaim(&mut self) {
        let message = String::from("Hello, DI!");
        self.writer.write(message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message_writer::{ MockMessageWriter };

    #[test]
    fn test_exclaim() {
        let mut mock_writer = MockMessageWriter::new();
        mock_writer
            .expect_write()
            .times(1)
            .returning(|msg| assert_eq!(String::from("Hello, DI!"), msg));

        let mut salute = Salutation::new(mock_writer);
        salute.exclaim()
    }
}

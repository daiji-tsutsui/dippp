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

    #[test]
    fn test_exclaim() {
        let writer = StubWriter::new();
        let mut salute = Salutation::new(writer);
        salute.exclaim();
        assert_eq!(salute.get_writer().written.len(), 1);
        assert_eq!(salute.get_writer().written[0], "Hello, DI!");
    }

    #[derive(Clone)]
    struct StubWriter {
        pub written: Vec<String>,
    }

    impl MessageWriter for StubWriter {
        fn write(&mut self, message: String) {
            self.written.push(message);
        }
    }

    impl StubWriter {
        pub fn new() -> Self {
            Self { written: Vec::<String>::new() }
        }
    }

    impl<T: MessageWriter + Clone> Salutation<T> {
        pub fn get_writer(&self) -> T {
            self.writer.clone()
        }
    }
}

// Exercise according to
//   Dependency Injection Principles, Practices, and Patterns
// by Steven van Deursen and Mark Seemann

extern crate log;
extern crate simple_logger as logger;

mod identity;
mod message_writer;
mod salutation;

fn main() {
    logger::init().unwrap();

    let identity = identity::dummy::Dummy::new();
    let writer = message_writer::secure::Secure::new(
        message_writer::console::Console::new(),
        identity,
    );
    let salute = salutation::Salutation::new(writer);
    salute.exclaim();
}

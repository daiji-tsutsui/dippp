// Exercise according to
//   Dependency Injection Principles, Practices, and Patterns
// by Steven van Deursen and Mark Seemann

mod message_writer;
mod salutation;
mod identity;

fn main() {
    let identity = identity::dummy::Dummy::new();
    let writer = message_writer::secure::Secure::new(
        message_writer::console::Console::new(),
        identity,
    );
    let salute = salutation::Salutation::new(writer);
    salute.exclaim();
}

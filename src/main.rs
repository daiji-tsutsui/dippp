// Exercise according to
//   Dependency Injection Principles, Practices, and Patterns
// by Steven van Deursen and Mark Seemann

mod message_writer;
mod salutation;
mod identity;

fn main() {
    let identity = identity::dummy::Dummy::new();
    let i_writer = message_writer::console::Console::new();
    let writer = message_writer::secure::Secure::new(i_writer, identity);
    let salute = salutation::Salutation::new(writer);
    salute.exclaim();
}

// Exercise according to
//   Dependency Injection Principles, Practices, and Patterns
// by Steven van Deursen and Mark Seemann

mod message_writer;
mod salutation;

use crate::message_writer::{ ConsoleMessageWriter };
use crate::salutation::{ Salutation };

fn main() {
    let writer = ConsoleMessageWriter::new();
    let salute = Salutation::new(writer);
    salute.exclaim();
}

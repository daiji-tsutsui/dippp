// Exercise according to
//   Dependency Injection Principles, Practices, and Patterns
// by Steven van Deursen and Mark Seemann

mod message_writer;
mod salutation;
mod identity;

use crate::message_writer::{ ConsoleMessageWriter, SecureMessageWriter };
use crate::salutation::{ Salutation };
use crate::identity::{ DummyIdentity };

fn main() {
    let identity = DummyIdentity::new();
    let i_writer = ConsoleMessageWriter::new();
    let writer = SecureMessageWriter::new(i_writer, identity);
    let salute = Salutation::new(writer);
    salute.exclaim();
}

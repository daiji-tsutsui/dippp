// Exercise according to
//   Dependency Injection Principles, Practices, and Patterns
// by Steven van Deursen and Mark Seemann

mod message_writer;
mod salutation;
mod identity;

use crate::message_writer::console::{ ConsoleMessageWriter };
use crate::message_writer::secure::{ SecureMessageWriter };
use crate::salutation::{ Salutation };
use crate::identity::dummy::{ DummyIdentity };

fn main() {
    let identity = DummyIdentity::new();
    let i_writer = ConsoleMessageWriter::new();
    let writer = SecureMessageWriter::new(i_writer, identity);
    let salute = Salutation::new(writer);
    salute.exclaim();
}

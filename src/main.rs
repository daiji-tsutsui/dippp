mod message_writer;
mod salutation;

use crate::message_writer::{ ConsoleMessageWriter };
use crate::salutation::{ Salutation };

fn main() {
    let writer = ConsoleMessageWriter::new();
    let salute = Salutation::new(writer);
    salute.exclaim();
}

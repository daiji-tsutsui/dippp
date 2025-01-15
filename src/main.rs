mod message_writer;
mod salutation;

use crate::message_writer::{ ConsoleMessageWriter };
use crate::salutation::{ Salutation };

fn main() {
    let writer = ConsoleMessageWriter { };
    let salute = Salutation { writer };
    salute.exclaim();
}

use clap::Parser;
use koolctl::ControlStream;

use crate::ctl::KoolCtl;

mod ctl;

fn main() {
    let ctl = KoolCtl::parse();
    let message = ctl.command.into();

    let mut stream = ControlStream::connect().expect("Failed to connect to control socket.");
    stream
        .write(&message)
        .expect("Failed to write message to stream.");
    stream.shutdown().expect("Failed to shutdown stream.");
}

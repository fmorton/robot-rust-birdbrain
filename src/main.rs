mod request;
mod device;
mod state;
mod microbit;
mod hummingbird;
mod finch;

use std::error::Error;
use crate::microbit::Microbit;
use crate::device::Device;
use crate::request::Request;
//use crate::device::new;

fn main() -> Result<(), Box<dyn Error>> {
    let device = Device::new("A");
    let connected = device.is_connected();
    println!("{:?} {}", {}, connected);

    let microbit = Microbit::new("A");

    Ok(())
}

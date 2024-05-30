extern crate rustpi_io;
use std::io::{Read, Write};

use rustpi_io::serial;

fn main() {
    let text_to_write = "Hello, World!";

    let mut spi = serial::SerialPi::new(
        serial::Device::CE0,
        serial::Speed::Khz15_2,
        serial::SpiMode::Mode0,
        serial::ComMode::FullDuplex,
    )
    .unwrap();
    write!(spi, "{}", text_to_write).unwrap();

    let mut buf = Vec::new();
    spi.read_to_end(&mut buf).unwrap();
    let received_string = String::from_utf8(buf).unwrap();
    assert_eq!(received_string, text_to_write);
}

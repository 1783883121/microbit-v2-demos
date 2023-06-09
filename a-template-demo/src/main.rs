#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use microbit::board::Board;
use microbit::hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();

    board.display_pins.col2.set_low().unwrap();
    board.display_pins.row3.set_high().unwrap();

    loop {}
}
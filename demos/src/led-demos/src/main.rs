#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::board::Board;
// use rtt_target::{rtt_init_print, rprintln};
use microbit::display::blocking::Display;
use microbit::hal::{self, prelude::*};
use panic_halt as _;

#[entry]
fn main() -> ! {
    // 点亮LED
    // light_led_fn_1();
    // light_led_fn_2();

    // 闪烁LED
    // delaying_fn_1();
    delaying_fn_2();

    loop {}
}

///点亮LED方式1
fn light_led_fn_1() {
    let mut board = Board::take().unwrap();
    /*
        col数字.set_low() 表示设置当前使用第几列
        row数字.set_high() 表示设置第几行亮
    */
    // 第2列第3行
    board.display_pins.col2.set_low().unwrap();
    board.display_pins.row3.set_high().unwrap();

    // 第4列第5行(由于之前设置了第二列，所有第二列的第五行也会亮，之前设置了第三行，所以第四列第三行也会亮)
    board.display_pins.col4.set_low().unwrap();
    board.display_pins.row5.set_high().unwrap();
}

///点亮方式2
fn light_led_fn_2() {
    let board = Board::take().unwrap();
    // make a timer
    let mut timer = hal::Timer::new(board.TIMER0);
    // create the Display
    let mut display = Display::new(board.display_pins);
    // and light up some LEDs
    // 设置LED点亮的图形  1表示该位置点亮  0表示不亮
    let heart = [
        [0, 1, 0, 1, 0],
        [0, 0, 0, 0, 0],
        [1, 0, 1, 0, 1],
        [0, 1, 0, 1, 0],
        [0, 0, 1, 0, 0],
    ];
    // show方法  (延迟时间 Timer对象 通过设置Time延迟实现闪烁功能,显示图形一个二维数组,过渡时间 显示时间)
    display.show(&mut timer, heart, 10000);
    // loop {
    //     display.show(&mut timer, heart, 1000);
    //     display.clear();
    //     timer.delay_ms(250u32);
    // }
}

///闪烁方式1
fn delaying_fn_1() {
    // rtt_init_print!();
    let mut board = Board::take().unwrap();

    let mut timer = hal::Timer::new(board.TIMER0);

    board.display_pins.col1.set_low().unwrap();
    let mut row1 = board.display_pins.row1;

    loop {
        row1.set_low().unwrap();
        // rprintln!("Dark!");
        timer.delay_ms(1_000_u16);
        row1.set_high().unwrap();
        // rprintln!("Light!");
        timer.delay_ms(1_000_u16);
    }
}

// 闪烁方式2
fn delaying_fn_2() {
    let board = Board::take().unwrap();
    // make a timer
    let mut timer = hal::Timer::new(board.TIMER0);
    // create the Display
    let mut display = Display::new(board.display_pins);
    // and light up some LEDs
    // 设置LED点亮的图形  1表示该位置点亮  0表示不亮
    let heart = [
        [0, 1, 0, 1, 0],
        [0, 0, 0, 0, 0],
        [1, 0, 1, 0, 1],
        [0, 1, 0, 1, 0],
        [0, 0, 1, 0, 0],
    ];
    loop {
        // show方法  (延迟时间 Timer对象 通过设置Time延迟实现闪烁功能,显示图形一个二维数组,过渡时间 显示时间)
        display.show(&mut timer, heart, 1000);
        display.clear();
        timer.delay_ms(500u32);
    }
}

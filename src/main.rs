#![no_std]
#![no_main]

use attiny_hal as hal;
use attiny_hal::prelude::*;
use panic_halt as _;

type CoreClock = hal::clock::MHz16;

#[hal::entry]
fn main() -> ! {
    let mut delay = hal::delay::Delay::<crate::CoreClock>::new();
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);
    
    let mut led = pins.pb1.into_output();
    led.set_high();

    loop {
        led.toggle();
        delay.delay_ms(500u16);
        led.toggle();
        delay.delay_ms(500u16);
    }
}

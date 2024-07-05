#![no_std]
#![no_main]

use arduino_hal as hal;
use panic_halt as _;

#[hal::entry]
fn main() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);

    // Digital pin 1 is also connected to an onboard LED marked "L"
    let mut led = pins.d1.into_output();
    led.set_high();

    loop {
        led.toggle();
        hal::delay_ms(1000);
        led.toggle();
        hal::delay_ms(200);
    }
}

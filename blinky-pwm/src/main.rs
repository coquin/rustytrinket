#![no_std]
#![no_main]

use attiny_hal::simple_pwm::{IntoPwmPin, Prescaler, Timer0Pwm};
use attiny_hal as hal;
use attiny_hal::prelude::*;
use panic_halt as _;

type CoreClock = hal::clock::MHz16; 

#[hal::entry]
fn main() -> ! {
    let mut delay = hal::delay::Delay::<crate::CoreClock>::new();
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);

    let mut timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);

    let mut d1 = pins.pb1.into_output().into_pwm(&mut timer0);
    d1.set_duty(0);
    d1.enable();

    let d = 2u8;

    loop { 
        let mut index: u8 = 0;
        while index < u8::max_value()  {
            d1.set_duty(index);
            index += 1;
            delay.delay_ms(d);
        }
        while index > 0 {
            d1.set_duty(index);
            index -= 1;
            delay.delay_ms(d);
        }
    }
}

#![no_std]
#![no_main]

use arduino_hal::port::mode::{Floating, Input};
use arduino_hal::port::Pin;
use panic_halt as _;

enum Led {
    Led1,
    Led2,
    Led3,
    Led4,
    Led5,
    Led6,
}

fn led_on(
    b: Pin<Input<Floating>>,
    c: Pin<Input<Floating>>,
) -> (Pin<Input<Floating>>, Pin<Input<Floating>>) {
    let _b = b.into_output_high();
    let _c = c.into_output();
    arduino_hal::delay_ms(100);
    return (_b.into_floating_input(), _c.into_floating_input());
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut pin2 = pins.d2.downgrade();
    let mut pin3 = pins.d3.downgrade();
    let mut pin4 = pins.d4.downgrade();

    let mut i = Led::Led1;

    loop {
        match i {
            Led::Led1 => {
                (pin3, pin2) = led_on(pin3, pin2);
                i = Led::Led2;
            }
            Led::Led2 => {
                (pin2, pin3) = led_on(pin2, pin3);
                i = Led::Led3;
            }
            Led::Led3 => {
                (pin4, pin3) = led_on(pin4, pin3);
                i = Led::Led4;
            }
            Led::Led4 => {
                (pin3, pin4) = led_on(pin3, pin4);
                i = Led::Led5;
            }
            Led::Led5 => {
                (pin2, pin4) = led_on(pin2, pin4);
                i = Led::Led6;
            }
            Led::Led6 => {
                (pin4, pin2) = led_on(pin4, pin2);
                i = Led::Led1;
            }
        }
    }
}

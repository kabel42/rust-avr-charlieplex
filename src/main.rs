#![no_std]
#![no_main]

use panic_halt as _;

enum Led {
    Led1,
    Led2,
    Led3,
    Led4,
    Led5,
    Led6,
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let mut pin2 = pins.d2;
    let mut pin3 = pins.d3;
    let mut pin4 = pins.d4;

    let mut i = Led::Led1;

    loop {
        match i {
            Led::Led1 => {
                let t4 = pin4.into_opendrain();
                let t3 = pin3.into_output_high();
                let t2 = pin2.into_output();
                arduino_hal::delay_ms(1);
                pin2 = t2.into_floating_input();
                pin3 = t3.into_floating_input();
                pin4 = t4.into_floating_input();
                i = Led::Led2;
            }
            Led::Led2 => {
                let t4 = pin4.into_opendrain();
                let t3 = pin3.into_output();
                let t2 = pin2.into_output_high();
                arduino_hal::delay_ms(1);
                pin2 = t2.into_floating_input();
                pin3 = t3.into_floating_input();
                pin4 = t4.into_floating_input();
                i = Led::Led3;

            }
            Led::Led3 => {
                let t2 = pin2.into_opendrain();
                let t3 = pin3.into_output();
                let t4 = pin4.into_output_high();
                arduino_hal::delay_ms(1);
                pin2 = t2.into_floating_input();
                pin3 = t3.into_floating_input();
                pin4 = t4.into_floating_input();
                i = Led::Led4;

            }
            Led::Led4 => {
                let t2 = pin2.into_opendrain();
                let t3 = pin3.into_output_high();
                let t4 = pin4.into_output();
                arduino_hal::delay_ms(1);
                pin2 = t2.into_floating_input();
                pin3 = t3.into_floating_input();
                pin4 = t4.into_floating_input();
                i = Led::Led5;

            }
            Led::Led5 => {
                let t3 = pin3.into_opendrain();
                let t2 = pin2.into_output_high();
                let t4 = pin4.into_output();
                arduino_hal::delay_ms(1);
                pin2 = t2.into_floating_input();
                pin3 = t3.into_floating_input();
                pin4 = t4.into_floating_input();
                i = Led::Led6;

            }
            Led::Led6 => {
                let t3 = pin3.into_opendrain();
                let t2 = pin2.into_output();
                let t4 = pin4.into_output_high();
                arduino_hal::delay_ms(1);
                pin2 = t2.into_floating_input();
                pin3 = t3.into_floating_input();
                pin4 = t4.into_floating_input();
                i = Led::Led1;

            }
        }
    }
}

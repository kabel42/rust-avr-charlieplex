#![no_std]
#![no_main]

use arduino_hal::port::mode::{Floating, Input};
use arduino_hal::port::Pin;
use panic_halt as _;

fn delay() {
    arduino_hal::delay_us(20);
}

fn _led_on(
    b: Pin<Input<Floating>>,
    c: Pin<Input<Floating>>,
) -> (Pin<Input<Floating>>, Pin<Input<Floating>>) {
    let _b = b.into_output_high();
    let _c = c.into_output();
    delay();
    return (_b.into_floating_input(), _c.into_floating_input());
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut pin2 = pins.d2.downgrade();
    let mut pin3 = pins.d3.downgrade();
    let mut pin4 = pins.d4.downgrade();

    let mut i: usize = 0;
    let mut j: i8 = 0;
    let mut data: [i8; 6] = [0; 6];
    let mut dir: i8 = 1;
    let mut cnt: u8 = 0;
    let min = -3;
    let max = 4;

    loop {
        if data[i] > j {
            match i {
                0 => {
                    (pin3, pin2) = _led_on(pin3, pin2);
                }
                1 => {
                    (pin2, pin3) = _led_on(pin2, pin3);
                }
                2 => {
                    (pin4, pin3) = _led_on(pin4, pin3);
                }
                3 => {
                    (pin3, pin4) = _led_on(pin3, pin4);
                }
                4 => {
                    (pin2, pin4) = _led_on(pin2, pin4);
                }
                5 => {
                    (pin4, pin2) = _led_on(pin4, pin2);
                }
                _ => {}
            };
          } else {
            delay();
          }
          i += 1;

          if i == 6
          {
            i = 0;
            j += 1;
            if j == max
            {
              j = 0;
              cnt += 1;
              if cnt == 150
              {
                cnt = 0;
                data[5] = data[4];
                data[4] = data[3];
                data[3] = data[2];
                data[2] = data[1];
                data[1] = data[0];
                data[0] += dir;
                if data[0] == max {dir = -1;}
                if data[0] == min {dir = 1;}
              }
            }
          }
    }
}

#![no_std]
#![no_main]

use arduino_hal::port::mode::{Floating, Input, Output};
use arduino_hal::port::Pin;
use panic_halt as _;

fn delay() {
    //arduino_hal::delay_us(20);
    arduino_hal::delay_ms(1);
}

enum TristatePin {
    Floating(Pin<Input<Floating>>),
    Output(Pin<Output>),
}

impl TristatePin {
    fn to_float(self) -> TristatePin {
        match self {
            TristatePin::Floating(p) => TristatePin::Floating(p.into_floating_input()),
            TristatePin::Output(p) => TristatePin::Floating(p.into_floating_input()),
        }
    }
    fn to_high(self) -> TristatePin {
        match self {
            TristatePin::Floating(p) => TristatePin::Output(p.into_output_high()),
            TristatePin::Output(p) => TristatePin::Output(p.into_output_high()),
        }
    }
    fn to_low(self) -> TristatePin {
        match self {
            TristatePin::Floating(p) => TristatePin::Output(p.into_output()),
            TristatePin::Output(p) => TristatePin::Output(p.into_output()),
        }
    }
}

struct Charlieplex {
    pin0: TristatePin,
    pin1: TristatePin,
    pin2: TristatePin,
    leds: [(usize, usize); 6],
}

impl Charlieplex {
    fn led_on(mut self, led: Option<usize>) -> Charlieplex{
        match led {
            Some(l) => {
                fn helper(pin: TristatePin, leds: &(usize, usize), led: usize) -> TristatePin {
                    if led == leds.0 {
                        pin.to_high()
                    } else if led == leds.1 {
                        pin.to_low()
                    } else {
                        pin.to_float()
                    }
                }

                self.pin0 = helper(self.pin0, &self.leds[l], 0);
                self.pin1 = helper(self.pin1, &self.leds[l], 1);
                self.pin2 = helper(self.pin2, &self.leds[l], 2);
            }
            _ => {
                self.pin0 = self.pin0.to_float();
                self.pin1 = self.pin1.to_float();
                self.pin2 = self.pin2.to_float();
            }
        }
        self
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut c = Charlieplex {
        pin0: TristatePin::Floating(pins.d2.downgrade()),
        pin1: TristatePin::Floating(pins.d3.downgrade()),
        pin2: TristatePin::Floating(pins.d4.downgrade()),
        leds: [(1, 0), (0, 1), (2, 1), (1, 2), (0, 2), (2, 0)],
    };

    let mut i: usize = 0;
    let mut j: i8 = 0;
    let mut data: [i8; 6] = [0; 6];
    let mut dir: i8 = 1;
    let mut cnt: u8 = 0;
    let min = -1;
    let max = 3;

    loop {
        if data[i] > j {
            c = c.led_on(Some(i));
        } else {
            c = c.led_on(None);
        }
        i += 1;

        if i == 6 {
            i = 0;
            j += 1;
            if j == max {
                j = 0;
                cnt += 1;
                if cnt == 10 {
                    cnt = 0;
                    data[5] = data[4];
                    data[4] = data[3];
                    data[3] = data[2];
                    data[2] = data[1];
                    data[1] = data[0];
                    data[0] += dir;
                    if data[0] == max {
                        dir = -1;
                    }
                    if data[0] == min {
                        dir = 1;
                    }
                }
            }
        }
        delay();
    }
}

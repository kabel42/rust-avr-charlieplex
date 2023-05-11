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
    a: TristatePin,
    b: TristatePin,
    c: TristatePin,
}

impl Charlieplex {
    fn led_on(self, led: usize) -> Charlieplex {
        match led {
            0 => Charlieplex {
                a: self.a.to_low(),
                b: self.b.to_high(),
                c: self.c.to_float(),
            },
            1 => Charlieplex {
                a: self.a.to_high(),
                b: self.b.to_low(),
                c: self.c.to_float(),
            },
            2 => Charlieplex {
                a: self.a.to_float(),
                b: self.b.to_low(),
                c: self.c.to_high(),
            },
            3 => Charlieplex {
                a: self.a.to_float(),
                b: self.b.to_high(),
                c: self.c.to_low(),
            },
            4 => Charlieplex {
                a: self.a.to_high(),
                b: self.b.to_float(),
                c: self.c.to_low(),
            },
            5 => Charlieplex {
                a: self.a.to_low(),
                b: self.b.to_float(),
                c: self.c.to_high(),
            },
            _ => Charlieplex {
                a: self.a.to_float(),
                b: self.b.to_float(),
                c: self.c.to_float(),
            },
        }
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut c = Charlieplex {
        a: TristatePin::Floating(pins.d2.downgrade()),
        b: TristatePin::Floating(pins.d3.downgrade()),
        c: TristatePin::Floating(pins.d4.downgrade()),
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
            c = c.led_on(i);
        } else {
            c = c.led_on(usize::MAX);
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

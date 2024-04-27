#![feature(try_blocks)]
// use std:sync::mpsc::channel;
use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

type Err = Box<dyn Error>;

enum PinType {
    Input,
    Output,
}

enum Pin {
    In(rppal::gpio::InputPin),
    Out(rppal::gpio::OutputPin),
}

struct Pixel {
    pin: Pin,
    is_on: bool,
}

impl Pixel {
    fn new(pin_number: u8, pin_type: PinType) -> Result<Self, Err> {
        let pin = Gpio::new()?.get(pin_number)?;
        Ok(Self {
            pin: match pin_type {
                PinType::Input => Pin::In(pin.into_input_pullup()),
                PinType::Output => Pin::Out(pin.into_output()),
            },
            is_on: false,
        })
    }
    fn set(&mut self, value: bool) -> Result<(), &'static str> {
        if let Pin::Out(pin) = &mut self.pin {
            if self.is_on != value {
                self.is_on = value;
                if value {
                    pin.set_high();
                } else {
                    pin.set_low();
                }
            }
            Ok(())
        } else {
            Err("Can' set a input pin")
        }
    }
    fn is_on(&self) -> bool {
        self.is_on
    }
}

fn main() -> Result<(), Err> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());
    let mut r = Pixel::new(14, PinType::Output)?;
    let mut g = Pixel::new(15, PinType::Output)?;
    let mut b = Pixel::new(18, PinType::Output)?;
    let mut w = Pixel::new(23, PinType::Output)?;
    let y = Gpio::new()?.get(3)?.into_input_pullup();
    let mut toggle = false;

    loop {
        if y.is_low() {
            toggle = !toggle;
        }
        r.set(!toggle)?;
        g.set(!toggle)?;
        b.set(!toggle)?;
        w.set(!toggle)?;
        thread::sleep(Duration::from_secs(1));
    }
}

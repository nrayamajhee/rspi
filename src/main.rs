#![feature(try_blocks)]

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::error::Error;
use std::thread;
use std::time::{Duration, Instant};

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

type Err = Box<dyn Error>;

struct Led {
    pin: rppal::gpio::OutputPin,
    is_on: bool,
}

impl Led {
    fn new(pin_number: u8) -> Result<Self, Err> {
        let mut pin = Gpio::new()?.get(pin_number)?.into_output();
        pin.set_low();
        Ok(Self { pin, is_on: false })
    }
    fn set(&mut self, value: bool) {
        if self.is_on != value {
            if value {
                self.pin.set_high();
            } else {
                self.pin.set_low();
            }
            self.is_on = value;
        }
    }
}

struct Button {
    pin: rppal::gpio::InputPin,
    pressed: bool,
}

impl Button {
    fn new(pin_number: u8) -> Result<Self, Err> {
        Ok(Self {
            pin: Gpio::new()?.get(pin_number)?.into_input_pullup(),
            pressed: false,
        })
    }
    fn is_pressed(&self) -> bool {
        self.pressed
    }
    fn is_pressed_up(&self) -> bool {
        self.pressed && self.pin.is_high()
    }
    fn update(&mut self) {
        self.pressed = self.pin.is_low();
    }
}

#[derive(PartialEq, FromPrimitive, Clone, Copy)]
enum Lighting {
    Off = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

#[derive(PartialEq, FromPrimitive, Clone, Copy)]
enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
    White = 3,
    All = 4,
}

fn main() -> Result<(), Err> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());
    let mut r = Led::new(14)?;
    let mut g = Led::new(15)?;
    let mut b = Led::new(18)?;
    let mut w = Led::new(23)?;
    let mut o = Button::new(2)?;
    let mut y = Button::new(3)?;
    let mut lighting = Lighting::Off;
    let mut color = Color::All;
    let mut flashing = false;
    let mut then = Instant::now();

    loop {
        if y.is_pressed_up() {
            lighting = FromPrimitive::from_usize((lighting as usize + 1) % 4).unwrap();
        }
        if o.is_pressed_up() {
            color = FromPrimitive::from_usize((color as usize + 1) % 5).unwrap();
        }
        y.update();
        o.update();
        if lighting != Lighting::Off {
            let now = Instant::now();
            if (now - then) >= Duration::from_millis(100 * (lighting as u64)) {
                flashing = !flashing;
                then = now;
            }
            match color {
                Color::Red => {
                    r.set(flashing);
                    g.set(false);
                    b.set(false);
                    w.set(false);
                }
                Color::Green => {
                    r.set(false);
                    g.set(flashing);
                    b.set(false);
                    w.set(false);
                }
                Color::Blue => {
                    r.set(false);
                    g.set(false);
                    b.set(flashing);
                    w.set(false);
                }
                Color::White => {
                    r.set(false);
                    g.set(false);
                    b.set(false);
                    w.set(flashing);
                }
                Color::All => {
                    r.set(flashing);
                    g.set(flashing);
                    b.set(flashing);
                    w.set(flashing);
                }
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
}

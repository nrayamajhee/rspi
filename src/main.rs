use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

const GPIO_LED: u8 = 14;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

    loop {
        pin.toggle();
        thread::sleep(Duration::from_secs(1));
    }
}

#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m;
use cortex_m_rt::entry;

use crate::hal::{prelude::*, stm32};
use stm32f4xx_hal as hal;

#[entry]
fn main() -> ! {
    // Access the device peripherals (dp) and cortex peripherals (cp):
    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED: it's connected to pin PA5 on the microcontroler
        let gpioa = dp.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();

        // The external LED, on the next pin down:
        let mut xled = gpioa.pa6.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

        loop {
            // On for 1s, off for 1s.
            // https://doc.rust-lang.org/std/convert/enum.Infallible.html
            led.set_high().unwrap();
            xled.set_low().unwrap();
            delay.delay_ms(1000_u32);
            led.set_low().unwrap();
            xled.set_high().unwrap();
            delay.delay_ms(1000_u32);
        }
    } else {
        panic!("failed to access peripherals");
    }
}

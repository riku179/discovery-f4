#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal::{
    prelude::*,
    prelude::_embedded_hal_digital_v2_OutputPin as OutputPin,
    gpio::gpiod::Parts,
    delay::Delay,
    stm32,
};

#[entry]
fn main() -> ! {
    let dp: stm32::Peripherals = stm32::Peripherals::take().unwrap();
    let cp: cortex_m::Peripherals = cortex_m::peripheral::Peripherals::take().unwrap();

    let gpiod: Parts = dp.GPIOD.split();

    let mut green = gpiod.pd12.into_push_pull_output();
    let mut orange = gpiod.pd13.into_push_pull_output();
    let mut red = gpiod.pd14.into_push_pull_output();
    let mut blue = gpiod.pd15.into_push_pull_output();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

    let mut delay = Delay::new(cp.SYST, clocks);

    let delay_ms= 500_u32;

    loop {
        blink_led(&mut green, &mut delay, delay_ms);
        blink_led(&mut orange, &mut delay, delay_ms);
        blink_led(&mut red, &mut delay, delay_ms);
        blink_led(&mut blue, &mut delay, delay_ms);
    }
}

fn blink_led<T: OutputPin>(led: &mut T, delay: &mut Delay, delay_ms: u32) -> () {
    led.set_high();
    delay.delay_ms(delay_ms);
    led.set_low();
    delay.delay_ms(delay_ms);
}
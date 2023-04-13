#![no_std]
#![no_main]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

//use panic_halt as _;
use panic_semihosting as _;

mod bsp;
use bsp::prelude::*;

// Traits
use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin}; // for pin.toggle()
use hal::clocks::Clock; // for system_clock.freq()


#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        bsp::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let sio = hal::Sio::new(pac.SIO);
    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led: bsp::Led = pins.led.into_mode();

    led.set_low().unwrap();
    loop {
        led.toggle().unwrap();
        delay.delay_ms(500);
        panic!("haaaalp!");
    }
}

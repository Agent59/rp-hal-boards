//! # Reset To Usb Boot Example for the Blok microcontroller
//!
//! Resets the Blok microcontroller after 10 seconds to usb boot mode.
//!
//! Afterwards the microcontroller should be automatically mounted as a drive,
//! just like when booted while holding down the boot button.
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

use core::iter::once;
use embedded_hal::timer::CountDown;
use panic_halt as _;
use blok_rp2040::{entry, hal};
use blok_rp2040::{
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        pac,
        pac::interrupt,
        pio::PIOExt,
        timer::Timer,
        watchdog::Watchdog,
        Sio,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);

    let core = pac::CorePeripherals::take().unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    loop {
        // waits 10 seconds
        delay.delay_ms(10_000);

        // resets to usb boot mode
        hal::rom_data::reset_to_usb_boot(0, 0);
    }
}

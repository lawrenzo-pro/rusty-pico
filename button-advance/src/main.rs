#![no_std]
#![no_main]

use core::{panic::PanicInfo, pin::Pin};
use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use defmt_rtt as _;
use embedded_hal::{
    digital::v2::{InputPin, OutputPin},
    watchdog,
};
use embedded_time::fixed_point::FixedPoint;
use hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    watchdog::Watchdog,
    Sio, gpio::Pins,
};
use rp2040_hal as hal;
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;
pub mod abstr{
    
}
#[entry]
fn start() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let sio = Sio::new(pac.SIO);
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let external_freq = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_freq,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST,clocks.system_clock.freq().to_Hz());
    let mut pins =hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS
    );
    let mut led1 = pins.gpio2.into_push_pull_output();
    let mut i = 10;
    let mut k = 200;
    let mut j = 0;
    loop {
        led1.set_high().unwrap();
        delay.delay_ms(500);
        led1.set_low().unwrap();
        delay.delay_ms(500);
        j = k / i;
        i = i - 1;
    }
}
fn blink() -> !{
    let mut pac = unsafe{pac::Peripherals::steal()};
    let core = unsafe{pac::CorePeripherals::steal()};
    let sio = Sio::new(pac.SIO);
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let external_freq = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_freq,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST,clocks.system_clock.freq().to_Hz());
    let pins =hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS
    );
    let mut led2 = pins.gpio6.into_push_pull_output();
    loop{
        led2.set_high().unwrap();
        delay.delay_ms(333);
        led2.set_low().unwrap();
        delay.delay_ms(333);
    }

}
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    blink();
}

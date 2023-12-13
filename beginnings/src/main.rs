#![no_std] //don't link the standard library,  
#![no_main] // the entry_point may not be main

extern crate panic_halt;
extern crate embedded_hal;
extern crate rp2040_hal;

use panic_halt as _;
use rp2040_hal as hal;
use hal::pac::{self, usbctrl_regs::buff_status};

use embedded_hal::{digital::v2::{OutputPin,InputPin}, watchdog};
use rp2040_hal::clocks::Clock;

//Link the bootloader
#[link_section = ".boot2"]
pub static BOOT2 : [u8;256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[rp2040_hal::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    ).ok().unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST,clocks.system_clock.freq().to_Hz());
    let sio = hal::Sio::new(pac.SIO);

    let pins: hal::gpio::Pins = hal::gpio::Pins::new(
      pac.IO_BANK0,
      pac.PADS_BANK0,
      sio.gpio_bank0,
      &mut pac.RESETS,
    );
    let mut led = pins.gpio6.into_push_pull_output();
    let button = pins.gpio13.into_pull_down_input();
    
    loop{
        if button.is_high().unwrap(){
            led.set_high().unwrap();
            delay.delay_ms(1000);
        }
        led.set_low().unwrap();
        delay.delay_ms(1000);
    }
}


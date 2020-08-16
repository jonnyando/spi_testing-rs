//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use nb::block;

use cortex_m_rt::entry;
use cortex_m::iprintln;
use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer, spi};

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut itm = cp.ITM;

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOC peripheral
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    // LED on stm32f103rb nucleo board uses pa5 for LED
    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
    // Using SPI2 (sckl: pb13, miso: pb14, mosi: pb15)
    let sclk = gpiob.pb13.into_alternate_push_pull(&mut gpiob.crh);
    let miso = gpiob.pb14.into_floating_input(&mut gpiob.crh);
    let mosi = gpiob.pb15.into_alternate_push_pull(&mut gpiob.crh);
    let mut cs = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);
    let spi_mode = spi::Mode{
        polarity: spi::Polarity::IdleHigh,
        phase: spi::Phase::CaptureOnFirstTransition,
    };
    let mut spi = spi::Spi::spi2(dp.SPI2, (sclk, miso, mosi), spi_mode, 100.khz(), clocks, &mut rcc.apb1);

    // Configure the syst timer to trigger an update every second
    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(1.hz());

    // Wait for the timer to trigger an update and change the state of the LED
    let mut transfer_buffer: [u8; 4];
    loop {
        transfer_buffer = [0x0A, 0x0A, 0xA0, 0xA0];

        let tx= &mut transfer_buffer;
        block!(timer.wait()).unwrap();
        led.set_high().unwrap();
        
        cs.set_low().unwrap();
        let rx = spi.transfer(tx).unwrap();
        cs.set_high().unwrap();

        block!(timer.wait()).unwrap();
        led.set_low().unwrap();
        iprintln!(&mut itm.stim[0], "Hello, world!");
        iprintln!(&mut itm.stim[0], "{}, {}, {}, {}", rx[0], rx[1], rx[2], rx[3]);
        if rx.contains(&0xFF) {
            iprintln!(&mut itm.stim[0], "not recieved!");
        } else {
            iprintln!(&mut itm.stim[0], "recieved!");
        }

    }
}
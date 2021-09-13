#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use nb::block;

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

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

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOC peripheral
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    // Configure GPIOA 6-7 and GPIOB 0-1  as push-pull output. The `crl` register is passed to the function
    // in order to configure the port. For pins 8-15, crh should be passed instead.
    let mut led1 = gpioa.pa6.into_push_pull_output(&mut gpioa.crl);
    let mut led2 = gpioa.pa7.into_push_pull_output(&mut gpioa.crl);
    let mut led3 = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
    let mut led4 = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);
    // Configure the syst timer to trigger an update every second
    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(50.hz());

    // Wait for the timer to trigger an update and change the state of the LED
    loop {
        block!(timer.wait()).unwrap();
        led1.set_high().unwrap();
        block!(timer.wait()).unwrap();
        led2.set_high().unwrap();
        block!(timer.wait()).unwrap();
        led3.set_high().unwrap();
        block!(timer.wait()).unwrap();
        led4.set_high().unwrap();
        block!(timer.wait()).unwrap();
        led1.set_low().unwrap();
        block!(timer.wait()).unwrap();
        led2.set_low().unwrap();
        block!(timer.wait()).unwrap();
        led3.set_low().unwrap();
        block!(timer.wait()).unwrap();
        led4.set_low().unwrap();

    }
}
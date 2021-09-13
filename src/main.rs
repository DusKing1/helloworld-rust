#![no_main]
#![no_std]

use panic_halt as _;

use nb::block;

use core::mem::MaybeUninit;
use cortex_m_rt::entry;
use pac::interrupt;
use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::gpio::*;
use stm32f1xx_hal::{pac, prelude::*,serial::{self,Serial}};

// Ugly approach, need to find a better way to do this
static mut LED1: MaybeUninit<stm32f1xx_hal::gpio::gpioa::PA6<Output<PushPull>>> =
    MaybeUninit::uninit();
static mut LED2: MaybeUninit<stm32f1xx_hal::gpio::gpioa::PA7<Output<PushPull>>> =
    MaybeUninit::uninit();
static mut LED3: MaybeUninit<stm32f1xx_hal::gpio::gpiob::PB0<Output<PushPull>>> =
    MaybeUninit::uninit();
static mut LED4: MaybeUninit<stm32f1xx_hal::gpio::gpiob::PB1<Output<PushPull>>> =
    MaybeUninit::uninit();
static mut KEY1: MaybeUninit<stm32f1xx_hal::gpio::gpioa::PA1<Input<PullDown>>> =
    MaybeUninit::uninit();
static mut KEY2: MaybeUninit<stm32f1xx_hal::gpio::gpioa::PA2<Input<PullDown>>> =
    MaybeUninit::uninit();
static mut KEY3: MaybeUninit<stm32f1xx_hal::gpio::gpioa::PA3<Input<PullDown>>> =
    MaybeUninit::uninit();
static mut KEY4: MaybeUninit<stm32f1xx_hal::gpio::gpioa::PA4<Input<PullDown>>> =
    MaybeUninit::uninit();


#[interrupt]
fn EXTI1() {
    let led1 = unsafe { &mut *LED1.as_mut_ptr() };
    let key1 = unsafe { &mut *KEY1.as_mut_ptr() };

    if key1.check_interrupt() {

        // block!(Timer::syst(cortex_m::Peripherals::take().unwrap().SYST,
        // &pac::Peripherals::take().unwrap()
        // .RCC.constrain().cfgr
        // .freeze(&mut pac::Peripherals::take().unwrap().FLASH.constrain().acr))
        // .start_count_down(50.hz()).wait()).unwrap();
        
        led1.toggle().unwrap();

        // if we don't clear this bit, the ISR would trigger indefinitely
        key1.clear_interrupt_pending_bit();
    }
}

#[interrupt]
fn EXTI2() {
    let led2 = unsafe { &mut *LED2.as_mut_ptr() };
    let key2 = unsafe { &mut *KEY2.as_mut_ptr() };

    if key2.check_interrupt() {

        // block!(Timer::syst(cortex_m::Peripherals::take().unwrap().SYST,
        // &pac::Peripherals::take().unwrap()
        // .RCC.constrain().cfgr
        // .freeze(&mut pac::Peripherals::take().unwrap().FLASH.constrain().acr))
        // .start_count_down(50.hz()).wait()).unwrap();
        
        led2.toggle().unwrap();

        key2.clear_interrupt_pending_bit();
    }
}

#[interrupt]
fn EXTI3() {
    let led3 = unsafe { &mut *LED3.as_mut_ptr() };
    let key3 = unsafe { &mut *KEY3.as_mut_ptr() };
    if key3.check_interrupt() {
        led3.toggle().unwrap();

        // block!(Timer::syst(cortex_m::Peripherals::take().unwrap().SYST,
        // &pac::Peripherals::take().unwrap()
        // .RCC.constrain().cfgr
        // .freeze(&mut pac::Peripherals::take().unwrap().FLASH.constrain().acr))
        // .start_count_down(50.hz()).wait()).unwrap();

        key3.clear_interrupt_pending_bit();
    }
}

#[interrupt]
fn EXTI4() {
    let led4 = unsafe { &mut *LED4.as_mut_ptr() };
    let key4 = unsafe { &mut *KEY4.as_mut_ptr() };
    if key4.check_interrupt() {
        led4.toggle().unwrap();

        // block!(Timer::syst(cortex_m::Peripherals::take().unwrap().SYST,
        // &pac::Peripherals::take().unwrap()
        // .RCC.constrain().cfgr
        // .freeze(&mut pac::Peripherals::take().unwrap().FLASH.constrain().acr))
        // .start_count_down(50.hz()).wait()).unwrap();

        key4.clear_interrupt_pending_bit();
    }
}


#[entry]
fn main() -> ! {
    // initialization phase
    let p = pac::Peripherals::take().unwrap();
    let _cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);
    let mut afio = p.AFIO.constrain(&mut rcc.apb2);

    {
        // the scope ensures that the key1234 reference is dropped before the first ISR can be executed.

        let led1 = unsafe { &mut *LED1.as_mut_ptr() };
        *led1 = gpioa.pa6.into_push_pull_output(&mut gpioa.crl);
        led1.set_high().unwrap();
        let led2 = unsafe { &mut *LED2.as_mut_ptr() };
        *led2 = gpioa.pa7.into_push_pull_output(&mut gpioa.crl);
        led2.set_high().unwrap();
        let led3 = unsafe { &mut *LED3.as_mut_ptr() };
        *led3 = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
        led3.set_high().unwrap();
        let led4 = unsafe { &mut *LED4.as_mut_ptr() };
        *led4 = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);
        led4.set_high().unwrap();

        let key1 = unsafe { &mut *KEY1.as_mut_ptr() };
        *key1 = gpioa.pa1.into_pull_down_input(&mut gpioa.crl);
        key1.make_interrupt_source(&mut afio);
        key1.trigger_on_edge(&p.EXTI, Edge::RISING);
        key1.enable_interrupt(&p.EXTI);

        let key2 = unsafe { &mut *KEY2.as_mut_ptr() };
        *key2 = gpioa.pa2.into_pull_down_input(&mut gpioa.crl);
        key2.make_interrupt_source(&mut afio);
        key2.trigger_on_edge(&p.EXTI, Edge::RISING);
        key2.enable_interrupt(&p.EXTI);

        let key3 = unsafe { &mut *KEY3.as_mut_ptr() };
        *key3 = gpioa.pa3.into_pull_down_input(&mut gpioa.crl);
        key3.make_interrupt_source(&mut afio);
        key3.trigger_on_edge(&p.EXTI, Edge::RISING);
        key3.enable_interrupt(&p.EXTI);

        let key4 = unsafe { &mut *KEY4.as_mut_ptr() };
        *key4 = gpioa.pa4.into_pull_down_input(&mut gpioa.crl);
        key4.make_interrupt_source(&mut afio);
        key4.trigger_on_edge(&p.EXTI, Edge::RISING);
        key4.enable_interrupt(&p.EXTI);

    } // initialization ends here

    let mut flash = p.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;

    // let timer = Timer::syst(cp.SYST, &clocks).start_count_down(50.hz());

    let serial = Serial::usart1(
        p.USART1,
        (tx, rx),
        &mut afio.mapr,
        serial::Config::default()
            .baudrate(115200.bps())
            .stopbits(serial::StopBits::STOP1)
            .parity_none(),
            //.parity_odd(),
        clocks,
        &mut rcc.apb2,
    );

    let (mut tx, _rx) = serial.split();

    let init_info = b"Rustlang is running!\r\n";

    let mut i =0;
    while i < init_info.len() {
        block!(tx.write(init_info[i])).unwrap();
        i = i+1;
    } // Showing the code is running on rustlang not C

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI1);
        pac::NVIC::unmask(pac::Interrupt::EXTI2);
        pac::NVIC::unmask(pac::Interrupt::EXTI3);
        pac::NVIC::unmask(pac::Interrupt::EXTI4);
    }

    loop {}
}
// #![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;

use nb::block;

// use cortex_m::asm;
use core::mem::MaybeUninit;
use cortex_m_rt::entry;
use pac::interrupt;
use stm32f1xx_hal::{
    pac,
    gpio::*,
    prelude::*,
    pwm::Channel,
    time::U32Ext,
    timer::{Tim3NoRemap, Timer},
    serial::{self,Serial},
};

static mut KEY1: MaybeUninit<stm32f1xx_hal::gpio::gpioa::PA1<Input<PullDown>>> =
    MaybeUninit::uninit();
static mut KEY2: MaybeUninit<stm32f1xx_hal::gpio::gpioa::PA2<Input<PullDown>>> =
    MaybeUninit::uninit();
static mut KEY3: MaybeUninit<stm32f1xx_hal::gpio::gpioa::PA3<Input<PullDown>>> =
    MaybeUninit::uninit();
static mut KEY4: MaybeUninit<stm32f1xx_hal::gpio::gpioa::PA4<Input<PullDown>>> =
    MaybeUninit::uninit();

static BREATHING_TABLE:[u16; 128] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 3, 3, 3, 5, 5, 7, 7, 9, 9,
    11, 11, 13, 15, 15, 17, 19, 21, 21, 23, 25, 27, 29, 31, 33, 35, 37,
    41, 43, 45, 49, 50, 52, 56, 58, 62, 66, 68, 72 ,76 ,80 ,84 ,88, 92, 
    96, 100, 103, 109, 113, 119, 123, 129, 133, 139, 145, 150, 156, 162,
    168, 174, 180, 186, 192, 200, 205, 211, 219, 227, 233, 241, 247, 254,
    262, 270, 278, 284, 292, 300, 307, 315, 323, 331, 339, 345, 352, 360,
    368 ,376, 382, 390, 398, 403, 411, 417, 423, 429, 437, 443, 447, 452,
    458, 462, 468, 472, 476, 480, 484, 486, 490, 492, 494, 496, 498, 500,
    500, 500];

static mut IA:usize = 1;
static mut IB:usize = 33;
static mut IC:usize = 65;
static mut ID:usize = 97;
static mut DA:u8 = 1;
static mut DB:u8 = 1;
static mut DC:u8 = 1;
static mut DD:u8 = 1;

static mut MODE:u8 = 1;
static mut SPEED:u8 = 1;

fn swapa() {
    unsafe{
        if DA==1 {
            DA = 0;
        } else {
            DA = 1;
        }
    }
}

fn swapb() {
    unsafe{
        if DB==1 {
            DB = 0;
        } else {
            DB = 1;
        }
    }
}

fn swapc() {
    unsafe{
        if DC==1 {
            DC = 0;
        } else {
            DC = 1;
        }
    }
}

fn swapd() {
    unsafe{
        if DD==1 {
            DD = 0;
        } else {
            DD = 1;
        }
    }
}

#[interrupt]
fn EXTI1() {
    let key1 = unsafe { &mut *KEY1.as_mut_ptr() };//change flow direction

    if key1.check_interrupt() {

        swapa();

        swapb();

        swapc();

        swapd();

        key1.clear_interrupt_pending_bit();
    }
}
#[interrupt]
fn EXTI2() {

    let key2 = unsafe { &mut *KEY2.as_mut_ptr() };

    if key2.check_interrupt() {

        unsafe {
            if SPEED >= 3 {
            } else {  
                SPEED = SPEED+1;
            }
        };
        key2.clear_interrupt_pending_bit();
    }
}
#[interrupt]
fn EXTI3() {
    let key3 = unsafe { &mut *KEY3.as_mut_ptr() };
    if key3.check_interrupt() {
        unsafe {
            if SPEED <= 1 {
            } else {
                SPEED = SPEED-1;
            }
        };
        key3.clear_interrupt_pending_bit();
    }
}
#[interrupt]
fn EXTI4() {
    let key4 = unsafe { &mut *KEY4.as_mut_ptr() };
    if key4.check_interrupt() {
        unsafe {
            if MODE ==1 {
                MODE = 0;
            } else {
                MODE = 1;
            }
        };
        key4.clear_interrupt_pending_bit();
    }
}


#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = pac::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = p.AFIO.constrain(&mut rcc.apb2);

    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

    {
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


    let c1 = gpioa.pa6.into_alternate_push_pull(&mut gpioa.crl);
    let c2 = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);
    let c3 = gpiob.pb0.into_alternate_push_pull(&mut gpiob.crl);
    let c4 = gpiob.pb1.into_alternate_push_pull(&mut gpiob.crl);
    let pins = (c1, c2, c3,c4);

    let mut pwm = 
        Timer::tim3(p.TIM3, &clocks, &mut rcc.apb1)
        .pwm::<Tim3NoRemap, _, _, _>(pins, &mut afio.mapr,16.khz(),);

    pwm.enable(Channel::C1);
    pwm.enable(Channel::C2);
    pwm.enable(Channel::C3);
    pwm.enable(Channel::C4);

    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;

    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(800.hz());

    let serial = Serial::usart1(
        p.USART1,
        (tx, rx),
        &mut afio.mapr,
        serial::Config::default()
            .baudrate(115200.bps())
            .stopbits(serial::StopBits::STOP1)
            .parity_none(),
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

    // timer.start_count_down(800.hz());

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI1);
        pac::NVIC::unmask(pac::Interrupt::EXTI2);
        pac::NVIC::unmask(pac::Interrupt::EXTI3);
        pac::NVIC::unmask(pac::Interrupt::EXTI4);
    }

    loop {
        // breath_a();
        unsafe { // this is for LED A
            if DA ==1 && IA < 127 {
                IA = IA + 1;
                if MODE ==1 {
                    pwm.set_duty(Channel::C1, BREATHING_TABLE[IA]);
                } else {
                    pwm.set_duty(Channel::C1, 1);
                }
                if SPEED == 1 {
                    block!(timer.wait()).unwrap();
                } else if SPEED==2 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                } else if SPEED==3 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                }
            } else if DA == 1 && IA >= 127 {
                DA = 0;
                IA= IA - 1;
                if MODE ==1 {
                    pwm.set_duty(Channel::C1, BREATHING_TABLE[IA]);
                } else {
                    pwm.set_duty(Channel::C1, 1);
                }
                if SPEED == 1 {
                    block!(timer.wait()).unwrap();
                } else if SPEED==2 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                } else if SPEED==3 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                }
            } else if DA==0 && IA > 0 {
                IA = IA - 1;
                if MODE ==1 {
                    pwm.set_duty(Channel::C1, BREATHING_TABLE[IA]);
                } else {
                    pwm.set_duty(Channel::C1, 480);
                }
                if SPEED == 1 {
                    block!(timer.wait()).unwrap();
                } else if SPEED==2 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                } else if SPEED==3 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                }
            } else if DA==0 && IA<=0{
                DA = 1;
                IA = IA + 1;
                if MODE ==1 {
                    pwm.set_duty(Channel::C1, BREATHING_TABLE[IA]);
                } else {
                    pwm.set_duty(Channel::C1, 480);
                }
                if SPEED == 1 {
                    block!(timer.wait()).unwrap();
                } else if SPEED==2 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                } else if SPEED==3 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                }
            }  
        //};

        //unsafe{// this is for LED B
            if DB ==1 && IB < 127 {
                IB = IB + 1;
                if MODE==1 {
                    pwm.set_duty(Channel::C2, BREATHING_TABLE[IB]);
                } else {
                    pwm.set_duty(Channel::C2, 1);
                }
                if SPEED == 1 {
                    block!(timer.wait()).unwrap();
                } else if SPEED==2 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                } else if SPEED==3 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                }
            } else if DB==1 && IB >= 127 {
                DB = 0;
                IB= IB - 1;
                if MODE==1 {
                    pwm.set_duty(Channel::C2, BREATHING_TABLE[IB]);
                } else {
                    pwm.set_duty(Channel::C2, 1);
                }
                if SPEED == 1 {
                    block!(timer.wait()).unwrap();
                } else if SPEED==2 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                } else if SPEED==3 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                }
            } else if DB==0 && IB > 0 {
                IB = IB-1;
                if MODE==1 {
                    pwm.set_duty(Channel::C2, BREATHING_TABLE[IB]);
                } else {
                    pwm.set_duty(Channel::C2, 480);
                }
                if SPEED == 1 {
                    block!(timer.wait()).unwrap();
                } else if SPEED==2 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                } else if SPEED==3 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                }
            } else if DB==0 && IB<=0{
                DB = 1;
                IB = IB + 1;
                if MODE==1 {
                    pwm.set_duty(Channel::C2, BREATHING_TABLE[IB]);
                } else {
                    pwm.set_duty(Channel::C2, 480);
                }
                if SPEED == 1 {
                    block!(timer.wait()).unwrap();
                } else if SPEED==2 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                } else if SPEED==3 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                }
            }
        //};
        
        //unsafe{// this is for LED C
            if DC ==1 && IC < 127 {
                IC = IC + 1;
                if MODE==1 {
                    pwm.set_duty(Channel::C3, BREATHING_TABLE[IC]);
                } else {
                    pwm.set_duty(Channel::C3, 1);
                }
                if SPEED == 1 {
                    block!(timer.wait()).unwrap();
                } else if SPEED==2 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                } else if SPEED==3 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                }
            } else if DC==1 && IC >= 127 {
                DC = 0;
                IC= IC - 1;
                if MODE==1 {
                    pwm.set_duty(Channel::C3, BREATHING_TABLE[IC]);
                } else {
                    pwm.set_duty(Channel::C3, 1);
                }
                if SPEED == 1 {
                    block!(timer.wait()).unwrap();
                } else if SPEED==2 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                } else if SPEED==3 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                }
            } else if DC==0 && IC > 0 {
                IC = IC-1;
                if MODE==1 {
                    pwm.set_duty(Channel::C3, BREATHING_TABLE[IC]);
                } else {
                    pwm.set_duty(Channel::C3, 480);
                }
                if SPEED == 1 {
                    block!(timer.wait()).unwrap();
                } else if SPEED==2 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                } else if SPEED==3 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                }
            } else if DC==0 && IC<=0 {
                DC = 1;
                IC = IC + 1;
                if MODE==1 {
                    pwm.set_duty(Channel::C3, BREATHING_TABLE[IC]);
                } else {
                    pwm.set_duty(Channel::C3, 480);
                }
                if SPEED == 1 {
                    block!(timer.wait()).unwrap();
                } else if SPEED==2 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                } else if SPEED==3 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                }
            }  
        //};

        //unsafe{// this is for LED D
            if DD==1 && ID < 127 {
                ID = ID + 1;
                if MODE==1 {
                    pwm.set_duty(Channel::C4, BREATHING_TABLE[ID]);
                } else {
                    pwm.set_duty(Channel::C4, 1);
                }
                if SPEED == 1 {
                    block!(timer.wait()).unwrap();
                } else if SPEED==2 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                } else if SPEED==3 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                }
            } else if DD==1 && ID >= 127 {
                DD = 0;
                ID= ID - 1;
                if MODE==1 {
                    pwm.set_duty(Channel::C4, BREATHING_TABLE[ID]);
                } else {
                    pwm.set_duty(Channel::C4, 1);
                }
                if SPEED == 1 {
                    block!(timer.wait()).unwrap();
                } else if SPEED==2 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                } else if SPEED==3 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                }
            } else if DD==0 && ID > 0 {
                ID = ID-1;
                if MODE==1 {
                    pwm.set_duty(Channel::C4, BREATHING_TABLE[ID]);
                } else {
                    pwm.set_duty(Channel::C4, 480);
                }
                if SPEED == 1 {
                    block!(timer.wait()).unwrap();
                } else if SPEED==2 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                } else if SPEED==3 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                }
            } else if DD==0 && ID<=0{
                DD = 1;
                ID = ID + 1;
                if MODE==1 {
                    pwm.set_duty(Channel::C4, BREATHING_TABLE[ID]);
                } else {
                    pwm.set_duty(Channel::C4, 480);
                }
                if SPEED == 1 {
                    block!(timer.wait()).unwrap();
                } else if SPEED==2 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                } else if SPEED==3 {
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                    block!(timer.wait()).unwrap();
                }
            }
        };


    }
}

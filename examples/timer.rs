//! Test the general purpose timers

//#![deny(warnings)]
#![no_std]
#![no_main]

extern crate panic_semihosting;

use core::cell::RefCell;
use core::fmt::Write;

use cortex_m_rt::entry;
use cortex_m_semihosting::hio;

use cortex_m::peripheral::NVIC;

use stm32f7xx_hal::{
    interrupt, pac,
    prelude::*,
    timer::{Event, Timer},
};

#[entry]
fn main() -> ! {
    let mut hstdout = hio::hstdout().unwrap();
    writeln!(hstdout, "Starting timer...").unwrap();

    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    unsafe {
        NVIC::unmask(pac::Interrupt::TIM2);
    }
    let mut timer = Timer::tim2(dp.TIM2, 1.Hz(), clocks, &mut rcc.apb1).unwrap();
    timer.listen(Event::TimeOut);

    loop {}
}

#[interrupt]
fn TIM2() {
    free(|cs| {
        TIMER
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .clear_interrupt(Event::TimeOut)
    });

    let mut hstdout = hio::hstdout().unwrap();
    writeln!(hstdout, "TIM2 intr!").unwrap();
}

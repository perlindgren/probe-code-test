// ex1.rs
//
// cargo embed --examples ex1
//
// Simple example to show the 1-1 relation between hardware tasks
// and interrupts handlers, and the prioritization done by the
// NVIC (Nested Vector Interrupt Controller)

#![no_std]
#![no_main]

use {
    hal::pac,
    nrf52840_hal as hal, 
    // panic_halt as _,
    panic_rtt_target as _,
    cortex_m::asm,
    rtt_target::{rprintln, rtt_init_print},

};

#[rtic::app(device = nrf52840_hal::pac)]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    // init runs with interrupts disabled
    #[init]
    fn init(_cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        // you can pend a hardware task (bound to interrupt) from init, however
        // it will be scheduled until init is done (but before idle)
        rtic::pend(pac::Interrupt::TIMER1);

        // this task will be executed first since it has higher priority
        rtic::pend(pac::Interrupt::TIMER2);

        (Shared {}, Local {}, init::Monotonics())
    }
    // after init is executed, interrupts are enabled by RTIC

    #[idle()]
    fn idle(_cx: idle::Context) -> ! {
        rprintln!("idle");
        loop {}
    }

    // Hardware task bound to interrupt TIMER0
    // priority is set to 1 by default
    #[task(binds = TIMER1)]
    fn t1(_cx: t1::Context) {
        rprintln!("t1");
        asm::nop();
        panic!("here");
        // asm::bkpt();
    }

    // Hardware task bound to interrupt TIMER0
    // priority is set to 2
    #[task(binds = TIMER2, priority = 2)]
    fn t2(_cx: t2::Context) {
        rprintln!("t2");
        asm::nop();
    }
}

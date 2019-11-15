#![no_main]
#![no_std]

extern crate panic_halt;
// use cortex_m;

use crate::hal::{prelude::*, stm32};
use stm32f4xx_hal as hal;

use cortex_m_semihosting::hprintln;
// use panic_semihosting as _;
use rtfm::cyccnt::{Instant, U32Ext as _};

#[rtfm::app(device = stm32f4xx_hal::stm32, peripherals = true, monotonic = rtfm::cyccnt::CYCCNT )]
const APP: () = {
    #[init(schedule = [hello])]
    fn init(mut cx: init::Context) {
        hprintln!("init").unwrap();

        // Initialize (enable) the monotonic timer (CYCCNT)
        cx.core.DCB.enable_trace();
        // required on devices that software lock the DWT (e.g. STM32F7)
        //unsafe { cx.core.DWT.lar.write(0xC5ACCE55) }
        cx.core.DWT.enable_cycle_counter();
        
        // Set up the LED: it's connected to pin PA5 on the microcontroler
        // Cortex-M peripherals
        //let cp: rtfm::Peripherals = cx.core;

        // Device specific peripherals
        //let dp: stm32::Peripherals = cx.device;

        //et gpioa = dp.GPIOA.split();
        //let mut led = gpioa.pa5.into_push_pull_output();

        // The external LED, on the next pin down:
        //let mut xled = gpioa.pa6.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        //let rcc = dp.RCC.constrain();
        //let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        let now = cx.start; // the start time of the system
        hprintln!("init @ {:?}", now).unwrap();
        
        cx.schedule.hello(now + 4_000_000.cycles()).unwrap();
    }

    #[idle]
    fn idle(cx: idle::Context) -> ! {
        hprintln!("idle").unwrap();
        hprintln!("{:?}", Instant::now()).unwrap();
        loop {}
    }

    #[task]
    fn hello(_: hello::Context) {
        hprintln!("Hello!").unwrap();
    }

    extern "C" {
        fn USART1();
    }
};

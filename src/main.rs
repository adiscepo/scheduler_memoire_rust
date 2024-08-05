// #![no_std]
// #![no_main]

// mod scheduler;
// use cortex_m_rt::exception;
// use cortex_m_semihosting::hprintln;
// use embedded_hal::digital::v2::OutputPin;
// use panic_halt as _;
// use rp_pico::entry;
// // use rp2040_hal::{self as hal, pac};
// use bsp::hal::{pac, sio::Sio};
// use cortex_m::peripheral::syst::SystClkSource;
// use rp_pico as bsp;
// use scheduler::scheduler;

// #[entry]
// fn main() -> ! {
//     let p = cortex_m::Peripherals::take().unwrap();
//     let mut syst = p.SYST;
//     syst.set_clock_source(SystClkSource::Core);
//     syst.set_reload(12_000_000);
//     syst.clear_current();
//     syst.enable_counter();
//     syst.enable_interrupt();
//     unsafe {
//         scheduler.init_scheduler();
//         scheduler.create_process(1000, task_0);
//     };
//     hprintln!("Hello world !").unwrap();
//     // Activer l'interruption
//     // unsafe {
//     //     pac::NVIC::unmask(pac::Interrupt::IO_IRQ_BANK0);
//     // }

//     // hprintln!(
//     //     "Is enabled ? {0}",
//     //     pac::NVIC::is_enabled(pac::Interrupt::IO_IRQ_BANK0)
//     // )
//     // .unwrap();

//     // hprintln!(
//     //     "Is pending ? {0}",
//     //     pac::NVIC::is_pending(pac::Interrupt::IO_IRQ_BANK0)
//     // )
//     // .unwrap();
//     // // Déclencher l'interruption par logiciel
//     // pac::NVIC::pend(pac::Interrupt::IO_IRQ_BANK0);

//     loop {
//         hprintln!("Loop").unwrap();
//     }
// }

// static mut TASK0_VAL: u32 = 0;

// pub fn task_0() {
//     unsafe {
//         // Impression du début de la tâche
//         let mut pac = pac::Peripherals::take().unwrap();
//         let sio = Sio::new(pac.SIO);
//         let pins = bsp::Pins::new(
//             pac.IO_BANK0,
//             pac.PADS_BANK0,
//             sio.gpio_bank0,
//             &mut pac.RESETS,
//         );
//         let mut led_pin = pins.led.into_push_pull_output();
//         led_pin.set_high().unwrap();

//         // Boucle de travail de la tâche
//         while TASK0_VAL < 10_000_000 {
//             TASK0_VAL += 1;
//             if TASK0_VAL % 100_000 == 0 {
//                 hprintln!("Task 0");
//                 // my_print(b"Task 0\n\0".as_ptr());
//             }
//         }
//         led_pin.set_low().unwrap();

//         // Réinitialiser la valeur de la tâche
//         TASK0_VAL = 0;

//         // Impression de la fin de la tâche
//         // my_print(b"F %d %d\n\0".as_ptr(), SCHEDULER.current_process, NOW);
//     }
// }
// #![deny(unsafe_code)]
#![no_main]
#![no_std]

mod scheduler;
use core::arch::asm;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::Peripherals;
use cortex_m_rt::exception;
use cortex_m_semihosting::hprint;
use panic_halt as _;
use rp2040_hal::rom_data::double_funcs::uint64_to_double;
use rp_pico::entry;
use scheduler::scheduler;

extern "C" {
    fn isr_systick();
}

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let mut syst = p.SYST;

    // configures the system timer to trigger a SysTick exception every second
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload((0x00ffffff - 1) as u32);
    syst.enable_counter();
    syst.enable_interrupt();
    unsafe {
        scheduler.init_scheduler();
        scheduler.create_process(2000, task_0);
        // scheduler.create_process(4000, task_1);
        asm!("b start_scheduler", options(noreturn));
        // scheduler::start_scheduler();
    }
    loop {}
}

fn task_0() {
    hprint!("TASK 0 begin").unwrap();
    let mut i: u32 = 0;
    while i < 10000000 {
        i += 1;
        if i % 1000000 == 0 {
            hprint!("TASK0 {}", i).unwrap();
        }
    }
    hprint!("TASK 0 end").unwrap();
}

fn task_1() {
    hprint!("TASK 1 begin").unwrap();
    let mut i: u32 = 0;
    while i < 10000000 {
        i += 1;
        if i % 1000000 == 0 {
            hprint!("TASK0 {}", i).unwrap();
        }
    }
    hprint!("TASK 0 begin").unwrap();
}

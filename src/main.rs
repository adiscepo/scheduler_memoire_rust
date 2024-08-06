#![no_main]
#![no_std]

mod scheduler;
mod timer;
use crate::scheduler::scheduler::{scheduler as Scheduler, start_scheduler};
use cortex_m_semihosting::hprint;
use panic_halt as _;
use rp_pico::entry;

#[entry]
fn main() -> ! {
    unsafe {
        Scheduler.init_scheduler();
        Scheduler.create_process(2000, task_0);
        Scheduler.create_process(2000, task_1);
        start_scheduler();
    }
    loop {}
}

fn task_0() {
    hprint!("TASK 0 begin").unwrap();
    let mut i: u32 = 0;
    while i < 1000000 {
        i += 1;
        if i % 100000 == 0 {
            hprint!("TASK0 {}", i).unwrap();
        }
    }
    hprint!("TASK 0 end").unwrap();
}

fn task_1() {
    hprint!("TASK 1 begin").unwrap();
    let mut i: u32 = 0;
    while i < 1000000 {
        i += 1;
        if i % 100000 == 0 {
            hprint!("TASK1 {}", i).unwrap();
        }
    }
    hprint!("TASK 1 end").unwrap();
}

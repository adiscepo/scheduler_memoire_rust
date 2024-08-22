#![no_main]
#![no_std]

mod scheduler;
mod timer;
use crate::scheduler::scheduler::{scheduler as Scheduler, start_scheduler};
use core::ptr;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
use rp_pico::{
    entry,
    hal::{
        self,
        clocks::{init_clocks_and_plls, ClockSource},
        gpio::{
            bank0::{Gpio10, Gpio11, Gpio12},
            Pin, PushPullOutput,
        },
        Sio, Watchdog,
    },
    pac, XOSC_CRYSTAL_FREQ,
};

// use cortex_m_semihosting::hprint;
use timer::timer::{get_elapsed_time_since_boot, to_ms};

static mut LED_PIN_T0: Option<Pin<Gpio10, PushPullOutput>> = None;
static mut LED_PIN_T1: Option<Pin<Gpio11, PushPullOutput>> = None;
static mut LED_PIN_T2: Option<Pin<Gpio12, PushPullOutput>> = None;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let sio = Sio::new(pac.SIO);

    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Permet de redéfinir la fréquence de la clock. C'est étrange mais parfois le
    // débuggeur (que cela soit par OpenOCD ou Probe-rs) rend le code très très lent.
    clocks.system_clock.get_freq();
    // info!("System clock frequency: {} Hz", sys_clock_freq);

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Initialisation des LED pins, stockées dans des variables statiques
    unsafe {
        LED_PIN_T0 = Some(pins.gpio10.into_push_pull_output());
        LED_PIN_T1 = Some(pins.gpio11.into_push_pull_output());
        LED_PIN_T2 = Some(pins.gpio12.into_push_pull_output());
    }

    unsafe {
        Scheduler.init_scheduler();
        Scheduler.create_process(2500, task0);
        Scheduler.create_process(1000, task1);
        Scheduler.create_process(500, task2);
        info!("Task: {}", Scheduler.current_process);
        start_scheduler();
    }
    loop {}
}

fn task0() {
    unsafe {
        info!(
            "D 0 {}",
            to_ms(get_elapsed_time_since_boot()) - Scheduler.delay
        );
        if let Some(led_pin) = LED_PIN_T0.as_mut() {
            led_pin.set_high().unwrap();
        }
    }

    let mut i: u32 = 0;
    while i < 10_000_000 {
        unsafe {
            ptr::write_volatile(&mut i, i + 1);
        }
        if i % 100_000 == 0 {
            info!("Task 0");
        }
    }

    unsafe {
        if let Some(led_pin) = LED_PIN_T0.as_mut() {
            led_pin.set_low().unwrap();
        }
        info!(
            "F 0 {}",
            to_ms(get_elapsed_time_since_boot()) - Scheduler.delay
        );
    }
}

fn task1() {
    unsafe {
        info!(
            "D 1 {}",
            to_ms(get_elapsed_time_since_boot()) - Scheduler.delay
        );
        if let Some(led_pin) = LED_PIN_T1.as_mut() {
            led_pin.set_high().unwrap();
        }
    }

    let mut i: u32 = 0;
    while i < 2_000_000 {
        unsafe {
            ptr::write_volatile(&mut i, i + 1);
        }
        if i % 100_000 == 0 {
            info!("Task 1");
        }
    }

    unsafe {
        if let Some(led_pin) = LED_PIN_T1.as_mut() {
            led_pin.set_low().unwrap();
        }
        info!(
            "F 1 {}",
            to_ms(get_elapsed_time_since_boot()) - Scheduler.delay
        );
    }
}

fn task2() {
    unsafe {
        info!(
            "D 2 {}",
            to_ms(get_elapsed_time_since_boot()) - Scheduler.delay
        );
        if let Some(led_pin) = LED_PIN_T2.as_mut() {
            led_pin.set_high().unwrap();
        }
    }

    let mut i: u32 = 0;
    while i < 1_500_000 {
        unsafe {
            ptr::write_volatile(&mut i, i + 1);
        }
        if i % 100_000 == 0 {
            info!("Task 2");
        }
    }

    unsafe {
        if let Some(led_pin) = LED_PIN_T2.as_mut() {
            led_pin.set_low().unwrap();
        }
        info!(
            "F 2 {}",
            to_ms(get_elapsed_time_since_boot()) - Scheduler.delay
        );
    }
}

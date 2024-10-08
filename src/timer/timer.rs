// use core::f32::consts;
// use cortex_m_semihosting::hprint;
// use rp_pico::hal::pac;
// use rp_pico::hal::timer::{Alarm, Timer};
// use rp_pico::pac::interrupt;
const TIMER_BASE: u32 = 0x40054000;
// pub fn init_timer(interval_ms: u32) {
//     hprint!("Initialisation du timer").unwrap();
//     let mut pac = pac::Peripherals::take().unwrap();

//     // Création du timer et de l'alarme
//     let mut timer = Timer::new(pac.TIMER, &mut pac.RESETS);
//     let mut alarm0 = timer.alarm_0().unwrap();

//     // Configuration de l'alarme afin qu'elle se déclanche toutes les 1 ms
//     let interval_ticks = fugit::MicrosDurationU32::millis(interval_ms);

//     let _ = alarm0.schedule(interval_ticks);
//     alarm0.enable_interrupt();

//     unsafe {
//         pac::NVIC::unmask(pac::Interrupt::TIMER_IRQ_0);
//         cortex_m::interrupt::enable();
//     }
// }

pub unsafe fn to_ms(elapsed_time_us: u32) -> u32 {
    elapsed_time_us / 1000
}

pub unsafe fn get_elapsed_time_since_boot() -> u32 {
    *((TIMER_BASE | 0xc) as *const u32)
}

// #[interrupt]
// fn TIMER_IRQ_0() {
//     hprint!("Timer appelé").unwrap();
//     unsafe { ELAPSED_MS += 1 };

//     // Efface l'interruption
//     let pac = unsafe { pac::Peripherals::steal() };
//     pac.TIMER.intr.write(|w| w.alarm_0().set_bit());
// }

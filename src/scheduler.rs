use core::{arch::asm, u32};

use cortex_m::asm;
use cortex_m_rt::exception;
use cortex_m_semihosting::{hprintln, nr::READ};
use rp2040_hal::pac;
use rp_pico::{hal::pac::interrupt, pac::ppb::aircr::ENDIANESS_R};

const PROCESS_STACK_SIZE: usize = 1024; // 1Kb
const MAX_PROCESSES: usize = 10;

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum State {
    UNDEFINED,
    DEFINED,
    READY,
    RUNNING,
    PREEMPTED,
    FAILED,
    ENDED,
}

// #[derive(Default)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Process {
    tos: *mut u32,
    stack: [u32; PROCESS_STACK_SIZE],
    deadline: u32,
    absolute_deadline: u32,
    release_time: u32,
    fn_ptr: *mut u32,
    state: State,
}

#[repr(C)]
pub struct Scheduler {
    pub current_process: usize,
    pub processes: [Process; MAX_PROCESSES + 1],
    pub timer: u32,
    pub delay: u32,
}

#[no_mangle] // Permet d'être utilisé en assembleur
pub static mut scheduler: Scheduler = Scheduler {
    current_process: 0,
    processes: [Process {
        tos: core::ptr::null_mut(),
        stack: [0; PROCESS_STACK_SIZE],
        deadline: 0,
        absolute_deadline: 0,
        release_time: 0,
        fn_ptr: core::ptr::null_mut(),
        state: State::UNDEFINED,
    }; MAX_PROCESSES + 1],
    timer: 0,
    delay: 0,
};

extern "C" {
    pub fn end_set_task();
}

impl Scheduler {
    pub unsafe fn init_scheduler(&mut self) {
        cortex_m::interrupt::disable();

        // setup_systick(); // Active Systick
        // pac::NVIC::unmask(pac::Interrupt::IO_IRQ_BANK0); // Active l'interruption de fin de tâche
        // Défini le processus d'idle
        let idle_process = &mut self.processes[MAX_PROCESSES];
        idle_process.stack[PROCESS_STACK_SIZE - 1] = 0x01000000;
        idle_process.stack[PROCESS_STACK_SIZE - 2] = idle as u32;
        idle_process.stack[PROCESS_STACK_SIZE - 3] = end_task as u32;
        idle_process.tos = &mut idle_process.stack[PROCESS_STACK_SIZE - 16];
        idle_process.absolute_deadline = u32::MAX;
        idle_process.deadline = 0;
        idle_process.state = State::UNDEFINED;
        self.current_process = MAX_PROCESSES;

        hprintln!("Scheduler created !").unwrap();
        cortex_m::interrupt::enable();
    }

    pub fn create_process(&mut self, deadline: u32, fn_ptr: unsafe fn()) -> bool {
        let mut available_process: Option<usize> = None;

        // Recherche d'un processus disponible
        for i in 0..MAX_PROCESSES {
            if self.processes[i].state == State::UNDEFINED
                || self.processes[i].state == State::ENDED
            {
                available_process = Some(i);
                break;
            }
        }

        // Si aucun processus disponible n'est trouvé, retourner false
        let available_process = match available_process {
            Some(index) => index,
            None => return false,
        };

        // Création et initialisation du nouveau processus
        let new_process = &mut self.processes[available_process];
        new_process.stack[PROCESS_STACK_SIZE - 1] = 0x01000000; // Défini le xPSR sur Thumb
        new_process.stack[PROCESS_STACK_SIZE - 2] = fn_ptr as u32; // PC de la pile pointe vers le code de la fonction
        new_process.stack[PROCESS_STACK_SIZE - 3] = end_task as u32; // LR : Fonction de retour
        new_process.stack[PROCESS_STACK_SIZE - 4] = 0x12121212; // Permet de contrôler en mémoire lors du débug
        new_process.stack[PROCESS_STACK_SIZE - 5] = 0x33333333;
        new_process.stack[PROCESS_STACK_SIZE - 6] = 0x22222222;
        new_process.stack[PROCESS_STACK_SIZE - 7] = 0x11111111;
        new_process.stack[PROCESS_STACK_SIZE - 8] = 0x00000000;
        new_process.stack[PROCESS_STACK_SIZE - 9] = 0x11111111;
        new_process.stack[PROCESS_STACK_SIZE - 10] = 0x10101010;
        new_process.stack[PROCESS_STACK_SIZE - 11] = 0x99999999;
        new_process.stack[PROCESS_STACK_SIZE - 12] = 0x88888888;
        new_process.stack[PROCESS_STACK_SIZE - 13] = 0xdeadbeef;
        new_process.stack[PROCESS_STACK_SIZE - 14] = 0x66666666;
        new_process.stack[PROCESS_STACK_SIZE - 15] = 0x55555555;
        new_process.stack[PROCESS_STACK_SIZE - 16] = 0x44444444;
        new_process.tos = unsafe { new_process.stack.as_mut_ptr().add(PROCESS_STACK_SIZE - 16) }; // Défini le pointeur vers le haut de la pile

        new_process.absolute_deadline = deadline;
        new_process.deadline = deadline;
        new_process.fn_ptr = fn_ptr as *mut u32;
        new_process.state = State::DEFINED;

        true
    }
}

#[no_mangle]
unsafe fn schedule() -> usize {
    cortex_m::interrupt::disable();
    if scheduler.delay == 0 {
        scheduler.delay = 0; // TODO: modif qd le get_absolute_time fonctionnera
    } else {
        // Print des logs
    }
    let mut pid: usize = MAX_PROCESSES;
    let mut earliest_deadline: u32 = u32::MAX;
    let now = u32::MAX; // TODO: modif idem

    for i in 0..MAX_PROCESSES {
        let process = &mut scheduler.processes[i];

        if process.state == State::UNDEFINED {
            continue;
        }

        if process.state == State::ENDED {
            process.stack[PROCESS_STACK_SIZE - 1] = 0x01000000;
            process.stack[PROCESS_STACK_SIZE - 2] = process.fn_ptr as u32;
            process.stack[PROCESS_STACK_SIZE - 3] = end_task as u32;
            process.stack[PROCESS_STACK_SIZE - 4] = 0x12121212;
            process.stack[PROCESS_STACK_SIZE - 5] = 0x33333333; // R3
            process.stack[PROCESS_STACK_SIZE - 6] = 0x22222222; // R2
            process.stack[PROCESS_STACK_SIZE - 7] = 0x11111111; // R1
            process.stack[PROCESS_STACK_SIZE - 8] = 0x00000000; // R0
            process.stack[PROCESS_STACK_SIZE - 9] = 0x11111111; // R11
            process.stack[PROCESS_STACK_SIZE - 10] = 0x10101010; // R10
            process.stack[PROCESS_STACK_SIZE - 11] = 0x99999999; // R9
            process.stack[PROCESS_STACK_SIZE - 12] = 0x88888888; // R8
            process.stack[PROCESS_STACK_SIZE - 13] = 0x77777777; // R7
            process.stack[PROCESS_STACK_SIZE - 14] = 0x66666666; // R6
            process.stack[PROCESS_STACK_SIZE - 15] = 0x55555555; // R5
            process.stack[PROCESS_STACK_SIZE - 16] = 0x44444444; // R4
            process.tos = process.stack.as_mut_ptr().add(PROCESS_STACK_SIZE - 16);

            process.state = State::DEFINED;
            process.release_time = process.absolute_deadline;
            process.absolute_deadline = process.release_time + process.deadline;
        }

        if process.state == State::DEFINED || process.state == State::FAILED {
            if process.release_time < now {
                process.state = State::READY;
            }
        }

        if process.state == State::RUNNING {
            process.state = State::PREEMPTED;
        }

        if process.state == State::READY
            || process.state == State::RUNNING
            || process.state == State::PREEMPTED
        {
            if process.absolute_deadline < earliest_deadline {
                pid = i;
                earliest_deadline = process.absolute_deadline;
            }
        }
    }
    if pid != MAX_PROCESSES {
        scheduler.processes[pid].state = State::RUNNING;
    }
    scheduler.current_process = pid;
    // if (pid == MAX_PROCESSES) // LOG
    cortex_m::interrupt::enable();
    pid
}

#[no_mangle]
fn irq_set_enabled() {
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::IO_IRQ_BANK0);
    }
}

unsafe fn end_task() {
    cortex_m::interrupt::disable();
    let pid: usize = unsafe { scheduler.current_process };
    let ended_proc = &mut scheduler.processes[pid];
    ended_proc.state = State::ENDED;
    cortex_m::interrupt::enable();
    end_set_task();
    loop {
        asm!("WFI");
    }
}

fn idle() {
    loop {
        asm::nop();
    }
}

#[interrupt]
fn IO_IRQ_BANK0() {
    hprintln!("Interruption").unwrap();
}

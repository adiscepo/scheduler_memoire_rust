use cortex_m::asm;
use cortex_m_rt::exception;
use cortex_m_semihosting::hprintln;
use rp2040_hal::pac;
use rp_pico::hal::pac::interrupt;

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
    pub fn setup_systick();
    pub fn start_scheduler();
    pub fn isr_pendsv();
    pub fn isr_systick();
    pub fn set_process_idle();
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

    pub fn create_process(&mut self, deadline: u32, fn_ptr: fn()) -> bool {
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
        new_process.stack[PROCESS_STACK_SIZE - 13] = 0x77777777;
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
fn schedule() -> usize {
    if unsafe { scheduler.current_process } == 1 {
        1
    } else {
        0
    }
}

#[no_mangle]
fn irq_set_enabled() {
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::IO_IRQ_BANK0);
    }
}

fn end_task() {
    cortex_m::interrupt::disable();
    let _pid: usize = unsafe { scheduler.current_process };
    // let ended_proc = &unsafe { scheduler.processes }[pid];
    unsafe {
        cortex_m::interrupt::enable();
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

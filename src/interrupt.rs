use crate::context::TrapFrame;
use riscv::register::{ stvec, sstatus };
use riscv::register::scause::Trap;
use riscv::register::scause::Exception;
use riscv::register::scause::Interrupt;
use crate::clock::{ TICK, clock_set_next_event };


global_asm!(include_str!("trap/trap.asm"));


#[no_mangle]
pub fn init() {
    extern {
        fn __alltraps();
    }
    unsafe {
        sstatus::set_sie();
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
    }
    println!("++++setup interrupt !++++");
}

#[no_mangle]
pub extern "C" fn rust_trap(tf: &mut TrapFrame) {
    match tf.scause.cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint(),
        Trap::Interrupt(Interrupt::SupervisorTimer) => super_timer(),
        _ => panic!("unexpected trap"),
    }
}

fn breakpoint() {
    panic!("a breakpoint set by kernel");
}

fn super_timer() {
    clock_set_next_event();
    unsafe {
        TICK = TICK + 1;
        if TICK % 100 == 0 {
            println!("100 ticks!");
        }
    }
}
    


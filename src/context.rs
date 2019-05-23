use riscv::register:: {
    sstatus::Sstatus,
    scause::Scause,
};

#[repr(C)]
pub struct TrapFrame {
    pub x: [usize; 32],  // General registers
    pub sstatus: Sstatus,   // Supervisor Status Register
    pub sepc: usize,    // Supervisor exception program counter
    pub stval: usize,   // Supervisor trap value
    pub scause: Scause, // Scause Register: record the cause of exception/ interrrupt/trap
}

impl TrapFrame {
    pub fn increase_spce(self: &mut Self) {
        self.sepc = self.sepc + 4;
    }
}

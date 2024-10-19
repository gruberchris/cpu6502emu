mod cpu;
mod memory;
mod types;

use cpu::{Cpu, INS_LDA_IM, INS_LDA_ZP};
use memory::Memory;

fn main() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    // initialize the memory
    cpu.reset(&mut memory);
    // initialize the clock cycles
    let mut cycles = 2;

    // example inline virtual machine code program to test with
    memory.write(0xFFFC, INS_LDA_IM);
    memory.write(0xFFFD, 0x42);
    cpu.execute(&mut cycles, &mut memory);

    // initialize and run a second inline virtual machine code program
    cpu.reset(&mut memory);
    cycles = 3;

    memory.write(0xFFFC, INS_LDA_ZP);
    memory.write(0xFFFD, 0x42);
    memory.write(0x0042, 0x84);
    cpu.execute(&mut cycles, &mut memory);
}

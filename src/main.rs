mod cpu;
mod memory;
mod types;

use cpu::{Cpu, INSTRUCT_LDA_IM};
use memory::Memory;

fn main() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    let mut cycles = 2;

    cpu.reset(&mut memory);

    // example inline virtual machine code program to test with
    memory.write(0xFFFC, INSTRUCT_LDA_IM);
    memory.write(0xFFFD, 0x42);

    cpu.execute(&mut cycles, &mut memory);
}

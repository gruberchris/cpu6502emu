mod cpu;
mod memory;
mod types;

use cpu::Cpu;
use memory::Memory;

fn main() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.reset(&mut memory);
}

use crate::types::Byte;

// The 6502 has 16-bit address bus, which means it can address 64KB of memory.
const MAX_MEMORY: u32 = 1024 * 64;

pub struct Memory {
    data: [Byte; MAX_MEMORY as usize],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            data: [0; MAX_MEMORY as usize],
        }
    }

    pub fn initialize(&mut self) {
        for i in 0..MAX_MEMORY {
            self.data[i as usize] = 0;
        }
    }

    pub fn write(&mut self, address: u16, data: Byte) {
        self.data[address as usize] = data;
    }

    pub fn read(&self, address: u16) -> Byte {
        self.data[address as usize]
    }
}

use crate::memory::Memory;
use crate::types::{Byte, Word};

pub struct Cpu {
    // program counter 16-bit
    pc: Word,
    // stack pointer 8-bit, can hold values from 0x00 to 0xFF. The stack is located in the first
    // 256 bytes of memory (0x0100 to 0x01FF)
    sp: Byte,

    // registers
    a: Byte, // accumulator register
    x: Byte, // index register
    y: Byte, // index register

    // 7-bit status register P
    c: Byte, // carry flag
    z: Byte, // zero flag
    i: Byte, // interrupt disable flag
    d: Byte, // decimal mode flag
    b: Byte, // break command flag
    v: Byte, // overflow flag
    n: Byte, // negative flag
}

pub const INS_LDA_IM: Byte = 0xA9; // LDA Immediate 2 bytes, 2 cycles
pub const INS_LDA_ZP: Byte = 0xA5; // LDA Zero Page 2 bytes, 3 cycles
pub const INS_LDA_ZPX: Byte = 0xB5; // LDA Zero Page X 2 bytes, 4 cycles
pub const INS_LDA_ABS: Byte = 0xAD; // LDA Absolute 3 bytes, 4 cycles
pub const INS_LDA_ABX: Byte = 0xBD; // LDA Absolute X 3 bytes, 4+ cycles (add 1 if page crossed)
pub const INS_LDA_ABY: Byte = 0xB9; // LDA Absolute Y 3 bytes, 4+ cycles (add 1 if page crossed)
pub const INS_LDA_IDX: Byte = 0xA1; // LDA Indirect X 2 bytes, 6 cycles
pub const INS_LDA_IDY: Byte = 0xB1; // LDA Indirect Y 2 bytes, 5+ cycles (add 1 if page crossed)

pub const INS_JSR: Byte = 0x20; // JSRt Absolute 3 bytes, 6 cycles

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
            c: 0,
            z: 0,
            i: 0,
            d: 0,
            b: 0,
            v: 0,
            n: 0,
        }
    }

    pub fn reset(&mut self, memory: &mut Memory) {
        self.pc = 0xFFFC;
        self.sp = 0xFF;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.c = 0;
        self.z = 0;
        self.i = 0;
        self.d = 0;
        self.b = 0;
        self.v = 0;
        self.n = 0;

        memory.initialize();
    }

    pub fn fetch_byte(&mut self, cycles: &mut u32, memory: &Memory) -> Byte {
        let data = memory.read(self.pc);
        self.pc += 1; // a panic here could mean the program ran with too many cycles
        *cycles -= 1;
        data
    }

    pub fn read_byte(&mut self, cycles: &mut u32, address: Byte, memory: &Memory) -> Byte {
        let data = memory.read(address as u16);
        self.pc += 1;
        *cycles -= 1; // a panic here could mean the program ran with too few cycles
        data
    }

    pub fn fetch_word(&mut self, cycles: &mut u32, memory: &Memory) -> Word {
        // 6502 is little endian
        // read the low byte first, then shift the high byte by 8 bits and OR it with the low byte
        let data =
            memory.read(self.pc as u16) as Word | ((memory.read(self.pc as u16 + 1) as Word) << 8);
        self.pc += 2;
        *cycles -= 2;
        data
    }

    pub fn read_word(&mut self, cycles: &mut u32, address: Word, memory: &Memory) -> Word {
        // 6502 is little endian
        // read the low byte first, then shift the high byte by 8 bits and OR it with the low byte
        let data = memory.read(address) as Word | ((memory.read(address + 1) as Word) << 8);
        self.pc += 2;
        *cycles -= 2;
        data
    }

    pub fn write_word(&mut self, cycles: &mut u32, address: Word, data: Word, memory: &mut Memory) {
        memory.write(address, data as Byte);
        memory.write(address + 1, (data >> 8) as Byte);
        self.pc += 2;
        *cycles -= 2;
    }

    pub fn execute(&mut self, cycles: &mut u32, memory: &mut Memory) {
        while *cycles > 0 {
            // one clock cycle to fetch the instruction
            let instruction: Byte = self.fetch_byte(cycles, memory);
            match instruction {
                INS_LDA_IM => {
                    // one clock cycle to fetch the byte
                    self.a = self.fetch_byte(cycles, memory);
                    self.z = if self.a == 0 { 1 } else { 0 };
                    self.n = if self.a & 0b10000000 > 0 { 1 } else { 0 };
                    println!("LDA Immediate: {:#X}", self.a);
                }
                INS_LDA_ZP => {
                    // one clock cycle to fetch the zero page address
                    let zero_page_address = self.fetch_byte(cycles, memory);
                    // one clock cycle to read the byte from the zero page address
                    self.a = self.read_byte(cycles, zero_page_address, memory);
                    self.z = if self.a == 0 { 1 } else { 0 };
                    self.n = if self.a & 0b10000000 > 0 { 1 } else { 0 };
                    println!("LDA Zero Page: {:#X}", self.a);
                }
                INS_LDA_ZPX => {
                    // one clock cycle to fetch the zero page address
                    let mut zero_page_address = self.fetch_byte(cycles, memory);
                    // one clock cycle to increment the zero page address by the x register, wrapping around on overflow
                    zero_page_address = zero_page_address.wrapping_add(self.x);
                    *cycles -= 1;
                    // one clock cycle to set the a register to the value at the zero page address
                    self.a = self.read_byte(cycles, zero_page_address, memory);
                    self.z = if self.a == 0 { 1 } else { 0 };
                    self.n = if self.a & 0b10000000 > 0 { 1 } else { 0 };
                    println!("LDA Zero Page X: {:#X}", self.a);
                }
                INS_LDA_ABS => {
                    let low_byte = self.fetch_byte(cycles, memory);
                    let high_byte = self.fetch_byte(cycles, memory);
                    let address = (high_byte << 4) | low_byte;
                    self.a = self.read_byte(cycles, address, memory);
                    self.z = if self.a == 0 { 1 } else { 0 };
                    self.n = if self.a & 0b10000000 > 0 { 1 } else { 0 };
                    println!("LDA Absolute: {:#X}", self.a);
                }
                INS_LDA_ABX => {
                    let base_address = self.fetch_word(cycles, memory);
                    let address = base_address.wrapping_add(self.x as Word);

                    // Check if a page boundary is crossed
                    if (base_address & 0xFF00) != (address & 0xFF00) {
                        *cycles -= 1; // Add an extra cycle if page boundary is crossed
                    }

                    self.a = self.read_byte(cycles, address as Byte, memory);
                    self.z = if self.a == 0 { 1 } else { 0 };
                    self.n = if self.a & 0b10000000 > 0 { 1 } else { 0 };
                    println!("LDA Absolute X: {:#X}", self.a);
                }
                INS_LDA_ABY => {
                    let base_address = self.fetch_word(cycles, memory);
                    let address = base_address.wrapping_add(self.y as Word);

                    // Check if a page boundary is crossed
                    if (base_address & 0xFF00) != (address & 0xFF00) {
                        *cycles -= 1; // Add an extra cycle if page boundary is crossed
                    }

                    self.a = self.read_byte(cycles, address as Byte, memory);
                    self.z = if self.a == 0 { 1 } else { 0 };
                    self.n = if self.a & 0b10000000 > 0 { 1 } else { 0 };
                    println!("LDA Absolute Y: {:#X}", self.a);
                }
                INS_LDA_IDX => {
                    // Fetch the zero-page address
                    let zero_page_address = self.fetch_byte(cycles, memory);

                    // Add the X register to the zero-page address (with wrapping)
                    let indexed_address = zero_page_address.wrapping_add(self.x);
                    *cycles -= 1; // 1 cycle for the addition

                    // Fetch the effective address from the zero-page address
                    let address = self.read_word(cycles, indexed_address as Word, memory);
                    *cycles -= 1; // 1 cycle for reading the second byte of the address

                    // Read the byte from the effective address
                    self.a = self.read_byte(cycles, address as Byte, memory);

                    // Update the zero and negative flags
                    self.z = if self.a == 0 { 1 } else { 0 };
                    self.n = if self.a & 0b10000000 > 0 { 1 } else { 0 };
                    println!("LDA Indirect X: {:#X}", self.a);
                }
                INS_LDA_IDY => {
                    // Fetch the zero-page address
                    let zero_page_address = self.fetch_byte(cycles, memory);

                    // Fetch the effective address from the zero-page address
                    let base_address = self.read_word(cycles, zero_page_address as Word, memory);

                    // Add the Y register to the base address
                    let address = base_address.wrapping_add(self.y as Word);

                    // Check if a page boundary is crossed
                    if (base_address & 0xFF00) != (address & 0xFF00) {
                        *cycles -= 1; // Add an extra cycle if page boundary is crossed
                    }

                    // Read the byte from the effective address
                    self.a = self.read_byte(cycles, address as Byte, memory);

                    // Update the zero and negative flags
                    self.z = if self.a == 0 { 1 } else { 0 };
                    self.n = if self.a & 0b10000000 > 0 { 1 } else { 0 };
                    println!("LDA Indirect Y: {:#X}", self.a);
                }
                INS_JSR => {
                    // Fetch the low byte of the address
                    let low_byte = self.fetch_byte(cycles, memory);
                    // Fetch the high byte of the address
                    let high_byte = self.fetch_byte(cycles, memory);
                    // Combine the bytes to form the address
                    let address = (high_byte as Word) << 8 | (low_byte as Word);

                    // Calculate the return address (current PC - 1)
                    let return_address = self.pc.wrapping_sub(1);

                    // Push the return address onto the stack
                    self.sp = self.sp.wrapping_sub(1);
                    self.write_word(cycles, 0x0100 + self.sp as Word, return_address, memory);

                    // Set the program counter to the new address
                    self.pc = address;

                    // Adjust cycles to ensure the instruction requires 6 cycles
                    *cycles -= 2;

                    println!("JSR: {:#X}", address);
                }
                _ => {
                    println!("Instruction not handled: {:#X}", instruction);
                }
            }
        }
    }
}

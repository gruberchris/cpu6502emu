use crate::memory::Memory;
use crate::types::{Byte, Word};

pub struct Cpu {
    // program counter 16-bit
    pc: Word,
    // stack pointer 8-bit
    sp: Word,

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
        self.sp = 0x100;
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

    pub fn read_byte(&mut self, cycles: &mut u32, memory: &Memory) -> Byte {
        let data = memory.read(self.pc);
        self.pc += 1;
        *cycles -= 1;
        data
    }

    pub fn execute(&mut self, cycles: &mut u32, memory: &mut Memory) {
        while *cycles > 0 {
            let instruction: Byte = self.read_byte(cycles, memory);
            match instruction {
                INS_LDA_IM => {
                    self.a = self.read_byte(cycles, memory);
                    self.z = if self.a == 0 { 1 } else { 0 };
                    self.n = if self.a & 0b10000000 > 0 { 1 } else { 0 };
                    println!("LDA Immediate: {:#X}", self.a);
                }
                _ => {
                    println!("Instruction not handled: {:#X}", instruction);
                }
            }
        }
    }
}

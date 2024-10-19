use crate::memory::Memory;
use crate::types::{Byte, Word};

pub struct Cpu {
    // program counter
    pc: Word,
    // stack pointer
    sp: Word,

    // registers
    a: Byte,
    x: Byte,
    y: Byte,

    c: Byte, // carry flag
    z: Byte, // zero flag
    i: Byte, // interrupt disable flag
    d: Byte, // decimal mode flag
    b: Byte, // break command flag
    v: Byte, // overflow flag
    n: Byte, // negative flag
}

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
        self.sp = 0x0100;
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
}

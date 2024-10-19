# My MOS Technology 6502 CPU Emulator

This project is a software emulation of the [MOS Technology 6502](https://en.wikipedia.org/wiki/MOS_Technology_6502) CPU. The 6502 was a popular 8-bit microprocessor in the 1970s and 1980s, and was used in many home computers and video game consoles of that era. The 6502 was known for its simplicity and low cost, and was used in systems such as the Apple II, the Commodore 64, and the Nintendo Entertainment System.

The 6502 CPU is a simple, 8-bit processor with a 16-bit address bus. It has a small set of registers, including an 8-bit accumulator, an 8-bit X register, and an 8-bit Y register. The 6502 has a small set of instructions, including arithmetic, logical, and branch instructions. The 6502 also has a small set of addressing modes, including zero page, absolute, and indirect addressing.

This emulator is created using the Rust programming language.

## Getting Started

To build and run the emulator, you will need to have the Rust programming language installed on your system. You can install Rust by following the instructions on the [Rust website](https://www.rust-lang.org/).

Once you have Rust installed, you can build and run the emulator by running the following commands:

```bash
$ git clone https://github.com/gruberchris/cpu6502emu.git
$ cd cpu6502emu
$ cargo run
```

This will build and run the emulator, which will load a simple test program written in virtual machine code and execute it.

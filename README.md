# Rust-Boy
Rust Boy is an emulator for the Gameboy (Color) written in rust.  It is being designed to run on windows and on custom hardware powered by a cortex-m microcontroller.

This is my second ever Rust project.  Rust Boy's primary goal is to help me become more familiar with Rust, and to learn how to use rust in a #![no_std] environment.  

Once a simple game such as Tetris works with Rust Boy running on Windows then the work to run Rust Boy on a cortex-m will begin.  

## Cpu Test Checklist
A checklist of all Blargg CPU tests as they are tested and pass.

- [X] 01-special
- [ ] 02-interrupts
- [X] 03-op sp,hl
- [X] 04-op r,imm
- [X] 05-op rp
- [X] 06-ld r,r
- [X] 07-jr,jp,call,ret,rst
- [X] 08-misc instrs
- [X] 09-op r,r
- [X] 10-bit ops
- [X] 11-op a,(hl)

## MBC Checklist
A check list of the implemented Memory Bank Controllers
- [X] MBC 0
- [ ] MBC 1
- [ ] MBC 2
- [ ] MBC 3
- [ ] MBC 5
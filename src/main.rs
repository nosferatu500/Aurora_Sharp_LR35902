use std::env::args;

mod cpu;
mod interconnect;
mod rom;
mod opcode;
mod wram;
mod echo;

use cpu::Cpu;
use interconnect::Interconnect;
use rom::Rom;

fn main() {
    let rom_file = args().nth(1).unwrap();

    let rom = Rom::new(&rom_file).unwrap();

    let inter = Interconnect::new(rom);

    let mut cpu = Cpu::new(inter);

    let mut i = 0;
    loop {
        println!("#{}", i);
        cpu.run_next_instruction();
        i += 1;
    }
}

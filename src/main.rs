use std::env::args;

mod cpu;
mod interconnect;
mod rom;
mod opcode;
mod wram;
mod echo;
mod hram;
mod eram;
mod sdt;
mod timer;

use cpu::Cpu;
use interconnect::Interconnect;
use rom::Rom;

fn main() {
    let rom_file = args().nth(1).unwrap();

    let rom = Rom::new(&rom_file).unwrap();

    let inter = Interconnect::new(rom);

    let mut cpu = Cpu::new(inter);

    cpu.power_up();

    loop {
        let time = cpu.cycle();
        cpu.interconnect.cycle(time);
    }
}

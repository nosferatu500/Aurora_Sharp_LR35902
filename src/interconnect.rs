use rom::Rom;
use wram::Wram;
use echo::Echo;
use hram::Hram;

mod map {
    pub struct Range(u16, u16);

    impl Range {
        pub fn contains(self, addr: u16) -> Option<u16> {
            let Range(start, end) = self;

            if addr >= start && addr < start + (end - start + 0x1) {
                Some(addr - start)
            } else {
                None
            }
        }
    }

    pub const ROM: Range = Range(0x0000, 0x3FFF);
    pub const SROM: Range = Range(0x4000, 0x7FFF);
    pub const VRAM: Range = Range(0x8000, 0x9FFF);
    pub const ERAM: Range = Range(0xA000, 0xBFFF);
    pub const WRAM: Range = Range(0xC000, 0xDFFF);
    pub const ECHO: Range = Range(0xE000, 0xFDFF);
    pub const OAM: Range = Range(0xFE00, 0xFE9F);
    pub const NV: Range = Range(0xFEA0, 0xFEFF);
    pub const IO: Range = Range(0xFF00, 0xFF7F);
    pub const HRAM: Range = Range(0xFF80, 0xFFFE);
}

pub struct Interconnect {
    rom: Rom,
    wram: Wram,
    echo: Echo,
    hram: Hram,
}

impl Interconnect {
    pub fn new(rom: Rom) -> Interconnect {
        Interconnect {
            rom,
            wram: Wram::new(),
            echo: Echo::new(),
            hram: Hram::new(),
        }
    }

    pub fn load8(&self, addr: u16) -> u8 {
        if let Some(offset) = map::ROM.contains(addr) {
            return self.rom.load8(offset);
        }

        panic!("Unhandled load 8bit address {:#x}", addr);
    }

    pub fn load16(&self, addr: u16) -> u16 {
        if let Some(offset) = map::ROM.contains(addr) {
            return self.rom.load16(offset);
        }

        if let Some(offset) = map::HRAM.contains(addr) {
            return self.hram.load16(offset);
        }

        panic!("Unhandled load 16bit address {:#x}", addr);
    }

    pub fn store8(&mut self, addr: u16, value: u8) {
        if let Some(offset) = map::WRAM.contains(addr) {
            if offset >= 0xC000 && offset < 0xDE01 {
                let addr = offset + 0x2000;
                self.echo.store8(addr, value)
            }
            return self.wram.store8(offset, value);
        }

        if let Some(offset) = map::ECHO.contains(addr) {
            if offset >= 0xE000 && offset < 0xFE01 {
                let addr = offset - 0x2000;
                self.wram.store8(addr, value)
            }
            return self.echo.store8(offset, value);
        }

        panic!("Unhandled store 8bit address {:#x}", addr);
    }
}

use rom::Rom;
use wram::Wram;
use echo::Echo;
use hram::Hram;
use eram::Eram;
use sdt::Sdt;
use timer::Timer;

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
    eram: Eram,
    sdt: Sdt,
    timer: Timer,

    pub interrupt_enable: u8,
    pub interrupt_flag: u8,
}

impl Interconnect {
    pub fn new(rom: Rom) -> Interconnect {
        Interconnect {
            rom,
            wram: Wram::new(),
            echo: Echo::new(),
            hram: Hram::new(),
            eram: Eram::new(),
            sdt: Sdt::new(),
            timer: Timer::new(),

            interrupt_enable: 0,
            interrupt_flag: 0,
        }
    }

    pub fn cycle(&mut self, ticks: u32) {
		self.timer.cycle(ticks);

		self.interrupt_flag |= self.timer.interrupt;
		
        self.timer.interrupt = 0;
	}

    pub fn load8(&self, addr: u16) -> u8 {
        if let Some(offset) = map::ROM.contains(addr) {
            return self.rom.load8(offset);
        }

        if let Some(offset) = map::HRAM.contains(addr) {
            return self.hram.load8(offset);
        }

        if let Some(offset) = map::ERAM.contains(addr) {
            return self.eram.load8(offset);
        }

        if let Some(offset) = map::IO.contains(addr) {
            match addr {
                0xFF01 => return self.sdt.rb(addr),
                0xFF02 => return self.sdt.rb(addr),
                0xFF04 => return self.timer.rb(addr),
                0xFF05 => return self.timer.rb(addr),
                0xFF06 => return self.timer.rb(addr),
                0xFF07 => return self.timer.rb(addr),
                _ => println!("Load IO part not implemented addr: {:#x} offset: {:#x}", addr, offset),
            };
            return 0;
        }

        if 0xFF0F == addr {
            return self.interrupt_flag;
        }

        if 0xFFFF == addr {
            return self.interrupt_enable;
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

        if let Some(offset) = map::ERAM.contains(addr) {
            return self.eram.load16(offset);
        }

        if let Some(offset) = map::WRAM.contains(addr) {
            return self.wram.load16(offset);
        }

        if let Some(offset) = map::ECHO.contains(addr) {
            return self.echo.load16(offset);
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

        if let Some(offset) = map::ERAM.contains(addr) {
            return self.eram.store8(offset, value);
        }

        if let Some(offset) = map::IO.contains(addr) {
            match addr {
                0xFF01 => { return self.sdt.wb(addr, value); },
                0xFF02 => { return self.sdt.wb(addr, value); },
                0xFF04 => { return self.timer.wb(addr, value); },
                0xFF05 => { return self.timer.wb(addr, value); },
                0xFF06 => { return self.timer.wb(addr, value); },
                0xFF07 => { return self.timer.wb(addr, value); },
                _ => println!("Store IO part not implemented addr: {:#x} offset: {:#x}", addr, offset),
            }
            return;
        }

        if 0xFF0F == addr {
            return self.interrupt_flag = value;
        }

        if 0xFFFF == addr {
            return self.interrupt_enable = value;
        }

        panic!("Unhandled store 8bit address {:#x}", addr);
    }
}

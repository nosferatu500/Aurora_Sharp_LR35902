use interconnect::Interconnect;
use opcode::Opcode;

struct Clock {
    m: u16,
    t: u16,
}

impl Clock {
    pub fn new() -> Clock {
        Clock { m: 0, t: 0 }
    }
}

#[derive(Clone, Copy)]
struct Flag {
    z: bool, // 7
    n: bool, // 6
    h: bool, // 5
    c: bool, // 4
}

impl Flag {
    pub fn init() -> Flag {
        Flag {
            z: false,
            n: false,
            h: false,
            c: false,
        }
    }
}

#[derive(Clone, Copy)]
struct Register {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,

    pc: u16,
    sp: u16,

    flag: Flag,
}

impl Register {
    pub fn new() -> Register {
        Register {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xd8,
            f: 0xb0,
            h: 0x01,
            l: 0x4d,

            pc: 0x0100,
            sp: 0xfffe,

            flag: Flag::init(),
        }
    }

    pub fn af(&self) -> u16 {
        (self.a as u16) << 8 | (self.f as u16)
    }

    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | (self.c as u16)
    }

    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | (self.e as u16)
    }

    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | (self.l as u16)
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0x00FF) as u8;
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }
}

pub struct Cpu {
    regs: [u16; 16],

    current_pc: u16,

    interconnect: Interconnect,

    register: Register,

    clock: Clock,

    op: Opcode,
}

impl Cpu {
    pub fn new(interconnect: Interconnect) -> Cpu {
        let pc = 0x0100;
        Cpu {
            regs: [0; 16],

            current_pc: pc,

            interconnect,

            register: Register::new(),

            clock: Clock::new(),

            op: Opcode::jp_nn,
        }
    }

    fn store16(&mut self, addr: u16, value: u16) {
        self.interconnect.store8(addr, (value & 0xff) as u8);
        self.interconnect.store8(addr + 1, (value >> 8) as u8);
    }

    fn load16(&mut self, addr: u16) -> u16 {
        let lhs = self.interconnect.load8(addr) as u16;
        let rhs = (self.interconnect.load8(addr + 1) as u16) << 8;

        return lhs | rhs;
    }

    pub fn power_up(&mut self) {
        self.store16(0xFF05, 0x00);
        self.store16(0xFF06, 0x00);
        self.store16(0xFF07, 0x00);
        self.store16(0xFF10, 0x80);
        self.store16(0xFF11, 0xBF);
        self.store16(0xFF12, 0xF3);
        self.store16(0xFF14, 0xBF);
        self.store16(0xFF16, 0x3F);
        self.store16(0xFF17, 0x00);
        self.store16(0xFF19, 0xBF);
        self.store16(0xFF1A, 0x7F);
        self.store16(0xFF1B, 0xFF);
        self.store16(0xFF1C, 0x9F);
        self.store16(0xFF1E, 0xBF);
        self.store16(0xFF20, 0xFF);
        self.store16(0xFF21, 0x00);
        self.store16(0xFF22, 0x00);
        self.store16(0xFF23, 0xBF);
        self.store16(0xFF24, 0x77);
        self.store16(0xFF25, 0xF3);
        self.store16(0xFF26, 0xF1);
        self.store16(0xFF40, 0x91);
        self.store16(0xFF42, 0x00);
        self.store16(0xFF43, 0x00);
        self.store16(0xFF45, 0x00);
        self.store16(0xFF47, 0xFC);
        self.store16(0xFF48, 0xFF);
        self.store16(0xFF49, 0xFF);
        self.store16(0xFF4A, 0x00);
        self.store16(0xFF4B, 0x00);
    }

    fn get_regs(&self, index: u16) -> u16 {
        self.regs[index as usize]
    }

    fn set_regs(&mut self, index: u16, value: u16) {
        self.regs[index as usize] = value;
    }

    pub fn run_next_instruction(&mut self) {
        let instruction = self.interconnect.load8(self.register.pc);

        self.current_pc = self.register.pc;

        let pc = self.register.pc;

        self.register.pc = pc.wrapping_add(1);

        self.decode(instruction);
    }

    fn decode(&mut self, instruction: u8) {
        let value = instruction & 0xff;
        let opcode = self.op.find(value);

        println!("Opcode: {:#x}", value);

        match opcode {
            Opcode::nop => return,

            Opcode::ld_bc_nn => {
                let addr = self.register.pc;
                let nn = self.load16(addr);

                self.register.pc = self.register.pc.wrapping_add(2);

                self.register.set_bc(nn);
                return;
            }

            Opcode::ld_de_nn => {
                let addr = self.register.pc;
                let nn = self.load16(addr);

                self.register.pc = self.register.pc.wrapping_add(2);

                self.register.set_de(nn);
                return;
            }

            Opcode::ld_hl_nn => {
                let addr = self.register.pc;
                let nn = self.load16(addr);

                self.register.pc = self.register.pc.wrapping_add(2);

                self.register.set_hl(nn);
                return;
            }

            Opcode::ld_sp_nn => {
                let addr = self.register.pc;
                let nn = self.load16(addr);

                self.register.pc = self.register.pc.wrapping_add(2);

                self.register.sp = nn;
                return;
            }

            Opcode::ld_a_a => return,
            Opcode::ld_a_b => {
                self.register.a = self.register.b;
                return;
            }
            Opcode::ld_a_c => {
                self.register.a = self.register.c;
                return;
            }
            Opcode::ld_a_d => {
                self.register.a = self.register.d;
                return;
            }
            Opcode::ld_a_e => {
                self.register.a = self.register.e;
                return;
            }
            Opcode::ld_a_h => {
                self.register.a = self.register.h;
                return;
            }
            Opcode::ld_a_l => {
                self.register.a = self.register.l;
                return;
            }
            Opcode::ld_a_bc => {
                let value = self.interconnect.load8(self.register.bc());
                self.register.a = value;
                return;
            }
            Opcode::ld_a_de => {
                let value = self.interconnect.load8(self.register.de());
                self.register.a = value;
                return;
            }
            Opcode::ld_a_nn => {
                //TODO: Probably incorrect.
                let value = self.interconnect.load16(self.register.pc);
                self.register.a = self.interconnect.load8(value);
                self.register.pc = self.register.pc.wrapping_add(2);
                return;
            }
            Opcode::ld_a_sharp => {
                let value = self.interconnect.load8(self.register.pc);
                self.register.a = value;
                self.register.pc = self.register.pc.wrapping_add(1);
                return;
            }
            Opcode::ld_a_hl => {
                let addr = self.register.hl();
                self.register.a = self.interconnect.load8(addr);
                return;
            }
            Opcode::ld_b_b => {
                self.register.b = self.register.b;
                return;
            }
            Opcode::ld_b_c => {
                self.register.b = self.register.c;
                return;
            }
            Opcode::ld_b_d => {
                self.register.b = self.register.d;
                return;
            }
            Opcode::ld_b_e => {
                self.register.b = self.register.e;
                return;
            }
            Opcode::ld_b_h => {
                self.register.b = self.register.h;
                return;
            }
            Opcode::ld_b_l => {
                self.register.b = self.register.l;
                return;
            }
            Opcode::ld_b_hl => {
                let addr = self.register.hl();
                self.register.b = self.interconnect.load8(addr);
                return;
            }
            Opcode::ld_c_b => {
                self.register.c = self.register.b;
                return;
            }
            Opcode::ld_c_c => {
                self.register.c = self.register.c;
                return;
            }
            Opcode::ld_c_d => {
                self.register.c = self.register.d;
                return;
            }
            Opcode::ld_c_e => {
                self.register.c = self.register.e;
                return;
            }
            Opcode::ld_c_h => {
                self.register.c = self.register.h;
                return;
            }
            Opcode::ld_c_l => {
                self.register.c = self.register.l;
                return;
            }
            Opcode::ld_c_hl => {
                let addr = self.register.hl();
                self.register.c = self.interconnect.load8(addr);
                return;
            }
            Opcode::ld_d_b => {
                self.register.d = self.register.b;
                return;
            }
            Opcode::ld_d_c => {
                self.register.d = self.register.c;
                return;
            }
            Opcode::ld_d_d => {
                self.register.d = self.register.d;
                return;
            }
            Opcode::ld_d_e => {
                self.register.d = self.register.e;
                return;
            }
            Opcode::ld_d_h => {
                self.register.d = self.register.h;
                return;
            }
            Opcode::ld_d_l => {
                self.register.d = self.register.l;
                return;
            }
            Opcode::ld_d_hl => {
                let addr = self.register.hl();
                self.register.d = self.interconnect.load8(addr);
                return;
            }
            Opcode::ld_e_b => {
                self.register.e = self.register.b;
                return;
            }
            Opcode::ld_e_c => {
                self.register.e = self.register.c;
                return;
            }
            Opcode::ld_e_d => {
                self.register.e = self.register.d;
                return;
            }
            Opcode::ld_e_e => {
                self.register.e = self.register.e;
                return;
            }
            Opcode::ld_e_h => {
                self.register.e = self.register.h;
                return;
            }
            Opcode::ld_e_l => {
                self.register.e = self.register.l;
                return;
            }
            Opcode::ld_e_hl => {
                let addr = self.register.hl();
                self.register.e = self.interconnect.load8(addr);
                return;
            }
            Opcode::ld_h_b => {
                self.register.h = self.register.b;
                return;
            }
            Opcode::ld_h_c => {
                self.register.h = self.register.c;
                return;
            }
            Opcode::ld_h_d => {
                self.register.h = self.register.d;
                return;
            }
            Opcode::ld_h_e => {
                self.register.h = self.register.e;
                return;
            }
            Opcode::ld_h_h => {
                self.register.h = self.register.h;
                return;
            }
            Opcode::ld_h_l => {
                self.register.h = self.register.l;
                return;
            }
            Opcode::ld_h_hl => {
                let addr = self.register.hl();
                self.register.h = self.interconnect.load8(addr);
                return;
            }
            Opcode::ld_l_b => {
                self.register.l = self.register.b;
                return;
            }
            Opcode::ld_l_c => {
                self.register.l = self.register.c;
                return;
            }
            Opcode::ld_l_d => {
                self.register.l = self.register.d;
                return;
            }
            Opcode::ld_l_e => {
                self.register.l = self.register.e;
                return;
            }
            Opcode::ld_l_h => {
                self.register.l = self.register.h;
                return;
            }
            Opcode::ld_l_l => {
                self.register.l = self.register.l;
                return;
            }
            Opcode::ld_l_hl => {
                let addr = self.register.hl();
                self.register.l = self.interconnect.load8(addr);
                return;
            }
            Opcode::ld_hl_b => {
                let value = self.register.b;
                let addr = self.register.hl();
                self.interconnect.store8(addr, value);
                return;
            }
            Opcode::ld_hl_c => {
                let value = self.register.c;
                let addr = self.register.hl();
                self.interconnect.store8(addr, value);
                return;
            }
            Opcode::ld_hl_d => {
                let value = self.register.d;
                let addr = self.register.hl();
                self.interconnect.store8(addr, value);
                return;
            }
            Opcode::ld_hl_e => {
                let value = self.register.e;
                let addr = self.register.hl();
                self.interconnect.store8(addr, value);
                return;
            }
            Opcode::ld_hl_h => {
                let value = self.register.h;
                let addr = self.register.hl();
                self.interconnect.store8(addr, value);
                return;
            }
            Opcode::ld_hl_l => {
                let value = self.register.l;
                let addr = self.register.hl();
                self.interconnect.store8(addr, value);
                return;
            }
            Opcode::ld_hl_n => {
                let value = self.interconnect.load8(self.current_pc);
                let addr = self.register.hl();
                self.interconnect.store8(addr, value);
                return;
            }

            Opcode::ld_b_a => {
                self.register.b = self.register.a;
                return;
            }

            Opcode::ld_c_a => {
                self.register.c = self.register.a;
                return;
            }

            Opcode::ld_d_a => {
                self.register.d = self.register.a;
                return;
            }

            Opcode::ld_e_a => {
                self.register.e = self.register.a;
                return;
            }

            Opcode::ld_h_a => {
                self.register.h = self.register.a;
                return;
            }

            Opcode::ld_l_a => {
                self.register.l = self.register.a;
                return;
            }

            Opcode::ld_bc_a => {
                let value = self.register.a;
                self.register.set_bc(value as u16);
                return;
            }

            Opcode::ld_de_a => {
                let value = self.register.a;
                self.register.set_de(value as u16);
                return;
            }

            Opcode::ld_hl_a => {
                let value = self.register.a;
                let addr = self.register.hl();
                self.interconnect.store8(addr, value);
                return;
            }

            Opcode::ld_nn_a => {
                let addr = self.interconnect.load16(self.current_pc);
                let value = self.register.a;
                self.interconnect.store8(addr, value);
                return;
            }

            Opcode::add_a_a => {
                let res = self.register.a.wrapping_add(self.register.a);

                self.register.flag.z = res == 0;
                self.register.flag.n = false;
                self.register.flag.h = (self.register.a & 0xF) + (self.register.a & 0xF) > 0xF;
                self.register.flag.c = (self.register.a as u16) + (self.register.a as u16) > 0xF;

                self.register.a = res;
                return;
            }

            Opcode::add_a_b => {
                let res = self.register.a.wrapping_add(self.register.b);

                self.register.flag.z = res == 0;
                self.register.flag.n = false;
                self.register.flag.h = (self.register.a & 0xF) + (self.register.b & 0xF) > 0xF;
                self.register.flag.c = (self.register.a as u16) + (self.register.b as u16) > 0xF;

                self.register.a = res;
                return;
            }

            Opcode::add_a_c => {
                let res = self.register.a.wrapping_add(self.register.c);

                self.register.flag.z = res == 0;
                self.register.flag.n = false;
                self.register.flag.h = (self.register.a & 0xF) + (self.register.c & 0xF) > 0xF;
                self.register.flag.c = (self.register.a as u16) + (self.register.c as u16) > 0xF;

                self.register.a = res;
                return;
            }

            Opcode::add_a_d => {
                let res = self.register.a.wrapping_add(self.register.d);

                self.register.flag.z = res == 0;
                self.register.flag.n = false;
                self.register.flag.h = (self.register.a & 0xF) + (self.register.d & 0xF) > 0xF;
                self.register.flag.c = (self.register.a as u16) + (self.register.d as u16) > 0xF;

                self.register.a = res;
                return;
            }

            Opcode::add_a_e => {
                let res = self.register.a.wrapping_add(self.register.e);

                self.register.flag.z = res == 0;
                self.register.flag.n = false;
                self.register.flag.h = (self.register.a & 0xF) + (self.register.e & 0xF) > 0xF;
                self.register.flag.c = (self.register.a as u16) + (self.register.e as u16) > 0xF;

                self.register.a = res;
                return;
            }

            Opcode::add_a_h => {
                let res = self.register.a.wrapping_add(self.register.h);

                self.register.flag.z = res == 0;
                self.register.flag.n = false;
                self.register.flag.h = (self.register.a & 0xF) + (self.register.h & 0xF) > 0xF;
                self.register.flag.c = (self.register.a as u16) + (self.register.h as u16) > 0xF;

                self.register.a = res;
                return;
            }

            Opcode::add_a_l => {
                let res = self.register.a.wrapping_add(self.register.l);

                self.register.flag.z = res == 0;
                self.register.flag.n = false;
                self.register.flag.h = (self.register.a & 0xF) + (self.register.l & 0xF) > 0xF;
                self.register.flag.c = (self.register.a as u16) + (self.register.l as u16) > 0xF;

                self.register.a = res;
                return;
            }

            Opcode::add_a_hl => {
                let value = self.interconnect.load8(self.register.hl());
                let res = self.register.a.wrapping_add(value as u8);

                self.register.flag.z = res == 0;
                self.register.flag.n = false;
                self.register.flag.h = (self.register.a & 0xF) + (value as u8 & 0xF) > 0xF;
                self.register.flag.c = (self.register.a as u16) + (value as u16) > 0xF;

                self.register.a = res;
                return;
            }

            Opcode::push_af => {
                self.register.sp -= 2;

                let value = self.register.af();

                let addr = self.register.sp;
                self.store16(addr, value);
                return;
            }

            Opcode::push_bc => {
                self.register.sp -= 2;

                let value = self.register.bc();

                let addr = self.register.sp;
                self.store16(addr, value);
                return;
            }

            Opcode::push_de => {
                self.register.sp -= 2;

                let value = self.register.de();

                let addr = self.register.sp;
                self.store16(addr, value);
                return;
            }

            Opcode::push_hl => {
                self.register.sp -= 2;

                let value = self.register.hl();

                let addr = self.register.sp;
                self.store16(addr, value);
                return;
            }

            Opcode::call_nn => {
                self.register.sp -= 2;

                let value = self.register.pc;

                let mut addr = self.register.sp;
                self.store16(addr, value);

                addr = self.register.pc;
                let nn = self.load16(addr);

                self.register.pc = nn;

                return;
            }

            Opcode::di => {
                // TODO: Probably incorrect implementation.
                // Require delay for executing.

                self.interconnect.interrupt = 0;
                return;
            }

            Opcode::xor_a_a => {
                let res = self.register.a ^ self.register.a;

                self.register.flag.z = res == 0;
                self.register.flag.n = false;
                self.register.flag.h = false;
                self.register.flag.c = false;

                self.register.a = res;
                return;
            }

            Opcode::xor_a_b => {
                let res = self.register.a ^ self.register.b;

                self.register.flag.z = res == 0;
                self.register.flag.n = false;
                self.register.flag.h = false;
                self.register.flag.c = false;

                self.register.a = res;
                return;
            }

            Opcode::xor_a_c => {
                let res = self.register.a ^ self.register.c;

                self.register.flag.z = res == 0;
                self.register.flag.n = false;
                self.register.flag.h = false;
                self.register.flag.c = false;

                self.register.a = res;
                return;
            }

            Opcode::xor_a_d => {
                let res = self.register.a ^ self.register.d;

                self.register.flag.z = res == 0;
                self.register.flag.n = false;
                self.register.flag.h = false;
                self.register.flag.c = false;

                self.register.a = res;
                return;
            }

            Opcode::xor_a_e => {
                let res = self.register.a ^ self.register.e;

                self.register.flag.z = res == 0;
                self.register.flag.n = false;
                self.register.flag.h = false;
                self.register.flag.c = false;

                self.register.a = res;
                return;
            }

            Opcode::xor_a_h => {
                let res = self.register.a ^ self.register.h;

                self.register.flag.z = res == 0;
                self.register.flag.n = false;
                self.register.flag.h = false;
                self.register.flag.c = false;

                self.register.a = res;
                return;
            }

            Opcode::xor_a_l => {
                let res = self.register.a ^ self.register.l;

                self.register.flag.z = res == 0;
                self.register.flag.n = false;
                self.register.flag.h = false;
                self.register.flag.c = false;

                self.register.a = res;
                return;
            }

            Opcode::xor_a_hl => {
                let value = self.interconnect.load8(self.register.hl());
                let res = self.register.a ^ value;

                self.register.flag.z = res == 0;
                self.register.flag.n = false;
                self.register.flag.h = false;
                self.register.flag.c = false;

                self.register.a = res;
                return;
            }

            Opcode::xor_a_asterisk => {
                let value = self.interconnect.load8(self.register.pc);
                let res = self.register.a ^ value;

                self.register.flag.z = res == 0;
                self.register.flag.n = false;
                self.register.flag.h = false;
                self.register.flag.c = false;

                self.register.a = res;

                self.register.pc = self.register.pc.wrapping_add(1);

                return;
            }

            Opcode::ret_nz => {
                if !self.register.flag.z {
                    self.register.pc = self.interconnect.load16(self.register.sp);
                    self.register.sp = self.register.sp.wrapping_add(2);
                }

                return;
            }

            Opcode::ret_z => {
                if self.register.flag.z {
                    self.register.pc = self.interconnect.load16(self.register.sp);
                    self.register.sp = self.register.sp.wrapping_add(2);
                }

                return;
            }

            Opcode::ret_nc => {
                if !self.register.flag.c {
                    self.register.pc = self.interconnect.load16(self.register.sp);
                    self.register.sp = self.register.sp.wrapping_add(2);
                }

                return;
            }

            Opcode::ret_c => {
                if self.register.flag.c {
                    self.register.pc = self.interconnect.load16(self.register.sp);
                    self.register.sp = self.register.sp.wrapping_add(2);
                }

                return;
            }

            Opcode::ldh_n_a => {
                let addr = 0xFF00 | self.interconnect.load8(self.register.pc) as u16;
                self.interconnect.store8(addr, self.register.a);
            }

            Opcode::ldh_a_n => {
                let addr = 0xFF00 | self.interconnect.load8(self.register.pc) as u16;
                self.register.a = self.interconnect.load8(addr);

                self.register.pc = self.register.pc.wrapping_add(1);
            }

            Opcode::add_a_sharp => {
                let value = self.interconnect.load8(self.current_pc);
                let res = self.register.a.wrapping_add(value);

                self.register.flag.z = res == 0;
                self.register.flag.n = false;
                self.register.flag.h = (self.register.a & 0xF) + (value & 0xF) > 0xF;
                self.register.flag.c = (self.register.a as u16) + (value as u16) > 0xF;

                self.register.a = res;
                return;
            }

            Opcode::sub_a_a => {
                let res = self.register.a.wrapping_sub(self.register.a);

                self.register.flag.z = res == 0;
                self.register.flag.n = true;
                self.register.flag.h = (self.register.a & 0x0F) < (self.register.a & 0x0F);
                self.register.flag.c = (self.register.a as u16) < (self.register.a as u16);

                self.register.a = res;
                return;
            }

            Opcode::sub_a_b => {
                let res = self.register.a.wrapping_sub(self.register.b);

                self.register.flag.z = res == 0;
                self.register.flag.n = true;
                self.register.flag.h = (self.register.a & 0x0F) < (self.register.b & 0x0F);
                self.register.flag.c = (self.register.a as u16) < (self.register.b as u16);

                self.register.a = res;
                return;
            }

            Opcode::sub_a_c => {
                let res = self.register.a.wrapping_sub(self.register.c);

                self.register.flag.z = res == 0;
                self.register.flag.n = true;
                self.register.flag.h = (self.register.a & 0x0F) < (self.register.c & 0x0F);
                self.register.flag.c = (self.register.a as u16) < (self.register.c as u16);

                self.register.a = res;
                return;
            }

            Opcode::sub_a_d => {
                let res = self.register.a.wrapping_sub(self.register.d);

                self.register.flag.z = res == 0;
                self.register.flag.n = true;
                self.register.flag.h = (self.register.a & 0x0F) < (self.register.d & 0x0F);
                self.register.flag.c = (self.register.a as u16) < (self.register.d as u16);

                self.register.a = res;
                return;
            }

            Opcode::sub_a_e => {
                let res = self.register.a.wrapping_sub(self.register.e);

                self.register.flag.z = res == 0;
                self.register.flag.n = true;
                self.register.flag.h = (self.register.a & 0x0F) < (self.register.e & 0x0F);
                self.register.flag.c = (self.register.a as u16) < (self.register.e as u16);

                self.register.a = res;
                return;
            }

            Opcode::sub_a_h => {
                let res = self.register.a.wrapping_sub(self.register.h);

                self.register.flag.z = res == 0;
                self.register.flag.n = true;
                self.register.flag.h = (self.register.a & 0x0F) < (self.register.h & 0x0F);
                self.register.flag.c = (self.register.a as u16) < (self.register.h as u16);

                self.register.a = res;
                return;
            }

            Opcode::sub_a_l => {
                let res = self.register.a.wrapping_sub(self.register.l);

                self.register.flag.z = res == 0;
                self.register.flag.n = true;
                self.register.flag.h = (self.register.a & 0x0F) < (self.register.l & 0x0F);
                self.register.flag.c = (self.register.a as u16) < (self.register.l as u16);

                self.register.a = res;
                return;
            }

            Opcode::sub_a_hl => {
                let value = self.interconnect.load8(self.register.hl());
                let res = self.register.a.wrapping_sub(value as u8);

                self.register.flag.z = res == 0;
                self.register.flag.n = true;
                self.register.flag.h = (self.register.a & 0x0F) < (value as u8 & 0x0F);
                self.register.flag.c = (self.register.a as u16) < (value as u16);

                self.register.a = res;
                return;
            }

            Opcode::sub_a_sharp => {
                let value = self.interconnect.load8(self.current_pc);
                let res = self.register.a.wrapping_sub(value);

                self.register.flag.z = res == 0;
                self.register.flag.n = true;
                self.register.flag.h = (self.register.a & 0x0F) < (value & 0x0F);
                self.register.flag.c = (self.register.a as u16) < (value as u16);

                self.register.a = res;
                return;
            }

            Opcode::jp_nn => {
                self.clock.m += 4;

                let addr = self.register.pc;
                let nn = self.load16(addr);

                self.register.pc = nn;
                return;
            }

            Opcode::rst_00 => {
                let value = self.current_pc;

                let addr = self.register.sp;
                self.store16(addr, value);

                self.register.pc = 0x00;
                return;
            }

            Opcode::rst_08 => {
                let value = self.current_pc;

                let addr = self.register.sp;
                self.store16(addr, value);

                self.register.pc = 0x08;
                return;
            }

            Opcode::rst_10 => {
                let value = self.current_pc;

                let addr = self.register.sp;
                self.store16(addr, value);

                self.register.pc = 0x10;
                return;
            }

            Opcode::rst_18 => {
                let value = self.current_pc;

                let addr = self.register.sp;
                self.store16(addr, value);

                self.register.pc = 0x18;
                return;
            }

            Opcode::rst_20 => {
                let value = self.current_pc;

                let addr = self.register.sp;
                self.store16(addr, value);

                self.register.pc = 0x20;
                return;
            }

            Opcode::rst_28 => {
                let value = self.current_pc;

                let addr = self.register.sp;
                self.store16(addr, value);

                self.register.pc = 0x28;
                return;
            }

            Opcode::rst_30 => {
                let value = self.current_pc;

                let addr = self.register.sp;
                self.store16(addr, value);

                self.register.pc = 0x30;
                return;
            }

            Opcode::rst_38 => {
                let value = self.current_pc;

                let addr = self.register.sp;
                self.store16(addr, value);

                self.register.pc = 0x38;
                return;
            }

            Opcode::jp_hl => {
                self.clock.m += 1;

                self.register.pc = self.register.hl();
                return;
            }
        }
    }
}

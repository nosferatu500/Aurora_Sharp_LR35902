use interconnect::Interconnect;
use opcode::Opcode;

struct Clock {
  m: u16,
  t: u16,
}

impl Clock {
  pub fn new() -> Clock {
    Clock {
      m: 0,
      t: 0,
    }
  }
}

struct Flag {
  z: u8, // 7
  n: u8, // 6
  h: u8, // 5
  c: u8, // 4
}

impl Flag {
  pub fn init() -> Flag {
    Flag {
      z: 0,
      n: 0,
      h: 0,
      c: 0,
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
            regs: [0xdeadbeef; 16],
            
            current_pc: pc,

            interconnect,

            register: Register::new(),

            clock: Clock::new(),

            op: Opcode::jp_nn,
        }
    }

    fn get_regs(&self, index: u16) -> u16 {
      self.regs[index as usize]
    }

    fn set_regs(&mut self, index: u16, value: u16) {
      self.regs[index as usize] = value;
    }

    pub fn run_next_instruction(&mut self) {
        let instruction = self.interconnect.load16(self.register.pc);

        self.current_pc = self.register.pc;

        let pc = self.register.pc;

        self.register.pc = pc.wrapping_add(2);

        self.decode(instruction);
    }

    fn decode(&mut self, instruction: u16) {
      let value = (instruction >> 8) & 0xff;
      let opcode = self.op.find(value); 

      println!("Opcode: {:#x}", value);

      match opcode {
        Opcode::nop => return,

        Opcode::ld_bc_nn => {
          let lhs = self.interconnect.load8(self.register.pc) as u16;
          let rhs = (self.interconnect.load8(self.register.pc + 1) as u16) << 8;

          let nn = lhs | rhs;

          self.register.set_bc(nn);
          return;
        }

        Opcode::ld_de_nn => {
          let lhs = self.interconnect.load8(self.register.pc) as u16;
          let rhs = (self.interconnect.load8(self.register.pc + 1) as u16) << 8;

          let nn = lhs | rhs;

          self.register.set_de(nn);
          return;
        }

        Opcode::ld_hl_nn => {
          let lhs = self.interconnect.load8(self.register.pc) as u16;
          let rhs = (self.interconnect.load8(self.register.pc + 1) as u16) << 8;

          let nn = lhs | rhs;

          self.register.set_hl(nn);
          return;
        }

        Opcode::ld_sp_nn => {
          let lhs = self.interconnect.load8(self.register.pc) as u16;
          let rhs = (self.interconnect.load8(self.register.pc + 1) as u16) << 8;

          let nn = lhs | rhs;

          self.register.sp = nn;
          return;
        }

        Opcode::ld_a_a => return,
        Opcode::ld_a_b => {
          self.register.a = self.register.b; 
          return;
        },
        Opcode::ld_a_c => {
          self.register.a = self.register.c; 
          return;
        },
        Opcode::ld_a_d => {
          self.register.a = self.register.d; 
          return;
        },
        Opcode::ld_a_e => {
          self.register.a = self.register.e; 
          return;
        },
        Opcode::ld_a_h => {
          self.register.a = self.register.h; 
          return;
        },
        Opcode::ld_a_l => {
          self.register.a = self.register.l; 
          return;
        },
        Opcode::ld_a_hl => {
          self.register.a = self.register.hl() as u8;
          return;
        },
        Opcode::ld_b_b => {
          self.register.b = self.register.b;
          return;
        },
        Opcode::ld_b_c => {
          self.register.b = self.register.c;
          return;
        },
        Opcode::ld_b_d => {
          self.register.b = self.register.d;
          return;
        },
        Opcode::ld_b_e => {
          self.register.b = self.register.e;
          return;
        },
        Opcode::ld_b_h => {
          self.register.b = self.register.h;
          return;
        },
        Opcode::ld_b_l => {
          self.register.b = self.register.l;
          return;
        },
        Opcode::ld_b_hl => {
          self.register.b = self.register.hl() as u8;
          return;
        },
        Opcode::ld_c_b => {
          self.register.c = self.register.b;
          return;
        },
        Opcode::ld_c_c => {
          self.register.c = self.register.c;
          return;
        },
        Opcode::ld_c_d => {
          self.register.c = self.register.d;
          return;
        },
        Opcode::ld_c_e => {
          self.register.c = self.register.e;
          return;
        },
        Opcode::ld_c_h => {
          self.register.c = self.register.h;
          return;
        },
        Opcode::ld_c_l => {
          self.register.c = self.register.l;
          return;
        },
        Opcode::ld_c_hl =>{
           self.register.c = self.register.hl() as u8;
           return;
        },
        Opcode::ld_d_b => {
          self.register.d = self.register.b;
          return;
        },
        Opcode::ld_d_c => {
          self.register.d = self.register.c;
          return;
        },
        Opcode::ld_d_d => {
          self.register.d = self.register.d;
          return;
        },
        Opcode::ld_d_e => {
          self.register.d = self.register.e;
          return;
        },
        Opcode::ld_d_h => {
          self.register.d = self.register.h;
          return;
        },
        Opcode::ld_d_l => {
          self.register.d = self.register.l;
          return;
        },
        Opcode::ld_d_hl => {
          self.register.d = self.interconnect.load8(self.register.hl());
          return;
        },
        Opcode::ld_e_b => {
          self.register.e = self.register.b;
          return;
        },
        Opcode::ld_e_c => {
          self.register.e = self.register.c;
          return;
        },
        Opcode::ld_e_d => {
          self.register.e = self.register.d;
          return;
        },
        Opcode::ld_e_e => {
          self.register.e = self.register.e;
          return;
        },
        Opcode::ld_e_h => {
          self.register.e = self.register.h;
          return;
        },
        Opcode::ld_e_l => {
          self.register.e = self.register.l;
          return;
        },
        Opcode::ld_e_hl => {
          self.register.e = self.register.hl() as u8;
          return;
        },
        Opcode::ld_h_b => {
          self.register.h = self.register.b;
          return;
        },
        Opcode::ld_h_c => {
          self.register.h = self.register.c;
          return;
        },
        Opcode::ld_h_d => {
          self.register.h = self.register.d;
          return;
        },
        Opcode::ld_h_e => {
          self.register.h = self.register.e;
          return;
        },
        Opcode::ld_h_h => {
          self.register.h = self.register.h;
          return;
        },
        Opcode::ld_h_l => {
          self.register.h = self.register.l;
          return;
        },
        Opcode::ld_h_hl => {
          self.register.h = self.register.hl() as u8;
          return;
        },
        Opcode::ld_l_b => {
          self.register.l = self.register.b;
          return;
        },
        Opcode::ld_l_c => {
          self.register.l = self.register.c;
          return;
        },
        Opcode::ld_l_d => {
          self.register.l = self.register.d;
          return;
        },
        Opcode::ld_l_e => {
          self.register.l = self.register.e;
          return;
        },
        Opcode::ld_l_h => {
          self.register.l = self.register.h;
          return;
        },
        Opcode::ld_l_l => {
          self.register.l = self.register.l;
          return;
        },
        Opcode::ld_l_hl => {
          self.register.l = self.register.hl() as u8;
          return;
        },
        Opcode::ld_hl_b => {
          let value = self.register.b;
          self.register.set_hl(value as u16);
          return;
        }
        Opcode::ld_hl_c => {
          let value = self.register.c;
          self.register.set_hl(value as u16);
          return;
        }
        Opcode::ld_hl_d => {
          let value = self.register.d;
          self.register.set_hl(value as u16);
          return;
        }
        Opcode::ld_hl_e => {
          let value = self.register.e;
          self.register.set_hl(value as u16);
          return;
        }
        Opcode::ld_hl_h => {
          let value = self.register.h;
          self.register.set_hl(value as u16);
          return;
        }
        Opcode::ld_hl_l => {
          let value = self.register.l;
          self.register.set_hl(value as u16);
          return;
        }
        Opcode::ld_hl_n => {
          let value = self.interconnect.load8(self.current_pc);
          self.register.set_hl(value as u16);
          return;
        },

        Opcode::jp_nn => {
          self.clock.m += 4;
          let lhs = self.interconnect.load8(self.register.pc) as u16;
          let rhs = (self.interconnect.load8(self.register.pc + 1) as u16) << 8;

          let nn = lhs | rhs;

          self.register.pc = nn;
          return;
        }

        Opcode::rst_00 => {
          let value = self.current_pc;

          self.interconnect.store8(self.register.sp, (value & 0xff) as u8);
          self.interconnect.store8(self.register.sp + 1, (value >> 8) as u8);


          self.register.pc = 0x00;
          return;
        }

        Opcode::rst_08 => {
          let value = self.current_pc;

          self.interconnect.store8(self.register.sp, (value & 0xff) as u8);
          self.interconnect.store8(self.register.sp + 1, (value >> 8) as u8);


          self.register.pc = 0x08;
          return;
        }

        Opcode::rst_10 => {
          let value = self.current_pc;

          self.interconnect.store8(self.register.sp, (value & 0xff) as u8);
          self.interconnect.store8(self.register.sp + 1, (value >> 8) as u8);

          self.register.pc = 0x10;
          return;
        }

        Opcode::rst_18 => {
          let value = self.current_pc;

          self.interconnect.store8(self.register.sp, (value & 0xff) as u8);
          self.interconnect.store8(self.register.sp + 1, (value >> 8) as u8);

          self.register.pc = 0x18;
          return;
        }

        Opcode::rst_20 => {
          let value = self.current_pc;

          self.interconnect.store8(self.register.sp, (value & 0xff) as u8);
          self.interconnect.store8(self.register.sp + 1, (value >> 8) as u8);

          self.register.pc = 0x20;
          return;
        }

        Opcode::rst_28 => {
          let value = self.current_pc;

          self.interconnect.store8(self.register.sp, (value & 0xff) as u8);
          self.interconnect.store8(self.register.sp + 1, (value >> 8) as u8);

          self.register.pc = 0x28;
          return;
        }

        Opcode::rst_30 => {
          let value = self.current_pc;

          self.interconnect.store8(self.register.sp, (value & 0xff) as u8);
          self.interconnect.store8(self.register.sp + 1, (value >> 8) as u8);

          self.register.pc = 0x30;
          return;
        }

        Opcode::rst_38 => {
          let value = self.current_pc;

          self.interconnect.store8(self.register.sp, (value & 0xff) as u8);
          self.interconnect.store8(self.register.sp + 1, (value >> 8) as u8);

          self.register.pc = 0x38;
          return;
        }

        Opcode::jp_hl => {
          self.clock.m += 1;
          
          self.register.pc = self.register.hl();
          return;
        }
      }

      panic!("Unknown instruction: {:#06x} {:016b}", instruction, instruction);
    }
}
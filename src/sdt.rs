pub struct Sdt {
	enabled: bool,
	data: u8,
	control: u8,
}

impl Sdt {
	pub fn new() -> Sdt {
		Sdt { 
      enabled: true, 
      data: 0, 
      control: 0,
    }
	}

	pub fn wb(&mut self, a: u16, v: u8) {
		match a {
			0xFF01 => {
				if self.enabled {
					println!("{}", v as char);
				}
				self.data = v;
			},
			0xFF02 => { self.control = v; },
			_ => { panic!("Serial does not handle address {:4X} (write)", a); },
		};
	}

	pub fn rb(&self, a: u16) -> u8 {
		match a {
			0xFF01 => self.data,
			0xFF02 => self.control,
			_ => panic!("Serial does not handle address {:4X} (read)", a),
		}
	}
}
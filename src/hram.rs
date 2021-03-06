pub struct Hram {
    data: Vec<u8>,
}

impl Hram {
    pub fn new() -> Hram {
        Hram {
            data: vec![0; 0xFFFE - 0xFF80 + 0x1],
        }
    }

    pub fn load8(&self, offset: u16) -> u8 {
        self.data[offset as usize]
    }

    pub fn load16(&self, offset: u16) -> u16 {
        let offset = offset as usize;

        let b0 = self.data[offset + 0] as u16;
        let b1 = self.data[offset + 1] as u16;

        b0 | (b1 << 8)
    }

    pub fn store8(&mut self, offset: u16, value: u8) {
        self.data[offset as usize] = value;
    }
}

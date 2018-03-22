use std::path::Path;
use std::fs::File;
use std::io::*;

pub struct Rom {
    data: Vec<u8>,
}

impl Rom {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Rom> {
        let mut file = try!(File::open(&path));

        let mut data = Vec::new();

        try!(file.read_to_end(&mut data));

        Ok(Rom { data: data })
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
}

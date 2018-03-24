use std::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Callback {

}

impl Callback {
    pub fn find(self, value: u8) -> Callback {
        unsafe { mem::transmute(value) }
    }
}
use std::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    nop = 0x00,

    ld_bc_nn = 0x01,
    ld_de_nn = 0x11,
    ld_hl_nn = 0x21,
    ld_sp_nn = 0x31,

    ld_a_a = 0x7F,
    ld_a_b = 0x78,
    ld_a_c = 0x79,
    ld_a_d = 0x7A,
    ld_a_e = 0x7B,
    ld_a_h = 0x7C,
    ld_a_l = 0x7D,
    ld_a_hl = 0x7E,
    ld_b_b = 0x40,
    ld_b_c = 0x41,
    ld_b_d = 0x42,
    ld_b_e = 0x43,
    ld_b_h = 0x44,
    ld_b_l = 0x45,
    ld_b_hl = 0x46,
    ld_c_b = 0x48,
    ld_c_c = 0x49,
    ld_c_d = 0x4A,
    ld_c_e = 0x4B,
    ld_c_h = 0x4C,
    ld_c_l = 0x4D,
    ld_c_hl = 0x4E,
    ld_d_b = 0x50,
    ld_d_c = 0x51,
    ld_d_d = 0x52,
    ld_d_e = 0x53,
    ld_d_h = 0x54,
    ld_d_l = 0x55,
    ld_d_hl = 0x56,
    ld_e_b = 0x58,
    ld_e_c = 0x59,
    ld_e_d = 0x5A,
    ld_e_e = 0x5B,
    ld_e_h = 0x5C,
    ld_e_l = 0x5D,
    ld_e_hl = 0x5E,
    ld_h_b = 0x60,
    ld_h_c = 0x61,
    ld_h_d = 0x62,
    ld_h_e = 0x63,
    ld_h_h = 0x64,
    ld_h_l = 0x65,
    ld_h_hl = 0x66,
    ld_l_b = 0x68,
    ld_l_c = 0x69,
    ld_l_d = 0x6A,
    ld_l_e = 0x6B,
    ld_l_h = 0x6C,
    ld_l_l = 0x6D,
    ld_l_hl = 0x6E,
    ld_hl_b = 0x70,
    ld_hl_c = 0x71,
    ld_hl_d = 0x72,
    ld_hl_e = 0x73,
    ld_hl_h = 0x74,
    ld_hl_l = 0x75,
    ld_hl_n = 0x36,

    jp_nn = 0xC3,
    jp_hl = 0xE9,

    rst_00 = 0xC7,
    rst_08 = 0xCF,
    rst_10 = 0xD7,
    rst_18 = 0xDF,
    rst_20 = 0xE7,
    rst_28 = 0xEF,
    rst_30 = 0xF7,
    rst_38 = 0xFF,
}

impl Opcode {
    pub fn find(self, value: u16) -> Opcode {
        unsafe { mem::transmute(value as u8) }
    }
}

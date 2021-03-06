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
    ld_a_bc = 0x0A,
    ld_a_de = 0x1A,
    ld_a_nn = 0xFA,
    ld_a_sharp = 0x3E,
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

    ld_b_a = 0x47,
    ld_c_a = 0x4F,
    ld_d_a = 0x57,
    ld_e_a = 0x5F,
    ld_h_a = 0x67,
    ld_l_a = 0x6F,
    ld_bc_a = 0x02,
    ld_de_a = 0x12,
    ld_hl_a = 0x77,
    ld_nn_a = 0xEA,

    add_a_a = 0x87,
    add_a_b = 0x80,
    add_a_c = 0x81,
    add_a_d = 0x82,
    add_a_e = 0x83,
    add_a_h = 0x84,
    add_a_l = 0x85,
    add_a_hl = 0x86,
    add_a_sharp = 0xC6,

    sub_a_a = 0x97,
    sub_a_b = 0x90,
    sub_a_c = 0x91,
    sub_a_d = 0x92,
    sub_a_e = 0x93,
    sub_a_h = 0x94,
    sub_a_l = 0x95,
    sub_a_hl = 0x96,
    sub_a_sharp = 0xD6,

    push_af = 0xF5,
    push_bc = 0xC5,
    push_de = 0xD5,
    push_hl = 0xE5,

    call_nn = 0xCD,

    jp_nn = 0xC3,
    jp_hl = 0xE9,

    di = 0xF3,
    ei = 0xFB,

    callback = 0xCB,

    rst_00 = 0xC7,
    rst_08 = 0xCF,
    rst_10 = 0xD7,
    rst_18 = 0xDF,
    rst_20 = 0xE7,
    rst_28 = 0xEF,
    rst_30 = 0xF7,
    rst_38 = 0xFF,

    xor_a_a = 0xAF,
    xor_a_b = 0xA8,
    xor_a_c = 0xA9,
    xor_a_d = 0xAA,
    xor_a_e = 0xAB,
    xor_a_h = 0xAC,
    xor_a_l = 0xAD,
    xor_a_hl = 0xAE,
    xor_a_asterisk = 0xEE,

    ldh_n_a = 0xE0,

    ldh_a_n = 0xF0,

    ret_nz = 0xC0,
    ret_z = 0xC8,
    ret_nc = 0xD0,
    ret_c = 0xD8,

    cp_a_a = 0xBF,
    cp_a_b = 0xB8,
    cp_a_c = 0xB9,
    cp_a_d = 0xBA,
    cp_a_e = 0xBB,
    cp_a_h = 0xBC,
    cp_a_l = 0xBD,
    cp_a_hl = 0xBE,
    cp_a_sharp = 0xFE,

    jr_nz_n = 0x20,
    jr_z_n = 0x28,
    jr_nc_n = 0x30,
    jr_c_n = 0x38,

    ret = 0xC9,

    halt = 0x76,
}

impl Opcode {
    pub fn find(self, value: u8) -> Opcode {
        unsafe { mem::transmute(value) }
    }
}

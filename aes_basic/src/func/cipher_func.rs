use crate::func::common::{gf_mul, sbox, add_round_key};

pub fn sub_bytes(state: &mut [u8; 16]) {
    for i in 0..16 {
        state[i] = sbox(state[i]);
    }
}

pub fn shift_rows(state: &mut [u8; 16]) {
    let mut tmp = [0u8; 16];
    // 行0
    tmp[0]  = state[0];
    tmp[4]  = state[4];
    tmp[8]  = state[8];
    tmp[12] = state[12];
    // 行1
    tmp[1]  = state[5];
    tmp[5]  = state[9];
    tmp[9]  = state[13];
    tmp[13] = state[1];
    // 行2
    tmp[2]  = state[10];
    tmp[6]  = state[14];
    tmp[10] = state[2];
    tmp[14] = state[6];
    // 行3
    tmp[3]  = state[15];
    tmp[7]  = state[3];
    tmp[11] = state[7];
    tmp[15] = state[11];

    state.copy_from_slice(&tmp);
}

pub fn mix_columns(state: &mut [u8; 16]) {
    for i in 0..4 {
        let s0 = state[i*4];
        let s1 = state[i*4+1];
        let s2 = state[i*4+2];
        let s3 = state[i*4+3];
        state[i*4] = gf_mul(0x02, s0) ^ gf_mul(0x03, s1) ^ s2 ^ s3;
        state[i*4+1] = s0 ^ gf_mul(0x02, s1) ^ gf_mul(0x03, s2) ^ s3;
        state[i*4+2] = s0 ^ s1 ^ gf_mul(0x02, s2) ^ gf_mul(0x03, s3);
        state[i*4+3] = gf_mul(0x03, s0) ^ s1 ^ s2 ^ gf_mul(0x02, s3);
    }
}

pub fn cipher(in_vec: &[u8; 16], nr: i32, w: &Vec<u32>) -> [u8; 16] {
    let mut state = in_vec.clone();
    let mut round_key = w[0..4].try_into().expect("Slice with incorrect length");
    add_round_key(&mut state, round_key);
    for round in 1..nr {
        sub_bytes(&mut state);
        shift_rows(&mut state);
        mix_columns(&mut state);
        round_key = w[(4*round as usize)..(4*round as usize+4)].try_into().expect("Slice with incorrect length");
        add_round_key(&mut state, round_key);
    }
    sub_bytes(&mut state);
    shift_rows(&mut state);
    round_key = w[(4*nr as usize)..(4*nr as usize+4)].try_into().expect("Slice with incorrect length");
    add_round_key(&mut state, round_key);
    state
}
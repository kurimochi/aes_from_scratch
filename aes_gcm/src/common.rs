use aes_basic::aes_encrypt;

use crate::ghash::ghash;

pub fn init(key: &Vec<u8>, iv: &[u8]) -> (u128, [u8; 16]) {
    let h: u128 = u128::from_be_bytes(aes_encrypt(&[0u8; 16], key));
    let mut y0 = [0u8; 16];
    if iv.len() == 12 {
        y0[..12].copy_from_slice(iv);
        y0[15] = 1;
    } else {
        let mut buf = iv.to_vec();
        while buf.len() % 16 != 0 {
            buf.push(0);
        }
        let iv_bits_len = (iv.len() as u64) * 8;
        let mut length_block = [0u8; 16];
        length_block[8..].copy_from_slice(&iv_bits_len.to_be_bytes());
        buf.extend_from_slice(&length_block);
        y0 = ghash(&[0u8; 0], &buf, h, true).to_be_bytes();
    }
    (h, y0)
}

pub fn authenticate(y0: &[u8; 16], key: &Vec<u8>, aad: &[u8], cipher: &[u8], h: u128) -> u128 {
    let aes_out = aes_encrypt(&y0, key);
    let ghash_out = ghash(aad, &cipher, h, false);
    let aes_out_u128 = u128::from_be_bytes(aes_out);
    let tag = aes_out_u128 ^ ghash_out;
    tag
}
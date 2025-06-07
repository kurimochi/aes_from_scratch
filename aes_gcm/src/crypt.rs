use aes_basic::aes_encrypt;

use crate::common::{init, authenticate};
use crate::error::GcmError;

pub fn encrypt_gcm(input: &[u8], aad: &[u8], key: &Vec<u8>, iv: &[u8]) -> (Vec<u8>, u128) {
    let (h, y0) = init(key, iv);
    // Encrypt (AES-CTR)
    let plain: Vec<Vec<u8>> = input.chunks(16)
                                .map(|chunk| chunk.to_vec())
                                .collect();
    let mut cipher: Vec<u8> = Vec::new();
    for (i, plain_i) in plain.iter().enumerate() {
        let mut ctr = y0.clone();
        let counter = (u32::from_be_bytes([y0[12], y0[13], y0[14], y0[15]])).wrapping_add((i + 1) as u32);
        ctr[12..].copy_from_slice(&counter.to_be_bytes());

        let keystream = aes_encrypt(&ctr, key);

        // XOR plain_i with keystream
        let cipher_block: Vec<u8> = plain_i.iter()
            .zip(keystream.iter())
            .map(|(a, b)| a ^ b)
            .collect();

        cipher.extend(cipher_block);
    }

    let t = authenticate(&y0, key, aad, &cipher, h);
    (cipher, t)
}

pub fn decrypt_gcm(input: &[u8], aad: &[u8], key: &Vec<u8>, iv: &[u8], t_recv: u128) -> Result<Vec<u8>, GcmError> {
    let (h, y0) = init(key, iv);
    let t_calc = authenticate(&y0, key, aad, input, h);
    if t_calc != t_recv {
        return Err(GcmError::AuthenticationFailed);
    }

    let mut plain: Vec<u8> = Vec::new();
    let cipher: Vec<Vec<u8>> = input.chunks(16)
                                .map(|chunk| chunk.to_vec())
                                .collect();
    for (i, cipher_i) in cipher.iter().enumerate() {
        let mut ctr = y0.clone();
        let counter = (u32::from_be_bytes([y0[12], y0[13], y0[14], y0[15]])).wrapping_add((i + 1) as u32);
        ctr[12..].copy_from_slice(&counter.to_be_bytes());

        let keystream = aes_encrypt(&ctr, key);

        // XOR plain_i with keystream
        let plain_block: Vec<u8> = cipher_i.iter()
            .zip(keystream.iter())
            .map(|(a, b)| a ^ b)
            .collect();

        plain.extend(plain_block);
    }
    Ok(plain)
}
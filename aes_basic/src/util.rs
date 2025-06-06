use crate::func::{cipher, inv_cipher, key_expansion};

pub fn aes_encrypt(
    message: &[u8; 16],
    key: &Vec<u8>,
) -> [u8; 16] {
    let mut result = message.clone();
    // Convert key to u32
    let key_u32: Vec<u32> = key
                                .chunks(4)
                                .map(|chunk| u32::from_be_bytes(
                                    chunk.try_into().unwrap()
                                ))
                                .collect();

    // Key length check and encryption
    if key_u32.len() == 4 || key_u32.len() == 6 || key_u32.len() == 8 {
        result = cipher(&mut result, (key_u32.len()+6) as i32, &key_expansion(&key_u32));
    } else {
        panic!("[!] Invalid key length. Key must be 16, 24, or 32 bytes long.");
    }

    result
}

pub fn aes_decrypt(
    cipherbytes: &[u8; 16],
    key: &Vec<u8>,
) -> [u8; 16] {
    let mut result = cipherbytes.clone();
    // Convert key to u32
    let key_u32: Vec<u32> = key
                                .chunks(4)
                                .map(|chunk| u32::from_be_bytes(
                                    chunk.try_into().unwrap()
                                ))
                                .collect();

    // Key length check and decryption
    if key_u32.len() == 4 || key_u32.len() == 6 || key_u32.len() == 8 {
        result = inv_cipher(&mut result, (key_u32.len()+6) as i32, &key_expansion(&key_u32));
    } else {
        panic!("[!] Invalid key length. Key must be 16, 24, or 32 bytes long.");
    }

    result
}
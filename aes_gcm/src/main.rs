use rand_apis::gen_random;
use aes_gcm::{encrypt_gcm, decrypt_gcm};

fn main() {
    let input = [
        0xd9, 0x31, 0x32, 0x25, 0xf8, 0x84, 0x06, 0xe5,
        0xa5, 0x59, 0x09, 0xc5, 0xaf, 0xf5, 0x26, 0x9a,
        0x86, 0xa7, 0xa9, 0x53, 0x15, 0x34, 0xf7, 0xda,
        0x2e, 0x4c, 0x30, 0x3d, 0x8a, 0x31, 0x8a, 0x72,
        0x1c, 0x3c, 0x0c, 0x95,
    ];
    let mut buf = [0u8; 16];
    gen_random(&mut buf)
        .expect("[!] Key generation failed");
    let key = buf.to_vec();
    let mut iv = [0u8; 12];
    gen_random(&mut iv)
        .expect("[!] IV generation failed");
    let aad: Vec<u8> = Vec::new();

    let (ciphertext, tag) = encrypt_gcm(&input, &aad, &key, &iv);
    let plain = decrypt_gcm(&ciphertext, &aad, &key, &iv, tag).unwrap();

    print!("key: ");
    for b in &key {
        print!("{:02x}", b);
    }
    println!();

    print!("plain: ");
    for b in &input {
        print!("{:02x}", b);
    }
    println!();

    print!("cipher: ");
    for b in &ciphertext {
        print!("{:02x}", b);
    }
    println!();

    print!("tag: ");
    for b in &tag.to_be_bytes() {
        print!("{:02x}", b);
    }
    println!("\n");
    print!("plain_decrypted: ");
    for b in &plain {
        print!("{:02x}", b);
    }
    println!();
}

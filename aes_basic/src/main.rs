use rand_apis::gen_random;
use aes_basic::{aes_encrypt, aes_decrypt};

fn main() {
    // Input vector
    let in_vec: [u8; 16] = [
        0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d,
        0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34,
    ];

    // Generate key
    let mut buf = [0u8; 32];
    gen_random(&mut buf)
        .expect("[!] Failed to generate key");
    let key = buf.to_vec();

    let c = aes_encrypt(&in_vec, &key);
    let m = aes_decrypt(&c, &key);

    print!("key: ");
    for i in key {
        print!("{:02x}", i);
    }
    println!();
    print!("in: ");
    for i in in_vec {
        print!("{:02x}", i);
    }
    println!("\n");
    print!("cipher: ");
    for i in c {
        print!("{:02x}", i);
    }
    println!("\n");
    print!("decrypt: ");
    for i in m {
        print!("{:02x}", i);
    }
}

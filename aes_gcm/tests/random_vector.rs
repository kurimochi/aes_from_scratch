use rand_apis::gen_random;
use aes_gcm::{encrypt_gcm, decrypt_gcm};

#[test]
fn random_vector() {
    let mut key = [0u8; 16];
    gen_random(&mut key).expect("[!] Key generation failed");
    let key = key.to_vec();

    let mut iv = [0u8; 12];
    gen_random(&mut iv).expect("[!] IV generation failed");

    let input = [0u8; 32];
    let aad: Vec<u8> = Vec::new();

    let (ciphertext, tag) = encrypt_gcm(&input, &aad, &key, &iv);
    let decrypted_plain = decrypt_gcm(&ciphertext, &aad, &key, &iv, tag).unwrap();

    assert_eq!(input.to_vec(), decrypted_plain, "Decrypted plaintext does not match original input");
}
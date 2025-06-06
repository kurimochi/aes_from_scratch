use aes_basic::{aes_encrypt, aes_decrypt};
use aes::Aes128;
use aes::cipher::{BlockEncrypt, BlockDecrypt, KeyInit, generic_array::GenericArray};

#[test]
fn nist_test_vector() {
    // Test vector (from FIPS 197 Upd.1)
    let key: Vec<u8> = vec![
        0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6,
        0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c,
    ];

    // Correct result
    let c_correct: [u8; 16] = [
        0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb,
        0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a, 0x0b, 0x32,
    ];
    let m_correct: [u8; 16] = [
        0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d,
        0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34,
    ];

    let c = aes_encrypt(&m_correct, &key);
    let m = aes_decrypt(&c, &key);

    assert_eq!(c, c_correct, "Ciphertext does not match the expected value.");
    assert_eq!(m, m_correct, "Decrypted plaintext does not match the expected value.");
}

#[test]
fn random_key_test() {
    // Generate a random key
    let mut buf = [0u8; 32];
    rand_apis::gen_random(&mut buf)
        .expect("[!] Failed to generate key");
    let key = buf.to_vec();

    // Input vector
    let in_vec: [u8; 16] = [
        0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d,
        0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34,
    ];

    let c = aes_encrypt(&in_vec, &key);
    let m = aes_decrypt(&c, &key);

    assert_eq!(m, in_vec, "Decrypted plaintext does not match the original input.");
}

#[test]
fn compare_with_aes_crate_encrypt_decrypt() {

    // Use a known key and plaintext
    let key_bytes = vec![
        0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6,
        0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c,
    ];
    let plaintext = [
        0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d,
        0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34,
    ];

    // Encrypt with your implementation
    let my_ciphertext = aes_encrypt(&plaintext, &key_bytes);

    // Encrypt with aes crate
    let cipher = Aes128::new(GenericArray::from_slice(&key_bytes));
    let mut block = GenericArray::clone_from_slice(&plaintext);
    cipher.encrypt_block(&mut block);
    let aes_crate_ciphertext = block.as_slice();

    assert_eq!(
        my_ciphertext, aes_crate_ciphertext,
        "cipher output does not match aes crate"
    );

    // Decrypt with your implementation
    let my_decrypted = aes_decrypt(&my_ciphertext, &key_bytes);

    // Decrypt with aes crate
    let mut block = GenericArray::clone_from_slice(&my_ciphertext);
    cipher.decrypt_block(&mut block);
    let aes_crate_decrypted = block.as_slice();

    assert_eq!(
        my_decrypted, aes_crate_decrypted,
        "inv_cipher output does not match aes crate"
    );
    assert_eq!(
        my_decrypted, plaintext,
        "inv_cipher output does not match original plaintext"
    );
}

#[test]
fn aes192_test_vector() {
    // Test vector for AES-192
    // Source: https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-38a.pdf
    let key: Vec<u8> = vec![
        0x8e, 0x73, 0xb0, 0xf7, 0xda, 0x0e, 0x64, 0x52,
        0xc8, 0x10, 0xf3, 0x2b, 0x80, 0x90, 0x79, 0xe5,
        0x62, 0xf8, 0xea, 0xd2, 0x52, 0x2c, 0x6b, 0x7b,
    ];

    // Correct result
    let c_correct: [u8; 16] = [
        0xbd, 0x33, 0x4f, 0x1d, 0x6e, 0x45, 0xf2, 0x5f,
        0xf7, 0x12, 0xa2, 0x14, 0x57, 0x1f, 0xa5, 0xcc,
    ];
    let m_correct: [u8; 16] = [
        0x6b, 0xc1, 0xbe, 0xe2, 0x2e, 0x40, 0x9f, 0x96,
        0xe9, 0x3d, 0x7e, 0x11, 0x73, 0x93, 0x17, 0x2a,
    ];

    let c = aes_encrypt(&m_correct, &key);
    let m = aes_decrypt(&c, &key);

    assert_eq!(c.as_slice(), c_correct.as_slice(), "Ciphertext does not match the expected value.");
    assert_eq!(m.as_slice(), m_correct.as_slice(), "Decrypted plaintext does not match the expected value.");
}

#[test]
fn aes256_test_vector() {
    // Test vector for AES-256
    // Source: https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-38a.pdf
    let key: Vec<u8> = vec![
        0x60, 0x3d, 0xeb, 0x10, 0x15, 0xca, 0x71, 0xbe,
        0x2b, 0x73, 0xae, 0xf0, 0x85, 0x7d, 0x77, 0x81,
        0x1f, 0x35, 0x2c, 0x07, 0x3b, 0x61, 0x08, 0xd7,
        0x2d, 0x98, 0x10, 0xa3, 0x09, 0x14, 0xdf, 0xf4,
    ];

    // Correct result
    let c_correct: [u8; 16] = [
        0xf3, 0xee, 0xd1, 0xbd, 0xb5, 0xd2, 0xa0, 0x3c,
        0x06, 0x4b, 0x5a, 0x7e, 0x3d, 0xb1, 0x81, 0xf8,
    ];
    let m_correct: [u8; 16] = [
        0x6b, 0xc1, 0xbe, 0xe2, 0x2e, 0x40, 0x9f, 0x96,
        0xe9, 0x3d, 0x7e, 0x11, 0x73, 0x93, 0x17, 0x2a,
    ];

    let c = aes_encrypt(&m_correct, &key);
    let m = aes_decrypt(&c, &key);

    assert_eq!(c.as_slice(), c_correct.as_slice(), "Ciphertext does not match the expected value.");
    assert_eq!(m.as_slice(), m_correct.as_slice(), "Decrypted plaintext does not match the expected value.");
}

#[test]
fn invalid_key_length_test() {
    // Test with an invalid key length
    let key: Vec<u8> = vec![0x00; 15]; // Invalid key length (15 bytes)

    let in_vec: [u8; 16] = [
        0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d,
        0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34,
    ];

    // Expect panic due to invalid key length
    let result = std::panic::catch_unwind(|| {
        aes_encrypt(&in_vec, &key);
    });

    assert!(result.is_err(), "Expected panic due to invalid key length.");
}
fn gf_mul(x: u128, mut h: u128) -> u128 {
    let mut z: u128 = 0;
    // NIST定義の還元多項式 R = 0xe1_00...00
    const R: u128 = 0xe1000000000000000000000000000000;

    // X の上位ビットから順に
    for i in (0..128).rev() {
        // ビット i が立っていたら z ⊕= h
        if ((x >> i) & 1) != 0 {
            z ^= h;
        }
        // h を右に1ビットシフトし、LSBが1だったら R を ⊕
        let lsb = (h & 1) != 0;
        h >>= 1;
        if lsb {
            h ^= R;
        }
    }
    z
}

pub fn ghash(aad: &[u8], ciphertext: &[u8], h: u128, skip_len_block: bool) -> u128 {
    let mut blocks: Vec<u128> = Vec::new();

    // AADパディング
    let aad_padding = (16 - aad.len() % 16) % 16;
    let mut aad_padded = aad.to_vec();
    aad_padded.extend(std::iter::repeat(0).take(aad_padding));
    for chunk in aad_padded.chunks(16) {
        let bytes: [u8; 16] = chunk.try_into().unwrap();
        blocks.push(u128::from_be_bytes(bytes));
    }

    // CIPHERTEXTパディング
    let ct_padding = (16 - ciphertext.len() % 16) % 16;
    let mut ct_padded = ciphertext.to_vec();
    ct_padded.extend(std::iter::repeat(0).take(ct_padding));
    for chunk in ct_padded.chunks(16) {
        let bytes: [u8; 16] = chunk.try_into().unwrap();
        blocks.push(u128::from_be_bytes(bytes));
    }

    // 長さブロック
    if !skip_len_block {
        let aad_bits = (aad.len() as u64) * 8;
        let ct_bits = (ciphertext.len() as u64) * 8;
        let mut len_block = [0u8; 16];
        len_block[..8].copy_from_slice(&aad_bits.to_be_bytes());
        len_block[8..].copy_from_slice(&ct_bits.to_be_bytes());
        blocks.push(u128::from_be_bytes(len_block));
    }

    // GHASH本体
    let mut x: u128 = 0;
    for block in blocks {
        x = gf_mul(x ^ block, h);
        // println!("x: {:032x}", x);
    }
    x
}
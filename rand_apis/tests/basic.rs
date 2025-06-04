// tests/basic.rs

use rand_apis::{gen_random, zeroize, RandError};

#[test]
fn integration_fill_and_zero() {
    let mut buf = [0u8; 64];
    // ライブラリを依存としてインポートした状態でテスト
    gen_random(&mut buf).expect("gen_random でエラー発生");
    // ほぼ確実に一度は 0 以外のバイトが埋まるはず
    assert!(buf.iter().any(|&b| b != 0), "バッファがまったく変化していません");

    // いったんランダムが入った状態で zeroize を呼ぶ
    zeroize(&mut buf);
    assert!(buf.iter().all(|&b| b == 0), "zero_memory がすべてクリアしていません: {:?}", buf);
}

#[test]
fn integration_fill_error() {
    // 空バッファを渡すとエラー
    let mut buf: [u8; 0] = [];
    let err = gen_random(&mut buf).unwrap_err();
    // ライブラリ外から見ても同じエラーが得られること
    match err {
        RandError::InvalidInput { detail } => {
            assert!(detail.contains("buffer"), "期待した detail が含まれていません: {}", detail);
        }
        _ => panic!("Empty buffer で別のエラーが返りました: {:?}", err),
    }
}

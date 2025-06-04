#[cfg(test)]
mod tests {
    use crate::util::{gen_random, zeroize};
    use crate::error::RandError;

    /// gen_random が正しくバッファを埋めること
    #[test]
    fn test_gen_random_non_empty() {
        let mut buf = [0u8; 32];
        // 最初は全バイトが 0 であることを確認
        assert!(buf.iter().all(|&b| b == 0));
        // 填充するときにエラーが出ないこと
        gen_random(&mut buf).unwrap();
        // 少なくとも 1バイトは 0 以外になっているはず（完全に0の乱数は非常に稀なので）
        assert!(buf.iter().any(|&b| b != 0), "buf がまったく変わっていない (すべて0) かもしれません");
    }

    /// gen_random に空バッファを渡すと InvalidInput エラーになること
    #[test]
    fn test_gen_random_empty() {
        let mut buf = [];
        let err = gen_random(&mut buf).unwrap_err();
        match err {
            RandError::InvalidInput { detail } => {
                assert!(detail.contains("buffer"), "InvalidInput の detail が想定外: {}", detail);
            }
            _ => panic!("空バッファで gen_random が別のエラーを返しました: {:?}", err),
        }
    }

    /// zeroize が確実にすべてのバイトを 0 にすること
    #[test]
    fn test_zeroize() {
        // まずバッファを適当な非ゼロ値 (0xAA) で埋める
        let mut buf = [0xAAu8; 16];
        assert!(buf.iter().all(|&b| b == 0xAA));
        // zeroize でクリア
        zeroize(&mut buf);
        // すべて 0 になっているか検証
        assert!(buf.iter().all(|&b| b == 0), "zeroize でクリアできていません: {:?}", buf);
    }

    /// zeroize に空バッファを渡してもパニックしないこと
    #[test]
    fn test_zeroize_empty() {
        let mut buf: [u8; 0] = [];
        zeroize(&mut buf); // 何もせずに終わるはず
        assert!(buf.is_empty());
    }
}

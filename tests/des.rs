use des::DES;

#[test]
fn test_ecb_mode_equivalence() {
    // If you implement ECB mode, test it matches single block
    let key = 0x1334_5779_9BBC_DFF1;
    let des = DES::new(key);
    let plain = 0x0123_4567_89AB_CDEF;

    let _single_block = des.encrypt(plain);
    // let ecb_result = encrypt_ecb(&[plain]);
    // assert_eq!(single_block, ecb_result[0]);
}

#[test]
fn test_with_real_data() {
    // Test with actual 8-byte data
    let key_bytes = b"KGenius\x01";
    let key = u64::from_le_bytes(*key_bytes);

    let data_bytes = b"HelloDES!";
    let mut padded = [0u8; 8];
    padded[..data_bytes.len()].copy_from_slice(data_bytes);
    let plaintext = u64::from_le_bytes(padded);

    let des = DES::new(key);
    let encrypted = des.encrypt(plaintext);

    // Verify we can roundtrip
    let decrypted = des.decrypt(encrypted);
    let decrypted_bytes = decrypted.to_le_bytes();
    assert_eq!(decrypted_bytes[..data_bytes.len()], *data_bytes);
}

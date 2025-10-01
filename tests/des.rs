use rand::random;
use std::time::Instant;

use des::Des;

const TEST_KEY: u64 = 0x1334_5779_9BBC_DFF1;
const TEST_PLAINTEXT: u64 = 0x0123_4567_89AB_CDEF;
const TEST_CIPHERTEXT: u64 = 0x85E8_1354_0F0A_B405;

/// Helper to create a test Des instance (use your actual key schedule)
fn des_instance() -> Des {
    Des::new(TEST_KEY)
}

// #[test]
fn test_ecb_mode_equivalence() {
    // If you implement ECB mode, test it matches single block
    let key = 0x1334_5779_9BBC_DFF1;
    let des = Des::new(key);
    let plain = 0x0123_4567_89AB_CDEF;

    let _single_block = des.encrypt(plain);
    // let ecb_result = encrypt_ecb(&[plain]);
    // assert_eq!(single_block, ecb_result[0]);
}

// #[test]
fn test_with_real_data() {
    // Test with actual 8-byte data
    let key_bytes = b"KGenius\x01";
    let key = u64::from_le_bytes(*key_bytes);

    let data_bytes = b"HelloDES!";
    let mut padded = [0u8; 8];
    padded[..data_bytes.len()].copy_from_slice(data_bytes);
    let plaintext = u64::from_le_bytes(padded);

    let des = Des::new(key);
    let encrypted = des.encrypt(plaintext);

    // Verify we can roundtrip
    let decrypted = des.decrypt(encrypted);
    let decrypted_bytes = decrypted.to_le_bytes();
    assert_eq!(decrypted_bytes[..data_bytes.len()], *data_bytes);
}

// #[test]
fn all_zero_paintext() {
    let des = des_instance();

    let plain = 0;
    let encrypted = des.encrypt(plain);
    let decrypted = des.decrypt(encrypted);
    assert_eq!(decrypted, plain, "All-zero plaintext failed");
}

// #[test]
#[should_panic(expected = "Invalid key size")]
fn invalid_key_size() {
    let _ = Des::new(0);
}

// #[test]
fn encrypt_decrypt_roundtrip() {
    let des = des_instance();
    let plaintext = TEST_PLAINTEXT;
    let ciphertext = des.encrypt(plaintext);
    let dectrypted = des.decrypt(plaintext);
    let re_ciphertext = des.encrypt(dectrypted);

    assert_eq!(ciphertext, TEST_CIPHERTEXT, "Encyption failed");
    assert_eq!(re_ciphertext, TEST_CIPHERTEXT, "Re-Encyption failed");
}

// #[test]
fn weak_keys_rejected() {
    let weak_keys = [0x0101010101010101, 0xFEFEFEFEFEFEFEFE, 0xE001E001E001E001];

    for key in weak_keys {
        let des = Des::new(key);
        let plaintext = TEST_PLAINTEXT;
        let encrypted = des.encrypt(plaintext);
        let dectrypted = des.decrypt(encrypted);
        assert_eq!(dectrypted, plaintext, "Weak key {key} failed roundtrip");
    }
}

// #[test]
fn multiple_blocks() {
    let des = des_instance();
    let blocks = [
        (0x0123456789ABCDEFu64, 0x85E813540F0AB405u64),
        (0xFEDCBA9876543210u64, 0xC08BF0FF627D3E6Fu64), // Another test vector
        (0x0000000000000000u64, 0x474D5E3B6F8A07F8u64), // Zero block
    ];
    for (plaintext, expected) in blocks {
        let encrypted = des.encrypt(plaintext);
        assert_eq!(encrypted, expected, "Failed on plaintext: {plaintext:016X}");

        let dectrypted = des.decrypt(encrypted);
        assert_eq!(dectrypted, plaintext, "Roundtrip failed on block");
    }
}

// #[test]
fn all_one_paintext() {
    let des = des_instance();

    let plain = 1;
    let encrypted = des.encrypt(plain);
    let decrypted = des.decrypt(encrypted);
    assert_eq!(decrypted, plain, "All-one plaintext failed");
}

// #[test]
fn different_inputs() {
    let des = des_instance();

    let plain1 = 0x0000000000000001;
    let plain2 = 0x0000000000000002;
    let enc1 = des.encrypt(plain1);
    let enc2 = des.encrypt(plain2);
    assert_ne!(
        enc1, enc2,
        "Encryption not deterministic for different inputs"
    );
}

// #[test]
fn performance() {
    let des = des_instance();
    let plaintext = TEST_PLAINTEXT;

    let start = Instant::now();
    for _ in 0..10000 {
        let _ = des.encrypt(plaintext);
    }
    let duration = start.elapsed();

    println!("10k encryption took: {duration:?}");
    // Reasonable benchmark: should be under 1ms on modern hardware
    assert!(duration.as_millis() < 100, "Performance degraded");
}

// #[test]
fn fuzz_properties() {
    let des = des_instance();

    for _ in 0..100 {
        let plaintext = random();
        let encrypted = des.encrypt(plaintext);
        let decrypted = des.decrypt(plaintext);

        assert_eq!(decrypted, encrypted, "Fuzz roundtrip failed");
        assert_ne!(encrypted, plaintext, "Encryption is identity function");

        let key2 = random();
        if key2 != TEST_KEY {
            let des2 = Des::new(key2);
            let encrypted2 = des2.encrypt(plaintext);
            assert_ne!(
                encrypted, encrypted2,
                "Different keys produced same encryption"
            );
        }
    }
}

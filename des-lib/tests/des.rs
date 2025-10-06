use des_lib::Des;
use rstest::rstest;

const TEST_KEY: u64 = 0x1334_5779_9BBC_DFF1;
const TEST_PLAINTEXT: u64 = 0x0123_4567_89AB_CDEF;
const TEST_CIPHERTEXT: u64 = 0x85E8_1354_0F0A_B405;

/// Helper to create a test Des instance (use your actual key schedule)
fn des_instance() -> Des {
    Des::new(TEST_KEY)
}

#[rstest]
#[case(TEST_PLAINTEXT, TEST_CIPHERTEXT, TEST_KEY)]
fn encrypt_decrypt_roundtrip(
    #[case] plaintext: u64,
    #[case] expected_ciphertext: u64,
    #[case] key: u64,
) {
    let des = Des::new(key);

    let ciphertext = des.encrypt(plaintext);
    let dectrypted = des.decrypt(ciphertext);
    let re_ciphertext = des.encrypt(dectrypted);

    assert_eq!(
        ciphertext, expected_ciphertext,
        "Encyption failed. Expected 0x{ciphertext:016X}, got 0x{expected_ciphertext:016X}"
    );
    assert_eq!(
        dectrypted, plaintext,
        "Decyption failed. Expected 0x{dectrypted:016X}, got 0x{plaintext:016X}"
    );
    assert_eq!(
        re_ciphertext, expected_ciphertext,
        "Re-encyption failed. Expected 0x{re_ciphertext:016X}, got 0x{expected_ciphertext:016X}"
    );
}

#[test]
fn weak_keys_rejected() {
    let weak_keys = [0x0101010101010101, 0xFEFEFEFEFEFEFEFE, 0xE001E001E001E001];

    for key in weak_keys {
        let des = Des::new(key);
        let plaintext = TEST_PLAINTEXT;
        let encrypted = des.encrypt(plaintext);
        let dectrypted = des.decrypt(encrypted);
        assert_eq!(
            dectrypted, plaintext,
            "Weak key {key:016X} failed roundtrip"
        );
    }
}

#[test]
fn all_zero_paintext() {
    let des = des_instance();

    let plain = 0;
    let encrypted = des.encrypt(plain);
    let decrypted = des.decrypt(encrypted);
    assert_eq!(decrypted, plain, "All-zero plaintext failed");
}

#[test]
fn all_one_paintext() {
    let des = des_instance();

    let plain = u64::MAX;
    let encrypted = des.encrypt(plain);
    let decrypted = des.decrypt(encrypted);
    assert_eq!(decrypted, plain, "All-one plaintext failed");
}

#[test]
fn different_inputs() {
    let des = des_instance();

    let plain1 = 1;
    let plain2 = 2;
    let enc1 = des.encrypt(plain1);
    let enc2 = des.encrypt(plain2);
    assert_ne!(
        enc1, enc2,
        "Encryption not deterministic for different inputs"
    );
}

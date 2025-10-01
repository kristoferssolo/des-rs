mod constants;

use crate::constants::{PC1_TABLE, PC2_TABLE, PERMUTATION, ROUND_ROTATIONS};

#[derive(Debug)]
pub struct Des {
    pub subkeys: [u64; 16],
}

impl Des {
    /// Create a new DES instance from a 64-bit key (8 bytes).
    #[must_use]
    pub fn new(key: u64) -> Self {
        let mut des = Self { subkeys: [0; 16] };
        des.generate_subkeys(key);
        des
    }

    /// Encrypt a 64-bit block.
    #[must_use]
    pub fn encrypt(&self, block: u64) -> u64 {
        self.des(block, true)
    }

    /// Decrypt a 64-bit block.
    #[must_use]
    pub fn decrypt(&self, block: u64) -> u64 {
        self.des(block, false)
    }

    /// Expand the right side of the data from 32 bits to 48.
    #[must_use]
    fn expand(&self, right: u32) -> u64 {
        let bytes = right.to_le_bytes();
        dbg!(bytes);
        0
    }

    /// Feistel function: Expand, XOR with subkey, S-box, permute.
    #[must_use]
    fn feistel(&self, right: u32, subkey: u64) -> u32 {
        todo!()
    }

    /// Core DES function: encrypt if forward=true, else decrypt.
    #[must_use]
    fn des(&self, mut block: u64, forward: bool) -> u64 {
        todo!()
    }

    /// Generate 16 subkeys from the 64-bit key.
    fn generate_subkeys(&mut self, key: u64) {
        let reduced_key = pc1(key);
        let (mut left, mut right) = split_key(reduced_key);
        for (idx, &shift_num) in ROUND_ROTATIONS.iter().enumerate() {
            left = shift(left, shift_num);
            right = shift(right, shift_num);
            let combined = (u64::from(right) << 28) | u64::from(left);
            let subkey = pc2(combined);
            self.subkeys[idx] = subkey;
        }
    }

    /// Helper functions for permutations (bit manipulation)
    #[must_use]
    fn permutate(&self, input: u32, table: &[u8], n: usize) -> u32 {
        todo!()
    }

    #[must_use]
    fn ip(&self, input: u64) -> u64 {
        todo!()
    }

    #[must_use]
    pub fn fp(&self, input: u64) -> u64 {
        todo!()
    }

    fn permutate_output(&self, input: u32) -> u32 {
        self.permutate(input, &PERMUTATION, 32)
    }
}

/// Reduces 64 bits to 56-bit key by applying PC-1 permutation.
/// Selects 56 specific bits (ignoring 8 parity bits) and permutes them.
///
/// Accounts for DES specification's big-endian bit numbering (1-64, MSB first)
/// versus Rust u64's little-endian bit numbering (0-63, LSB first).
#[must_use]
pub fn pc1(key: u64) -> u64 {
    PC1_TABLE
        .iter()
        .enumerate()
        .fold(0, |mut acc, (idx, &pos)| {
            // pos is 1-based DES bit position (1-64, big-endian MSB first)
            let des_bit_1based = u64::from(pos);
            let des_bit_0based = des_bit_1based.saturating_sub(1);

            // Map DES big-endian position to u64 little-endian position
            // DES bit 1 (MSB) = u64 bit 63, DES bit 64 (LSB) = u64 bit 0
            let bit_pos = 63u64.saturating_sub(des_bit_0based);

            // Extract bit from u64 at the correct position
            let bit = ((key >> bit_pos) & 1) << (55usize.saturating_sub(idx));

            acc |= bit;
            acc
        })
}

/// Compression permuation
/// Reduces 56-bits to 48-bit key
#[must_use]
pub fn pc2(key: u64) -> u64 {
    let key_56 = key & 0x00FF_FFFF_FFFF_FFFF;

    PC2_TABLE
        .iter()
        .enumerate()
        .fold(0, |mut acc, (idx, &pos)| {
            let bit_pos = u64::from(pos).saturating_sub(1);
            let bit = ((key_56 >> bit_pos) & 1) << (47 - idx);
            acc |= bit;
            acc
        })
}

#[must_use]
const fn split_key(key: u64) -> (u32, u32) {
    let is_56_bit = (key >> 56) == 0;

    if is_56_bit {
        let masked = key & 0x00FF_FFFF_FFFF_FFFF;
        let left = (masked >> 28) & 0x0FFF_FFFF;
        let right = masked & 0x0FFF_FFFF;
        return (left as u32, right as u32);
    }

    let left = (key >> 32) & 0xFFFF_FFFF;
    let right = key & 0xFFFF_FFFF;
    (left as u32, right as u32)
}

/// Circulary shifts 28-bit number left by `shift`.
#[must_use]
const fn shift(key: u32, shift: u8) -> u32 {
    const MASK: u32 = 0x0FFF_FFFF;
    let value = key & MASK; // 28-bits

    if shift == 0 {
        return value;
    }

    // Circular left shift formula:
    // (value << shift) gets the main shifted portion
    // (value >> (28 - shift)) gets the bits that wrapped around
    let main_shifted = (value << shift) & MASK;
    let wrapped_bits = (value >> (28 - shift)) & ((1 << shift) - 1);
    (main_shifted | wrapped_bits) & MASK
}

/// Encrypts data using ECB mode.
///
/// # Arguments
/// - `data` - Plaintext bytes (must be multiple of 8 for ECB)
/// - `key` - 8-byte DES key
///
/// # Returns
///
/// Ciphertext as Vec<u8>, same length as input
///
/// # Panics
///
/// If data length is not multiple of 8 bytes
#[must_use]
pub fn encrypt_ecb(data: &[u8], key: &[u8; 8]) -> Vec<u8> {
    todo!()
}

/// Decrypts ECB-encrypted data.
///
/// # Arguments
/// - `data` - Plaintext bytes (must be multiple of 8 for ECB)
/// - `key` - 8-byte DES key
///
/// # Returns
///
/// Ciphertext as Vec<u8>, same length as input
///
/// # Panics
///
/// If data length is not multiple of 8 bytes
#[must_use]
pub fn decrypt_ecb(data: &[u8], key: &[u8; 8]) -> Vec<u8> {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::constants::S_BOXES;

    use super::*;
    use claims::assert_le;
    use rand::random;
    use std::time::Instant;

    const TEST_KEY: u64 = 0x1334_5779_9BBC_DFF1;
    const RIGHT_KEY: u32 = 0x12345678;
    const TEST_PLAINTEXT: u64 = 0x0123456789ABCDEF;
    const TEST_CIPHERTEXT: u64 = 0x85E813540F0AB405;

    impl Des {
        fn apply_sboxes(&self, input: u64) -> u32 {
            // Implementation for testing S-boxes in isolation
            // Return 32-bit result after 8 S-boxes
            todo!()
        }
    }

    /// Helper to create a test Des instance (use your actual key schedule)
    fn des_instance() -> Des {
        Des::new(TEST_KEY)
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
    fn key_schedule_generates_correct_subkeys() {
        let expected_subkeys = [
            0xF3FDFBF373848CF5u64,
            0xF3738CF548C4F3F5u64,
            0x848C4F3F5F373848u64,
        ];

        let des = des_instance();
        let generated = des.subkeys;

        for (idx, &expected) in expected_subkeys.iter().enumerate() {
            let masked_gen = generated[idx];
            let masked_exp = expected;
            assert_eq!(
                masked_gen, masked_exp,
                "Subkey {idx} mismatch: expected {masked_exp:012X}, got {masked_gen:012X}"
            );
        }
    }

    // #[test]
    fn initial_permutation() {
        let input = TEST_KEY;
        let expected_ip = 0xC2B093C7A3A7C24A;
        let result = des_instance().ip(input);
        assert_eq!(result, expected_ip, "Initial permulation failed");
    }

    #[test]
    fn pc1_permutaion_correct() {
        let key = TEST_KEY;
        let expected_pc1 = 0x00F0_CCAA_F556_678F; // Truncated 56 bits from spec

        let result = pc1(key);

        assert_eq!(result, expected_pc1, "PC1 permutation failed");
        assert_eq!(result >> 56, 0, "PC1 result should have high 8 bits as 0");
        assert_eq!(
            result & 0x00FF_FFFF_FFFF_FFFF,
            expected_pc1,
            "PC1 should be 56 bits or less"
        );
    }

    #[test]
    fn pc2_permutaion_correct() {
        let combined = 0x04FE12506091CEu64; // [D₁ << 28] | C₁
        let expected_subkey = 0xF3FDFBF373848CF5u64; // Expected 48-bit result

        let result = pc2(combined);

        assert_eq!(result, expected_subkey, "PC1 permutation failed");
        assert_eq!(result >> 56, 0, "PC2 result should have high 8 bits as 0");
        assert_eq!(
            result & 0x00FF_FFFF_FFFF_FFFF,
            expected_subkey,
            "PC2 should be 56 bits or less"
        );
    }

    // #[test]
    fn expansion_permutation() {
        let des = des_instance();
        let right_half = RIGHT_KEY;
        let expanded = des.expand(right_half);

        // Expansion should produce 48 bits from 32
        assert_eq!(expanded >> 48, 0, "Expandsion exceeds 48 bits");

        // Test that expansion duplicates bits correctly
        // Bit 0 of expanded should match bit 31 of input (EXPANSION[0]=32)
        assert_eq!(
            (expanded >> 47) & 1,
            ((right_half as u64) >> 31) & 1,
            "Expansion bit 0 failed"
        );
        // Bit 1 should match bit 0 (EXPANSION[1]=1)
        assert_eq!(
            (expanded >> 46) & 1,
            (right_half as u64) & 1,
            "Expansion bit 1 failed"
        );
        // Test wraparound: bit 47 should match bit 0 again (EXPANSION[47]=1)
        assert_eq!(
            expanded & 1,
            (right_half as u64) & 1,
            "Expansion wraparound failed"
        );
    }

    // #[test]
    fn sbox_subsitution() {
        let sbox_tests = [
            // (box_idx, 6-bit input, expected 4-bit output)
            (0, 0b000000, 14), // S1: 00 0000 -> row 0, col 0 -> 14
            (0, 0b011111, 9),  // S1: 01 1111 -> row 1, col 15 -> 9
            (1, 0b100000, 0),  // S2: 10 0000 -> row 2, col 0 -> 0
            (2, 0b001010, 2),  // S3: 00 1010 -> row 0, col 10 -> 2
        ];

        for (box_idx, input, expected) in sbox_tests {
            let row = (input & 1) | ((input >> 4) & 0x2);
            let col = (input >> 1) & 0xF;
            let val = S_BOXES[box_idx][row as usize][col as usize];

            assert_eq!(
                val,
                expected as u8,
                "S{} failed: input {input:06b} (row {row}, col {col}) expected {expected}, got {val}",
                box_idx + 1
            );
        }
    }

    // #[test]
    fn permuation_pbox() {
        let des = des_instance();
        let input = RIGHT_KEY;
        let result = des.permutate_output(input);

        // P-box should preserve all bits (32 in, 32 out), just reorder
        let bit_count = input.count_ones();
        let result_bit_count = result.count_ones();
        assert_eq!(bit_count, result_bit_count, "P-box changes bit count");

        // Test specific bit mapping: PERMUTATION[0]=16 means bit 15 (0-based) of output = bit 15 of input
        let input_bit_15 = (input >> 15) & 1;
        let output_bit_0 = (result >> 31) & 1; // MSB first
        assert_eq!(input_bit_15, output_bit_0, "P-box bit mapping failed");
    }

    // #[test]
    fn feistel_function_properties() {
        let des = des_instance();
        let right = RIGHT_KEY;
        let subkey = 0xFEDCBA9876543210 & 0xFFFF_FFFF_FFFF;

        let feistel_result = des.feistel(right, subkey);

        // Feistel output should always be 32 bits
        assert_le!(feistel_result, u32::MAX, "Feistel output exceeds 32 bits");

        // Test that zero subkey produces deterministic result
        let zero_subkey_result = des.feistel(right, 0);
        let zero_expanded = des.expand(right);
        let sbox_result = des.apply_sboxes(zero_expanded);
        let expected = des.permutate_output(sbox_result as u32);
        assert_eq!(zero_subkey_result, expected, "Feistel with zero key failed");
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
    #[should_panic(expected = "Invalid key size")]
    fn invalid_key_size() {
        let _ = Des::new(0);
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
}

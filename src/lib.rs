mod constants;

use crate::constants::{IP, PC1_TABLE, PC2_TABLE, PERMUTATION, ROUND_ROTATIONS};

#[derive(Debug)]
pub struct Des {
    pub subkeys: [u64; 16],
}

impl Des {
    /// Create a new DES instance from a 64-bit key (8 bytes).
    #[must_use]
    pub fn new(key: u64) -> Self {
        let subkeys = generate_subkeys(key);
        Self { subkeys }
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

    /// Helper functions for permutations (bit manipulation)
    #[must_use]
    fn permutate(&self, input: u32, table: &[u8], n: usize) -> u32 {
        todo!()
    }

    #[must_use]
    fn ip(&self, message: u64) -> u64 {
        apply_permutaion(message, 64, 64, &IP)
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
    apply_permutaion(key, 64, 56, &PC1_TABLE)
}

/// Compression permuation
/// Reduces 56-bits to 48-bit key
#[must_use]
pub fn pc2(key: u64) -> u64 {
    let key_56 = key & 0x00FF_FFFF_FFFF_FFFF;

    apply_permutaion(key_56, 56, 48, &PC2_TABLE)
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

/// Concatenates two 28-bit numbers into 56-bit number
#[must_use]
fn concatenate_keys(left: u32, right: u32) -> u64 {
    (u64::from(left) << 28) | u64::from(right)
}

/// Generate 16 subkeys from the 64-bit key.
fn generate_subkeys(key: u64) -> [u64; 16] {
    let reduced_key = pc1(key); // C_0, D_0
    let (mut left, mut right) = split_key(reduced_key);

    ROUND_ROTATIONS
        .iter()
        .map(|&shift_amount| {
            left = shift(left, shift_amount); // C_(n-1) -> C_n
            right = shift(right, shift_amount); // D_(n-1) -> D_n
            let combined = concatenate_keys(left, right);
            pc2(combined)
        })
        .collect::<Vec<_>>()
        .try_into()
        .expect("Exactly 16 subkeys expected")
}

/// Generic bit permutation for arbitrary input/output sizes.
///
/// # Arguments
/// - `input` - The input value (treated as a bitfield of `input_bits` size)
/// - `input_bits` - Number of meaningful bits in the input (1-64)
/// - `output_bits` - Number of bits in the output (1-64)
/// - `position_table` - 1-based positions (1 to `input_bits`) where each output bit comes from
#[must_use]
fn apply_permutaion(input: u64, input_bits: u32, output_bits: u32, position_table: &[u8]) -> u64 {
    position_table
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, &pos)| {
            // Convert 1-based DES position to 0-based input position (MSB first)
            let pos_0based = u64::from(pos.saturating_sub(1));
            let input_bit_pos = u64::from(input_bits)
                .saturating_sub(1)
                .saturating_sub(pos_0based);

            // Extract bit from input
            let bit_value = (input >> input_bit_pos) & 1;

            // Extract bit from u64 at the correct position
            let output_bit_pos = u64::from(output_bits)
                .saturating_sub(1)
                .saturating_sub(idx as u64);
            let shifted_bit = bit_value << output_bit_pos;

            acc | shifted_bit
        })
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
    use super::*;
    use crate::constants::S_BOXES;
    use claims::{assert_ge, assert_le};
    use rand::random;
    use rstest::rstest;
    use std::time::Instant;

    const TEST_KEY: u64 = 0x1334_5779_9BBC_DFF1;

    const RIGHT_KEY: u32 = 0x1234_5678;
    const TEST_PLAINTEXT: u64 = 0x0123_4567_89AB_CDEF;
    const TEST_CIPHERTEXT: u64 = 0x85E8_1354_0F0A_B405;

    const TEST_PC1_RESULT: u64 = 0x00F0_CCAA_F556_678F; // From calculator after PC-1
    const TEST_COMBINED_KEY: u64 = 0x00F0_CCAA_F556_678F; // From calculator after re-combination
    const TEST_PC2_RESULT: u64 = 0x0000_CB3D_8B0E_17F5; // From calculator after PC-2

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

    #[test]
    fn initial_permutation() {
        let expected_ip = 0xCC00_CCFF_F0AA_F0AA;
        let result = des_instance().ip(TEST_PLAINTEXT);
        assert_eq!(
            result, expected_ip,
            "Initial permulation failed {result:016X} != {expected_ip:016X}"
        );
    }

    #[test]
    fn pc1_permutaion_correct() {
        let result = pc1(TEST_KEY);

        assert_eq!(result, TEST_PC1_RESULT, "PC1 permutation failed");
        assert_ge!(
            result.leading_zeros(),
            0,
            "PC1 result should have leading 8 bits as 0"
        );
    }

    #[rstest]
    #[case(0x00F0_CCAA_F556_678F, 0xCB3D_8B0E_17F5)] // K_0
    #[case(0x00E1_9955_FAAC_CF1E, 0x1B02_EFFC_7072)] // K_1
    #[case(0x00C3_32AB_F559_9E3D, 0x79AE_D9DB_C9E5)] // K_2
    #[case(0x000C_CAAF_F566_78F5, 0x55FC_8A42_CF99)] // K_3
    #[case(0x0033_2ABF_C599_E3D5, 0x72AD_D6DB_351D)] // K_4
    #[case(0x00CC_AAFF_0667_8F55, 0x7CEC_07EB_53A8)] // K_5
    #[case(0x0032_ABFC_399E_3D55, 0x63A5_3E50_7B2F)] // K_6
    #[case(0x00CA_AFF0_C678_F556, 0xEC84_B7F6_18BC)] // K_7
    #[case(0x002A_BFC3_39E3_D559, 0xF78A_3AC1_3BFB)] // K_8
    #[case(0x0055_7F86_63C7_AAB3, 0xE0DB_EBED_E781)] // K_9
    #[case(0x0055_FE19_9F1E_AACC, 0xB1F3_47BA_464F)] // K_10
    #[case(0x0057_F866_5C7A_AB33, 0x215F_D3DE_D386)] // K_11
    #[case(0x005F_E199_51EA_ACCF, 0x7571_F594_67E9)] // K_12
    #[case(0x007F_8665_57AA_B33C, 0x97C5_D1FA_BA41)] // K_13
    #[case(0x00FE_1995_5EAA_CCF1, 0x5F43_B7F2_E73A)] // K_14
    #[case(0x00F8_6655_7AAB_33C7, 0xBF91_8D3D_3F0A)] // K_15
    #[case(0x00F0_CCAA_F556_678F, 0xCB3D_8B0E_17F5)] // K_16
    fn pc2_permutaion_correct(#[case] before: u64, #[case] after: u64) {
        let result = pc2(before);

        assert_eq!(result, after, "PC2 permutation failed");
        assert_ge!(
            result.leading_zeros(),
            16,
            "PC2 result should have leading 16 bits as 0"
        );
    }

    #[test]
    fn split_key_56_bits() {
        let (left, right) = split_key(TEST_PC1_RESULT);

        assert_eq!(left, 0x0F0C_CAAF, "split_key left half mismatch",);
        assert_eq!(right, 0x0556_678F, "split_key right half mismatch",);

        // Verify 28-bit values have 4 leading zeros in u32
        assert_ge!(
            left.leading_zeros(),
            4,
            "Left should be 28-bit value in u32"
        );
        assert_ge!(
            right.leading_zeros(),
            4,
            "Right should be 28-bit value in u32"
        );
    }

    #[rstest]
    #[case(0x0F0C_CAAF, 0x0E19_955F, 1)] // C_1
    #[case(0x0E19_955F, 0x0C33_2ABF, 1)] // C_2
    #[case(0x0C33_2ABF, 0x00CC_AAFF, 2)] // C_3
    #[case(0x00CC_AAFF, 0x0332_ABFC, 2)] // C_4
    #[case(0x0332_ABFC, 0x0CCA_AFF0, 2)] // C_5
    #[case(0x0CCA_AFF0, 0x032A_BFC3, 2)] // C_6
    #[case(0x032A_BFC3, 0x0CAA_FF0C, 2)] // C_7
    #[case(0x0CAA_FF0C, 0x02AB_FC33, 2)] // C_8
    #[case(0x02AB_FC33, 0x0557_F866, 1)] // C_9
    #[case(0x0557_F866, 0x055F_E199, 2)] // C_10
    #[case(0x055F_E199, 0x057F_8665, 2)] // C_11
    #[case(0x057F_8665, 0x05FE_1995, 2)] // C_12
    #[case(0x05FE_1995, 0x07F8_6655, 2)] // C_13
    #[case(0x07F8_6655, 0x0FE1_9955, 2)] // C_14
    #[case(0x0FE1_9955, 0x0F86_6557, 2)] // C_15
    #[case(0x0F86_6557, 0x0F0C_CAAF, 1)] // C_16
    #[case(0x0556_678F, 0x0AAC_CF1E, 1)] // D_1
    #[case(0x0AAC_CF1E, 0x0559_9E3D, 1)] // D_2
    #[case(0x0559_9E3D, 0x0566_78F5, 2)] // D_3
    #[case(0x0566_78F5, 0x0599_E3D5, 2)] // D_4
    #[case(0x0599_E3D5, 0x0667_8F55, 2)] // D_5
    #[case(0x0667_8F55, 0x099E_3D55, 2)] // D_6
    #[case(0x099E_3D55, 0x0678_F556, 2)] // D_7
    #[case(0x0678_F556, 0x09E3_D559, 2)] // D_8
    #[case(0x09E3_D559, 0x03C7_AAB3, 1)] // D_9
    #[case(0x03C7_AAB3, 0x0F1E_AACC, 2)] // D_10
    #[case(0x0F1E_AACC, 0x0C7A_AB33, 2)] // D_11
    #[case(0x0C7A_AB33, 0x01EA_ACCF, 2)] // D_12
    #[case(0x01EA_ACCF, 0x07AA_B33C, 2)] // D_13
    #[case(0x07AA_B33C, 0x0EAA_CCF1, 2)] // D_14
    #[case(0x0EAA_CCF1, 0x0AAB_33C7, 2)] // D_15
    #[case(0x0AAB_33C7, 0x0556_678F, 1)] // D_16
    fn shift_rotation(#[case] key: u32, #[case] expected_output: u32, #[case] shift_amount: u8) {
        let result = shift(key, shift_amount);
        assert_eq!(
            result, expected_output,
            "shift(0x{key:08X}, {shift_amount}) should equal 0x{expected_output:08X}"
        );

        // Verify result is still 28 bits
        assert_eq!(
            result & 0x0FFF_FFFF,
            expected_output,
            "Shift result should preserve 28 bits"
        );
        assert_ge!(
            result.leading_zeros(),
            4,
            "Shift result should be 28-bit value in u32"
        );
    }

    #[rstest]
    #[case(0x0F0C_CAAF, 0x0556_678F, 0x00F0_CCAA_F556_678F)] // CD_0
    #[case(0x0E19_955F, 0x0AAC_CF1E, 0x00E1_9955_FAAC_CF1E)] // CD_1
    #[case(0x0C33_2ABF, 0x0559_9E3D, 0x00C3_32AB_F559_9E3D)] // CD_2
    #[case(0x00CC_AAFF, 0x0566_78F5, 0x000C_CAAF_F566_78F5)] // CD_3
    #[case(0x0332_ABFC, 0x0599_E3D5, 0x0033_2ABF_C599_E3D5)] // CD_4
    #[case(0x0CCA_AFF0, 0x0667_8F55, 0x00CC_AAFF_0667_8F55)] // CD_5
    #[case(0x032A_BFC3, 0x099E_3D55, 0x0032_ABFC_399E_3D55)] // CD_6
    #[case(0x0CAA_FF0C, 0x0678_F556, 0x00CA_AFF0_C678_F556)] // CD_7
    #[case(0x02AB_FC33, 0x09E3_D559, 0x002A_BFC3_39E3_D559)] // CD_8
    #[case(0x0557_F866, 0x03C7_AAB3, 0x0055_7F86_63C7_AAB3)] // CD_9
    #[case(0x055F_E199, 0x0F1E_AACC, 0x0055_FE19_9F1E_AACC)] // CD_10
    #[case(0x057F_8665, 0x0C7A_AB33, 0x0057_F866_5C7A_AB33)] // CD_11
    #[case(0x05FE_1995, 0x01EA_ACCF, 0x005F_E199_51EA_ACCF)] // CD_12
    #[case(0x07F8_6655, 0x07AA_B33C, 0x007F_8665_57AA_B33C)] // CD_13
    #[case(0x0FE1_9955, 0x0EAA_CCF1, 0x00FE_1995_5EAA_CCF1)] // CD_14
    #[case(0x0F86_6557, 0x0AAB_33C7, 0x00F8_6655_7AAB_33C7)] // CD_15
    #[case(0x0F0C_CAAF, 0x0556_678F, 0x00F0_CCAA_F556_678F)] // CD_16
    fn key_concatenation(#[case] left: u32, #[case] right: u32, #[case] combined: u64) {
        let result = concatenate_keys(left, right);

        assert_eq!(result, combined, "{result:016X} != {combined:016X}");

        // Verify correct bit layout
        assert_eq!(
            (result >> 28) & 0x0FFF_FFFF_FFFF,
            left as u64,
            "High 28 bits should be left"
        );
        assert_eq!(
            result & 0x0FFF_FFFF,
            right as u64,
            "Low 28 bits should be right"
        );
        assert_eq!(result >> 56, 0, "Combined should fit in 56 bits");
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

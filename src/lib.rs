mod constants;

use crate::constants::{EXPANSION_TABLE, IP, PC1_TABLE, PC2_TABLE, ROUND_ROTATIONS};

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

    /// Core DES function: encrypt if forward=true, else decrypt.
    #[must_use]
    fn des(&self, block: u64, forward: bool) -> u64 {
        let permutated_block = ip(block);

        let (left, right) = if forward {
            process_feistel_rounds(permutated_block, &self.subkeys)
        } else {
            let reversed_subkeys = self.subkeys.iter().rev().copied().collect::<Vec<_>>();
            process_feistel_rounds(permutated_block, &reversed_subkeys)
        };

        let combined = concatenate_halves(right, left, 64);
        fp(combined)
    }
}

/// Reduces 64 bits to 56-bit key by applying PC-1 permutation.
/// Selects 56 specific bits (ignoring 8 parity bits) and permutes them.
///
/// Accounts for DES specification's big-endian bit numbering (1-64, MSB first)
/// versus Rust u64's little-endian bit numbering (0-63, LSB first).
#[must_use]
pub fn pc1(key: u64) -> u64 {
    permutate(key, 64, 56, &PC1_TABLE)
}

/// Compression permuation
/// Reduces 56-bits to 48-bit key
#[must_use]
pub fn pc2(key: u64) -> u64 {
    let key_56 = key & 0x00FF_FFFF_FFFF_FFFF;
    permutate(key_56, 56, 48, &PC2_TABLE)
}

#[must_use]
const fn split_block(key: u64) -> (u32, u32) {
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

/// Concatenates two `input_bits`-bit numbers into 2*`input_bits`-bit number
#[must_use]
fn concatenate_halves(left: u32, right: u32, input_bits: u32) -> u64 {
    (u64::from(left) << input_bits) | u64::from(right)
}

/// Generate 16 subkeys from the 64-bit key.
fn generate_subkeys(key: u64) -> [u64; 16] {
    let reduced_key = pc1(key); // C_0, D_0
    let (mut left, mut right) = split_block(reduced_key);

    ROUND_ROTATIONS
        .iter()
        .map(|&shift_amount| {
            left = shift(left, shift_amount); // C_(n-1) -> C_n
            right = shift(right, shift_amount); // D_(n-1) -> D_n
            let combined = concatenate_halves(left, right, 28);
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
fn permutate(input: u64, input_bits: u32, output_bits: u32, position_table: &[u8]) -> u64 {
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

#[must_use]
fn ip(message: u64) -> u64 {
    permutate(message, 64, 64, &IP)
}

/// Expand the right side of the data from 32 bits to 48.
#[must_use]
fn expansion_permutation(right: u32) -> u64 {
    permutate(u64::from(right), 32, 48, &EXPANSION_TABLE)
}

#[must_use]
fn s_box_permutation(input: u64) -> u32 {
    // Implementation for testing S-boxes in isolation
    // Return 32-bit result after 8 S-boxes
    todo!()
}

#[must_use]
fn p_box_permutation(input: u32) -> u32 {
    todo!()
}

#[must_use]
pub fn fp(input: u64) -> u64 {
    todo!()
}

/// Process 16 Feistel rounds for ECB encryption/decryption.
#[must_use]
fn process_feistel_rounds(initial_block: u64, subkeys: &[u64]) -> (u32, u32) {
    let (mut left, mut right) = split_block(initial_block);
    for &subkey in subkeys {
        (left, right) = feistel(left, right, subkey);
    }

    (right, left) // left and right should be swapped
}

/// Feistel function: Expand, XOR with subkey, S-box, permute.
/// `R_i` = `L_(i-1)` XOR f(`R_(i-1)`, `K_1`)
#[must_use]
fn feistel(left: u32, right: u32, subkey: u64) -> (u32, u32) {
    let function_output = f_function(right, subkey);
    let new_right = left ^ function_output;
    // L_i = R_(i-1)
    let new_left = right;
    (new_right, new_left)
}

fn f_function(right: u32, subkey: u64) -> u32 {
    let expanded = expansion_permutation(right);
    let xored = expanded ^ subkey;
    let sboxed = s_box_permutation(xored);
    p_box_permutation(sboxed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::S_BOXES;
    use claims::{assert_ge, assert_le};
    use rstest::rstest;

    const TEST_KEY: u64 = 0x1334_5779_9BBC_DFF1;

    const TEST_PLAINTEXT: u64 = 0x0123_4567_89AB_CDEF;
    const TEST_CIPHERTEXT: u64 = 0x85E8_1354_0F0A_B405;

    const TEST_PC1_RESULT: u64 = 0x00F0_CCAA_F556_678F; // From calculator after PC-1
    const TEST_COMBINED_KEY: u64 = 0x00F0_CCAA_F556_678F; // From calculator after re-combination
    const TEST_PC2_RESULT: u64 = 0x0000_CB3D_8B0E_17F5; // From calculator after PC-2

    #[test]
    fn initial_permutation() {
        let expected_ip = 0xCC00_CCFF_F0AA_F0AA;
        let result = ip(TEST_PLAINTEXT);
        assert_eq!(
            result, expected_ip,
            "Initial permulation failed expected 0x{expected_ip:016X}, got 0x{result:016X}"
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
        let (left, right) = split_block(TEST_PC1_RESULT);

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

    #[test]
    fn split_key_64_bits() {
        let text = ip(TEST_PLAINTEXT);
        let (left, right) = split_block(text);

        assert_eq!(left, 0x0CC0_0CCFF, "split_key left half mismatch",);
        assert_eq!(right, 0x0F0A_AF0AA, "split_key right half mismatch",);
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
    fn concatenation(#[case] left: u32, #[case] right: u32, #[case] expected: u64) {
        let result = concatenate_halves(left, right, 28);

        assert_eq!(
            result, expected,
            "0x{left:08X} and 0x{right:08X} concatination failed, expected {expected:016X}, got {result:016X}"
        );

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

    #[rstest]
    #[case(0xF0AA_F0AA, 0x7A15_557A_1555)] // Round 1
    #[case(0xEF4A_6544, 0x75EA_5430_AA09)] // Round 2
    #[case(0xCC01_7709, 0xE580_02BA_E853)] // Round 3
    #[case(0xA25C_0BF4, 0x5042_F805_7FA9)] // Round 4
    #[case(0x7722_0045, 0xBAE9_0400_020A)] // Round 5
    #[case(0x8A4F_A637, 0xC542_5FD0_C1AF)] // Round 6
    #[case(0xE967_CD69, 0xF52B_0FE5_AB53)] // Round 7
    #[case(0x064A_BA10, 0x00C2_555F_40A0)] // Round 8
    #[case(0xD569_4B90, 0x6AAB_52A5_7CA1)] // Round 9
    #[case(0x247C_C67A, 0x1083_F960_C3F4)] // Round 10
    #[case(0xB7D5_D7B2, 0x5AFE_ABEA_FDA5)] // Round 11
    #[case(0xC578_3C78, 0x60AB_F01F_83F1)] // Round 12
    #[case(0x75BD_1858, 0x3ABD_FA8F_02F0)] // Round 13
    #[case(0x18C3_155A, 0x0F16_068A_AAF4)] // Round 14
    #[case(0xC28C_960D, 0xE054_594A_C05B)] // Round 15
    #[case(0x4342_3234, 0x206A_041A_41A8)] // Round 16
    fn permutation_expansion(#[case] block: u32, #[case] expected: u64) {
        let expanded = expansion_permutation(block);

        assert_eq!(expanded, expected);
        assert_eq!(expanded >> 48, 0, "Expansion exceeds 48 bits");
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
        let input = 0x0;
        let result = p_box_permutation(input);

        // P-box should preserve all bits (32 in, 32 out), just reorder
        let bit_count = input.count_ones();
        let result_bit_count = result.count_ones();
        assert_eq!(bit_count, result_bit_count, "P-box changes bit count");

        // Test specific bit mapping: PERMUTATION[0]=16 means bit 15 (0-based) of output = bit 15 of input
        let input_bit_15 = (input >> 15) & 1;
        let output_bit_0 = (result >> 31) & 1; // MSB first
        assert_eq!(input_bit_15, output_bit_0, "P-box bit mapping failed");
    }
}

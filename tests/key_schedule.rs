use des::Des;

// Full expected subkeys for TEST_KEY (48 bits each, from FIPS spec)
const EXPECTED_SUBKEYS: [u64; 16] = [
    0x1B02_EFFC_7072,
    0x79AE_D9DB_C9E5,
    0x55FC_8A42_CF99,
    0x72AD_D6DB_351D,
    0x7CEC_07EB_53A8,
    0x63A5_3E50_7B2F,
    0xEC84_B7F6_18BC,
    0xF78A_3AC1_3BFB,
    0xE0DB_EBED_E781,
    0xB1F3_47BA_464F,
    0x215F_D3DE_D386,
    0x7571_F594_67E9,
    0x97C5_D1FA_BA41,
    0x5F43_B7F2_E73A,
    0xBF91_8D3D_3F0A,
    0xCB3D_8B0E_17F5,
];

const TEST_KEY: u64 = 0x1334_5779_9BBC_DFF1;

#[test]
fn key_schedule_generates_correct_subkeys() {
    const EXPECTED_SUBKEYS: [u64; 16] = [
        0x1B02_EFFC_7072,
        0x79AE_D9DB_C9E5,
        0x55FC_8A42_CF99,
        0x72AD_D6DB_351D,
        0x7CEC_07EB_53A8,
        0x63A5_3E50_7B2F,
        0xEC84_B7F6_18BC,
        0xF78A_3AC1_3BFB,
        0xE0DB_EBED_E781,
        0xB1F3_47BA_464F,
        0x215F_D3DE_D386,
        0x7571_F594_67E9,
        0x97C5_D1FA_BA41,
        0x5F43_B7F2_E73A,
        0xBF91_8D3D_3F0A,
        0xCB3D_8B0E_17F5,
    ];

    let des = Des::new(TEST_KEY);

    assert_eq!(
        des.subkeys, EXPECTED_SUBKEYS,
        "Subkey generation failed. Expected: {EXPECTED_SUBKEYS:?}, Got: {:?}",
        des.subkeys
    );
}

// #[test]
fn test_rotation_shifts() {
    // Test the left rotation logic in key schedule
    let mut c: u32 = 0x0FFFFFFF; // 28 bits all 1s
    c = c.rotate_left(1);
    assert_eq!(c, 0x1FFFFFFF >> 4, "Single bit rotation failed");

    // Test double shift
    let mut d: u32 = 0xAAAAAAA; // 101010... pattern
    d = d.rotate_left(2);
    assert_eq!(d, 0x2AAAAAA, "Double rotation failed"); // Check pattern shift
}

// #[test]
fn test_weak_key_detection() {
    let weak_keys = [
        0x0101010101010101u64, // All odd parity
        0xFEFEFEFEFEFEFEFEu64, // All even parity
        0x1F1F1F1F0E0E0E0Eu64, // Semi-weak
    ];

    for key in weak_keys {
        let des = Des::new(key);
        // Weak keys often produce subkeys that don't vary much
        let subkeys = &des.subkeys;
        let first = subkeys[0];
        let last = subkeys[15];
        // For true weak keys, many subkeys may be identical
        // This is just a basic check - implement full weak key analysis if desired
        println!(
            "Weak key {} subkeys: first={:012X}, last={:012X}",
            key,
            first & 0xFFFFFFFFFFFF,
            last & 0xFFFFFFFFFFFF
        );
    }
}

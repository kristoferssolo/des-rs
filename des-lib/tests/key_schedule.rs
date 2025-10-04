use des_lib::Des;

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
    let des = Des::new(TEST_KEY);

    assert_eq!(
        des.subkeys, EXPECTED_SUBKEYS,
        "Subkey generation failed. Expected: {EXPECTED_SUBKEYS:?}, Got: {:?}",
        des.subkeys
    );
}

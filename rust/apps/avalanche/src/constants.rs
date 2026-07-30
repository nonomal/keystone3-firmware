pub const BLOCKCHAIN_ID_LEN: usize = 32;
pub const SUBNET_ID_LEN: usize = 32;
pub const SUBNET_AUTH_LEN: usize = 4;
pub const NODE_ID_LEN: usize = 20;
pub const PROOF_OF_POSESSION_PUBKEY_LEN: usize = 48;
pub const PROOF_OF_POSESSION_SIGNATURE_LEN: usize = 100;
pub const C_CHAIN_ADDRESS_LEN: usize = 20;
pub const ASSET_ID_LEN: usize = 32;
pub const ADDRESS_LEN: usize = 20;
pub const NAVAX_TO_AVAX_RATIO: f64 = 1_000_000_000.0;
pub const MAINNET_NETWORK_ID: u32 = 1;
pub const FUJI_NETWORK_ID: u32 = 5;
pub const MAINNET_AVAX_ASSET_ID: [u8; ASSET_ID_LEN] = [
    33, 230, 115, 23, 203, 196, 190, 42, 235, 0, 103, 122, 214, 70, 39, 120, 168, 245, 34, 116,
    185, 214, 5, 223, 37, 145, 178, 48, 39, 168, 125, 255,
];
pub const FUJI_AVAX_ASSET_ID: [u8; ASSET_ID_LEN] = [
    61, 155, 218, 192, 237, 29, 118, 19, 48, 207, 104, 14, 253, 235, 26, 66, 21, 158, 179, 135,
    214, 210, 149, 12, 150, 247, 210, 143, 97, 187, 226, 170,
];
pub const C_CHAIN_PREFIX: &str = "m/44'/60'/0'";
pub const X_P_CHAIN_PREFIX: &str = "m/44'/9000'/0'";

pub const X_BLOCKCHAIN_ID: [u8; BLOCKCHAIN_ID_LEN] = [
    237, 95, 56, 52, 30, 67, 110, 93, 70, 226, 187, 0, 180, 93, 98, 174, 151, 209, 176, 80, 198,
    75, 198, 52, 174, 16, 98, 103, 57, 227, 92, 75,
];

pub const X_TEST_BLOCKCHAIN_ID: [u8; BLOCKCHAIN_ID_LEN] = [
    171, 104, 235, 30, 225, 66, 160, 92, 254, 118, 140, 54, 225, 31, 11, 89, 109, 181, 163, 198,
    199, 122, 171, 230, 101, 218, 217, 230, 56, 202, 148, 247,
];

pub const P_BLOCKCHAIN_ID: [u8; BLOCKCHAIN_ID_LEN] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub const C_BLOCKCHAIN_ID: [u8; BLOCKCHAIN_ID_LEN] = [
    4, 39, 212, 178, 42, 42, 120, 188, 221, 212, 86, 116, 44, 175, 145, 181, 107, 173, 191, 249,
    133, 238, 25, 174, 241, 69, 115, 231, 52, 63, 214, 82,
];

pub const C_TEST_BLOCKCHAIN_ID: [u8; BLOCKCHAIN_ID_LEN] = [
    127, 201, 61, 133, 198, 214, 44, 91, 42, 192, 181, 25, 200, 112, 16, 234, 82, 148, 1, 45, 30,
    64, 112, 48, 214, 172, 208, 2, 28, 172, 16, 213,
];

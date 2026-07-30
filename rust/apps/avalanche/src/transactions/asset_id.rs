use crate::constants::{
    FUJI_AVAX_ASSET_ID, FUJI_NETWORK_ID, MAINNET_AVAX_ASSET_ID, MAINNET_NETWORK_ID,
};
use crate::errors::AvaxError;
use bytes::Bytes;
use core::convert::TryFrom;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssetId(Bytes);

impl AssetId {
    pub fn is_native_avax(&self, network_id: u32) -> bool {
        match network_id {
            MAINNET_NETWORK_ID => self.0.as_ref() == MAINNET_AVAX_ASSET_ID,
            FUJI_NETWORK_ID => self.0.as_ref() == FUJI_AVAX_ASSET_ID,
            _ => false,
        }
    }
}

impl TryFrom<Bytes> for AssetId {
    type Error = AvaxError;

    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        Ok(AssetId(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifies_native_avax_asset_for_network() {
        let mainnet = AssetId::try_from(Bytes::copy_from_slice(&MAINNET_AVAX_ASSET_ID)).unwrap();
        let fuji = AssetId::try_from(Bytes::copy_from_slice(&FUJI_AVAX_ASSET_ID)).unwrap();

        assert!(mainnet.is_native_avax(MAINNET_NETWORK_ID));
        assert!(!mainnet.is_native_avax(FUJI_NETWORK_ID));
        assert!(fuji.is_native_avax(FUJI_NETWORK_ID));
        assert!(!fuji.is_native_avax(MAINNET_NETWORK_ID));
        assert!(!fuji.is_native_avax(999));
    }
}

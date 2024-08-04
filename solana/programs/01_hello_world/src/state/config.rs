use anchor_lang::prelude::*;

#[derive(Default, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
/// Wormhole program related addresses.
pub struct WormholeAddresses {
    /// [BridgeData](wormhole_anchor_sdk::wormhole::BridgeData) address.
    pub bridge: Pubkey,
    /// [FeeCollector](wormhole_anchor_sdk::wormhole::FeeCollector) address.
    pub fee_collector: Pubkey,
    /// [SequenceTracker](wormhole_anchor_sdk::wormhole::SequenceTracker) address.
    pub sequence: Pubkey,
}

impl WormholeAddresses {
    pub const LEN: usize =
          32 // config
        + 32 // fee_collector
        + 32 // sequence
    ;
}

#[account]
#[derive(Default)]
/// Config account data.
pub struct Config {
    /// Program's owner.
    pub owner: Pubkey,
    /// Wormhole program's relevant addresses.
    pub wormhole: WormholeAddresses,
    /// AKA nonce. Just zero, but saving this information in this account
    /// anyway.
    pub batch_id: u32,
    /// AKA consistency level. u8 representation of Solana's
    /// [Finality](wormhole_anchor_sdk::wormhole::Finality).
    pub finality: u8,
}

impl Config {
    pub const MAXIMUM_SIZE: usize = 8 // discriminator
        + 32 // owner
        + WormholeAddresses::LEN
        + 4 // batch_id
        + 1 // finality
        
    ;
    /// AKA `b"config"`.
    pub const SEED_PREFIX: &'static [u8; 6] = b"config";
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_config() -> Result<()> {
        assert!(WormholeAddresses::LEN == 96, "WormholeAddress::LEN wrong value");
        assert!(Config::MAXIMUM_SIZE == 141, "Config::MAXIMUM_SIZE wrong value");

        Ok(())
    }
}
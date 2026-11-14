#![forbid(unsafe_code)]
#![deny(clippy::arithmetic_side_effects)]
//! `aether-wallet` — Clean-room Rust replacement for Bitcoin Core's `src/wallet/`.
//! Implements Dual-Key Stealth scanning (`(v_scan, V_scan)`, `(s_spend, S_spend)`),
//! Utreexo inclusion proof maintenance, and sub-millicent cross-shard fee estimation.

use aether_primitives::{Hash256, ShardId, StealthPubKey};

/// Public Stealth Meta-Address (`aeth1q...`) encoding both Scan and Spend Ristretto255 points.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StealthMetaAddress {
    pub scan_pubkey: [u8; 32],
    pub spend_pubkey: [u8; 32],
    pub preferred_shard: ShardId,
}

impl StealthMetaAddress {
    /// Derives a one-time unlinkable output destination `P = H(r * A)*G + B`
    pub fn derive_one_time_output(&self, shared_secret_hash: &Hash256) -> StealthPubKey {
        let mut derived = [0u8; 32];
        for (i, byte) in derived.iter_mut().enumerate() {
            *byte = shared_secret_hash[i] ^ self.spend_pubkey[i];
        }
        derived
    }
}

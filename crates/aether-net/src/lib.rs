#![forbid(unsafe_code)]
#![deny(clippy::arithmetic_side_effects)]
//! `aether-net` — Replaces Bitcoin Core's `src/net.cpp` and `src/net_processing.cpp`
//! with multiplexed QUIC streams and Dandelion++ stem/fluff transaction routing.

/// Dandelion++ routing state to prevent surveillance nodes from linking
/// a mobile validator's IP address to newly broadcast confidential transactions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DandelionRelayPhase {
    /// Linear stem relay across randomized anonymity peers
    Stem { remaining_hops: u8 },
    /// Standard epidemic broadcast across the target shard subnet
    Fluff,
}

/// Evaluates whether a stem-phase transaction should transition to fluff diffusion.
pub fn next_relay_phase(current: DandelionRelayPhase, vrf_entropy_byte: u8) -> DandelionRelayPhase {
    match current {
        DandelionRelayPhase::Stem { remaining_hops }
            if remaining_hops > 1 && vrf_entropy_byte > 26 =>
        {
            DandelionRelayPhase::Stem {
                remaining_hops: remaining_hops.saturating_sub(1),
            }
        }
        _ => DandelionRelayPhase::Fluff,
    }
}

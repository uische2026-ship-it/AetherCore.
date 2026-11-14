# Bitcoin Core (C++20) to AetherCore (Rust) Subsystem Mapping

This document provides a file-by-file reference for Bitcoin Core developers navigating the `aethercore` workspace.

| Bitcoin Core Source Path | AetherCore Rust Crate & Module | Key Engineering Improvements |
| :--- | :--- | :--- |
| `src/bitcoind.cpp`, `src/init.cpp` | `crates/aether-node/src/main.rs` | Async `tokio` runtime + `rayon` thread pool replacing manual POSIX threads and global `g_context` pointers |
| `src/primitives/transaction.h` | `crates/aether-primitives/src/lib.rs` | Plaintext `CTxIn`/`CTxOut` upgraded to `ConfidentialOutPoint` + `ConfidentialTxOut` with `ShardId` routing |
| `src/pow.cpp` (`CheckProofOfWork`) | `crates/aether-consensus/src/lib.rs` | SHA-256d ASIC mining replaced with Ristretto255 `VrfSlotProof` (< 0.45ms on mobile ARM64 cores) |
| `src/validation.cpp` (`ConnectBlock`) | `crates/aether-sharding/src/lib.rs` | Global `cs_main` lock removed; 16 shards execute in parallel with atomic `CrossShardReceipt` Merkle proofs |
| `src/coins.cpp` (`CCoinsViewCache`) | `crates/aether-sharding/src/lib.rs` | Multi-gigabyte LevelDB state replaced by a stateless 48 MB `UtreexoShard` Merkle accumulator |
| `src/script/interpreter.cpp` | `crates/aether-privacy/src/lib.rs` | Homomorphic Pedersen commitment conservation (`Sum(In) - Sum(Out) == Fee*G`) + Bulletproofs+ range proofs |
| `src/net_processing.cpp` | `crates/aether-net/src/lib.rs` | Multiplexed 0-RTT QUIC transport with Dandelion++ stem/fluff routing to conceal sender IP addresses |
| `src/wallet/wallet.cpp` | `crates/aether-wallet/src/lib.rs` | Dual-Key ECDH Stealth Address derivation (`aeth1q...`) and zero-knowledge output scanning |

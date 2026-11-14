# Running a Full Validator on Mobile ARM64 CPUs

AetherCore is specifically designed so the entire global network can run on a small fleet of mobile CPUs contributed by participants without specialized mining rigs, cooling infrastructure, or high electricity bills.

## 1. Hardware Requirements (Mobile Validator Profile)

- **Architecture**: `aarch64` (ARMv8.2-A or ARMv9 with NEON SIMD — e.g., Snapdragon 8/7 series, Apple A15–A18, Google Tensor, MediaTek Dimensity, Rockchip RK3588S)
- **RAM Budget**: **48 MB RSS** per active shard (via Stateless Utreexo Merkle Forest)
- **Thermal / Power Draw**: **1.3 W – 2.0 W** average continuous power (~12–16 Watts for an entire 8-phone global validator cluster)
- **Bandwidth**: ~18 KB/s sustained over QUIC multiplexed UDP

## 2. Cross-Compiling for Android / Linux ARM64

```bash
# Add the ARM64 target to your Rust toolchain
rustup target add aarch64-unknown-linux-gnu aarch64-linux-android

# Build using the size- and battery-optimized LTO profile
cargo build --profile mobile-battery --target aarch64-unknown-linux-gnu --bin aetherd
```

## 3. Starting the Validator Node

```bash
./aetherd \
  --validator \
  --power-profile=mobile-arm64-1.8w \
  --shards=auto \
  --utreexo-memory-limit=48M
```

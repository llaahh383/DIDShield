DIDShield: A Privacy-Preserving Decentralised Identity System for Secure Digital Wallet. 
This repository provides the prototypical implementations of DIDShield.

## Reproducing the evaluation (benchmarks)

The experiments used in the paper are implemented as Rust benchmarks under `./benches/`.

### Prerequisites
- Install Rust (Cargo): https://www.rust-lang.org/tools/install

### Run all benchmarks
From the repository root:

```bash
cargo bench

Run a specific benchmark (maps to benches/<name>.rs)

cargo bench --bench app-issue
cargo bench --bench cred-issue
cargo bench --bench layer1
cargo bench --bench layer2
cargo bench --bench system-setup
cargo bench --bench vrf

# Layer 1

This crate provides a small example entity capable of generating a `did:key` pair and publishing the resulting DID Document to an Ethereum network. The `publish_to_eth` function sends the DID Document as a transaction and returns the gas used as reported by the transaction receipt.

To try it out you will need access to an Ethereum node (for example an `http://` or `https://` RPC endpoint) and a private key funded with enough ETH for gas fees.

## Example with Anvil

The `examples/anvil_publish.rs` program uses the default Anvil devnet RPC
(`http://127.0.0.1:8545`) and the first pre-funded private key printed by Anvil.

```bash
anvil &
cargo run --example anvil_publish
```

The program will publish the DID Document as a transaction and print the gas
used from the receipt.

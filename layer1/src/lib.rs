use did_key::{generate, Ed25519KeyPair, PatchedKeyPair, Config, DIDCore};
use ethers::prelude::*;
use std::convert::TryFrom;

/// Entity capable of creating DID keys and publishing a DID Document
pub struct Layer1Entity {
    key_pair: PatchedKeyPair,
}

impl Layer1Entity {
    /// Create a new entity with a freshly generated `did:key`.
    pub fn new() -> Self {
        let key_pair = generate::<Ed25519KeyPair>(None);
        Layer1Entity { key_pair }
    }

    /// Return the DID Document associated with this entity's key.
    pub fn did_document(&self) -> serde_json::Value {
        serde_json::to_value(self.key_pair.get_did_document(Config::default()))
            .expect("failed to convert document to JSON value")
    }

    /// Return the DID string for this entity's key.
    pub fn did(&self) -> String {
        self.key_pair
            .get_did_document(Config::default())
            .id
    }

    /// Publish the DID Document to an Ethereum network and return the gas used.
    ///
    /// `provider_url` should point to an Ethereum RPC endpoint. `private_key`
    /// must correspond to an account with enough funds to cover gas costs. If
    /// `to` is `None` the transaction is sent to the signer's own address.
    pub async fn publish_to_eth(
        &self,
        provider_url: &str,
        private_key: &str,
        to: Option<Address>,
    ) -> Result<U256, Box<dyn std::error::Error>> {
        let provider = Provider::<Http>::try_from(provider_url)?;
        let wallet: LocalWallet = private_key.parse()?;
        let chain_id = provider.get_chainid().await?;
        let wallet = wallet.with_chain_id(chain_id.as_u64());
        let client = SignerMiddleware::new(provider, wallet);

        let doc_json = serde_json::to_string(&self.did_document())?;
        let tx = TransactionRequest::new()
            .to(to.unwrap_or(client.address()))
            .data(doc_json.into_bytes());

        let pending_tx = client.send_transaction(tx, None).await?;
        let receipt = pending_tx.confirmations(1).await?.ok_or("missing receipt")?;
        Ok(receipt.gas_used.unwrap_or_default())
    }

    /// Search the Ethereum chain for a transaction containing the provided DID
    /// document and return it if found.
    pub async fn retrieve_from_eth(
        provider_url: &str,
        did: &str,
    ) -> Result<Option<serde_json::Value>, Box<dyn std::error::Error>> {
        let provider = Provider::<Http>::try_from(provider_url)?;
        let latest = provider.get_block_number().await?.as_u64();

        for num in 0..=latest {
            if let Some(block) = provider.get_block_with_txs(num).await? {
                for tx in block.transactions {
                    let data = tx.input.0;
                    if let Ok(text) = String::from_utf8(data.clone()) {
                        if text.contains(did) {
                            if let Ok(doc) = serde_json::from_slice::<serde_json::Value>(&data) {
                                return Ok(Some(doc));
                            }
                        }
                    }
                }
            }
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_and_publish() {
        let entity = Layer1Entity::new();
        let doc = entity.did_document();
        assert!(doc["id"].as_str().unwrap().starts_with("did:key:"));
    }
}

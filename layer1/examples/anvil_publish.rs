use layer1::Layer1Entity;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let entity = Layer1Entity::new();
    let gas = entity
        .publish_to_eth(
            "http://127.0.0.1:8545",
            "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
            None,
        )
        .await?;
    println!("Published DID Document. Gas used: {}", gas);
    Ok(())
}

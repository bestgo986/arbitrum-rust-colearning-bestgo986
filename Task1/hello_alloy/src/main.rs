use anyhow::{Context, Result};
use ethers::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {

    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";

    let provider = Provider::<Http>::try_from(rpc_url)
        .with_context(|| format!("RPC 连接失败：{rpc_url}"))?;

 
    let latest_block = provider
        .get_block_number()
        .await
        .context("获取区块号失败")?;

    println!("Latest block number: {latest_block}");
    println!("Hello web3");

    Ok(())
}

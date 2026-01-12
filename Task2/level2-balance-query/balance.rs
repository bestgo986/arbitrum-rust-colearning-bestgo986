use anyhow::{Context, Result};
use ethers::prelude::*;
use ethers::utils::format_ether;
use std::{env, str::FromStr};

const DEFAULT_RPC: &str = "https://sepolia-rollup.arbitrum.io/rpc";
const DEFAULT_ADDR: &str = "0x66BD2C13FC975E20f95aC3e7fAC9fBe4401F2317"; 

async fn query(address: Address, rpc_url: &str) -> Result<(U256, String)> {
    let provider = Provider::<Http>::try_from(rpc_url)
        .with_context(|| format!("RPC 初始化失败: {rpc_url}"))?;
    let wei = provider.get_balance(address, None).await.context("get_balance 失败")?;
    Ok((wei, format_ether(wei)))
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = env::args().skip(1);

    let address_str = args.next().unwrap_or_else(|| DEFAULT_ADDR.to_string());
    let rpc_url = args.next().unwrap_or_else(|| DEFAULT_RPC.to_string());

    let address = Address::from_str(&address_str)
        .with_context(|| format!("地址格式不正确: {address_str}"))?;

    let (wei, eth) = query(address, &rpc_url).await?;

    println!("Address: {:?}", address);
    println!("RPC    : {}", rpc_url);
    println!("ETH    : {}", eth);
    println!(" Wei    : {}", wei);

    Ok(())
}

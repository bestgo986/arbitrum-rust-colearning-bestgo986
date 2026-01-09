use alloy::providers::{Provider, ProviderBuilder};
use alloy::primitives::Address;
use alloy::sol;
use std::error::Error;

/// 合约 ABI 绑定（逻辑不变）
sol! {
    #[sol(rpc)]
    contract HelloWeb3 {
        function hello_web3() pure public returns (string memory);
    }
}

/// 创建 Provider
fn build_provider(rpc_url: &str) -> Result<Provider, Box<dyn Error>> {
    let url = rpc_url.parse()?;
    let provider = ProviderBuilder::new().connect_http(url);
    Ok(provider)
}

/// 查询最新区块号
async fn fetch_latest_block(provider: &Provider) -> Result<u64, Box<dyn Error>> {
    let block_number = provider.get_block_number().await?;
    Ok(block_number)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. RPC 地址
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com";
    let provider = build_provider(rpc_url)?;

    // 2. 查询区块高度
    let latest_block = fetch_latest_block(&provider).await?;
    println!("Latest block number: {}", latest_block);

    // 3. 合约地址
    let contract_address: Address =
        "0x3f1f78ED98Cd180794f1346F5bD379D5Ec47DE90".parse()?;

    // 4. 合约实例
    let contract = HelloWeb3::new(contract_address, provider);

    // 5. 调用合约方法
    let result = contract.hello_web3().call().await?;
    println!("合约返回: {}", result);

    Ok(())
}


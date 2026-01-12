use ethers::prelude::*;
use std::convert::TryFrom;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 连到 Arbitrum Sepolia 测试网
    let provider = Provider::<Http>::try_from("https://sepolia-rollup.arbitrum.io/rpc")?;

    // 2. 动态获取实时 gas 价格（单位 Wei）
    let gas_price = provider.get_gas_price().await?;
    println!("当前 Arbitrum Sepolia  gasPrice = {} Wei", gas_price);

    // 3. 基础转账的 gasLimit 业界通用值 21000
    let gas_limit = 21_000;
    println!("基础转账固定 gasLimit = {}", gas_limit);

    // 4. 计算预估手续费：gas 费 = gasPrice * gasLimit
    let fee_wei = gas_price * U256::from(gas_limit);
    let fee_gwei = fee_wei / U256::from(1_000_000_000);
    println!("预估转账手续费 = {} Gwei", fee_gwei);

    Ok(())
}
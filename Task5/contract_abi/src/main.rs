use ethers::{
    prelude::*,
    types::{Address, H160, U256},
};
use eyre::Result;
use std::str::FromStr;
use std::sync::Arc;

// Multicall3 ABI 片段（只包含我们需要的 view 方法）
abigen!(
    Multicall3,
    r#"[
        function getBlockNumber() external view returns (uint256)
        function getBlockHash(uint256 blockNumber) external view returns (bytes32)
        function getCurrentBlockCoinbase() external view returns (address)
        function getEthBalance(address addr) external view returns (uint256)
        function getLastBlockHash() external view returns (bytes32)
    ]"#
);

#[tokio::main]
async fn main() -> Result<()> {
    // Arbitrum Sepolia RPC（公共节点，慢的话换 Alchemy/Infura）
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let client = Arc::new(provider);

    // Multicall3 合约地址（Arbitrum Sepolia 上已部署）
    let contract_addr = Address::from_str("0xcA11bde05977b3631167028862bE2a173976CA11")?;

    let multicall = Multicall3::new(contract_addr, client);

    // 1. 获取当前块号
    let block_number = multicall.get_block_number().call().await?;
    println!("Current block number: {}", block_number);

    // 2. 获取最后一个块的 hash
    let last_hash = multicall.get_last_block_hash().call().await?;
    println!("Last block hash: 0x{}", hex::encode(last_hash));

    // 3. 获取当前 coinbase（矿工地址，在 L2 上通常是 sequencer 相关）
    let coinbase = multicall.get_current_block_coinbase().call().await?;
    println!("Current block coinbase: {}", coinbase);

    // 4. 查询某个地址的 ETH 余额（这里用零地址示例，可换成你的测试钱包）
    let example_addr = H160::from_str("0x6a9270b0b307EA8BE1BD07c4C1AEb7C707faD9FA")?;
    let eth_balance = multicall.get_eth_balance(example_addr).call().await?;
    println!("ETH balance of zero address: {} wei", eth_balance);

    // 5. 示例：查询前一个块的 hash（当前块 -1）
    let prev_block = U256::from(block_number - 1);
    let prev_hash = multicall.get_block_hash(prev_block).call().await?;
    println!("Previous block hash: 0x{}", hex::encode(prev_hash));

    Ok(())
}
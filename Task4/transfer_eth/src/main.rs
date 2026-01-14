use ethers::prelude::*;
use dotenv::dotenv;
use std::env;
use std::convert::TryFrom;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. 加载环境变量
    dotenv().ok();

    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let to_address_str = env::var("TO_ADDRESS").expect("TO_ADDRESS must be set");

    // 2. 连接 Provider
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // 3. 创建 Wallet (签名器)
    let chain_id = provider.get_chainid().await?;
    let wallet: LocalWallet = private_key
        .parse::<LocalWallet>()?
        .with_chain_id(chain_id.as_u64());

    let from_address = wallet.address();
    println!("🚀 发送方地址: {:?}", from_address);

    // 4. 构建 Client (Provider + Wallet)
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());

    // 5. 准备交易参数
    let to_address: Address = to_address_str.parse()?;
    // 转账 0.0001 ETH (为了省钱，测试用)
    let value = U256::from(100000000000000u64);

    println!("📊 正在获取并加倍 Gas 费率 (EIP-1559)...");

    // 获取 EIP-1559 估算值
    let (max_fee, priority_fee) = provider.estimate_eip1559_fees(None).await?;

    // ⚠️ 暴力优化：直接乘以 2 倍，防止 Arbitrum 测试网波动导致交易失败
    let adjusted_max_fee = max_fee * 2;
    let adjusted_priority_fee = priority_fee * 2;

    println!("   原估算 MaxFee: {:?}", max_fee);
    println!("   调整后 MaxFee: {:?}", adjusted_max_fee);

    // 🛠️ 关键修改：使用 Eip1559TransactionRequest 专用结构体
    // 这样 max_fee_per_gas 方法一定存在，不会报错
    let tx = Eip1559TransactionRequest::new()
        .to(to_address)
        .value(value)
        .from(from_address)
        .max_fee_per_gas(adjusted_max_fee)
        .max_priority_fee_per_gas(adjusted_priority_fee);

    println!("💸 正在向 {:?} 发送 {} Wei...", to_address, value);

    // 6. 发送交易
    // 注意：这里 send_transaction 会自动处理类型转换
    let pending_tx = client.send_transaction(tx, None).await?;

    println!("⏳ 交易已广播，Hash: {:?}", pending_tx.tx_hash());
    println!("等待链上确认...");

    // 7. 等待回执
    let receipt = pending_tx.await?;

    match receipt {
        Some(r) => {
            println!("✅ 交易成功！");
            println!("   Block Number: {:?}", r.block_number);
            println!("   Gas Used: {:?}", r.gas_used);
            println!("   Explorer Link: https://sepolia.arbiscan.io/tx/{:?}", r.transaction_hash);
        },
        None => println!("❌ 交易未能在预期时间内确认。"),
    }

    Ok(())
}
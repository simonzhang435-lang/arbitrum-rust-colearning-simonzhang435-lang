use ethers::prelude::*;
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use dotenv::dotenv;
use std::env;
use ethers::types::{U256, Eip1559TransactionRequest};
use crate::config::NetworkConfig;

/// 创建 Provider 连接
pub fn create_provider(config: &NetworkConfig) -> Result<Provider<Http>, Box<dyn Error>> {
    let provider = Provider::<Http>::try_from(config.rpc_url.as_str())?;
    Ok(provider)
}

/// 获取当前 Gas 价格
pub async fn get_gas_price(provider: &Provider<Http>) -> Result<U256, Box<dyn Error>> {
    let gas_price = provider.get_gas_price().await?;
    Ok(gas_price)
}

/// 计算预估的转账 Gas 费用
pub fn estimate_transfer_fee(gas_price: U256, gas_limit: u64) -> U256 {
    gas_price * U256::from(gas_limit)
}

/// 查询指定地址的 ETH 余额
pub async fn query_eth_balance(
    provider: &Provider<Http>,
    address: &str,
) -> Result<String, Box<dyn Error>> {
    use ethers::utils::format_ether;

    // 解析地址
    let address_parsed = Address::from_str(address)?;

    // 查询余额（单位：Wei）
    let balance_wei = provider.get_balance(address_parsed, None).await?;

    // Wei -> ETH（人类可读）
    let balance_eth = format_ether(balance_wei);

    // 格式化地址
    let address_str = format!("{:#x}", address_parsed);

    // 组合返回字符串
    let result = format!(
        "📍 钱包地址：{}\n   余额 (Wei): {}\n   余额 (ETH): {}",
        address_str, balance_wei, balance_eth
    );

    Ok(result)
}

/// 执行 ETH 转账 (EIP-1559 模式)
/// 
/// # 参数
/// * `provider` - HTTP Provider
/// * `to_address` - 目标地址字符串
/// * `amount_eth` - 转账金额 (ETH)
/// * `chain_id` - 链 ID
/// 
/// # 返回
/// * `Result<String, Box<dyn Error>>` - 交易 Hash
pub async fn execute_transfer(
    provider: Provider<Http>,
    to_address: &str,
    amount_eth: f64,
    chain_id: u64,
) -> Result<String, Box<dyn Error>> {
    // 1. 加载环境变量
    dotenv().ok();
    let private_key = env::var("PRIVATE_KEY")
        .map_err(|_| "❌ 未找到 PRIVATE_KEY 环境变量，请在 .env 文件中配置")?;

    // 2. 创建钱包并绑定 Chain ID
    let wallet: LocalWallet = private_key
        .parse::<LocalWallet>()?
        .with_chain_id(chain_id);
    
    let from_address = wallet.address();
    println!("   🔑 使用钱包: {:?}", from_address);

    // 3. 创建 SignerMiddleware
    let client = SignerMiddleware::new(provider.clone(), wallet);
    let client_arc = Arc::new(client);

    // 4. 解析目标地址
    let to: Address = Address::from_str(to_address)?;

    // 5. 转换金额 ETH -> Wei
    let amount_wei = crate::utils::eth_to_wei(amount_eth);

    // 6. 估算 EIP-1559 费用 (推荐方式)
    // 自动获取网络当前的 Max Fee (BaseFee + Buffer) 和 Max Priority Fee
    let (max_fee, max_priority_fee) = provider.estimate_eip1559_fees(None).await?;
    
    // 打印费用信息
    use crate::utils::wei_to_gwei;
    println!("   ⛽ EIP-1559 费用估算:");
    println!("      Max Fee (总上限): {:.4} Gwei", wei_to_gwei(max_fee));
    println!("      Priority Fee (小费): {:.4} Gwei", wei_to_gwei(max_priority_fee));

    // 7. 构建 EIP-1559 交易请求
    let tx = Eip1559TransactionRequest::new()
        .to(to)
        .value(amount_wei)
        .from(from_address)
        .max_fee_per_gas(max_fee)
        .max_priority_fee_per_gas(max_priority_fee);

    // 8. 发送交易
    println!("   🚀 正在发送交易...");
    println!("      从: {:?}", from_address);
    println!("      到: {:?}", to);
    println!("      金额: {} ETH", amount_eth);

    let pending_tx = client_arc.send_transaction(tx, None).await?;
    
    // 9. 等待交易确认
    println!("   ⏳ 等待交易确认 (Hash: {:?})...", pending_tx.tx_hash());
    let receipt = pending_tx.await?;

    // 10. 返回结果
    match receipt {
        Some(r) => Ok(format!("{:?}", r.transaction_hash)),
        None => Err("❌ 交易未被确认（可能被丢弃）".into()),
    }
}

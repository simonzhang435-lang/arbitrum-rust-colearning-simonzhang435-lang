use ethers::providers::{Http, Middleware, Provider};
use ethers::types::U256;
use std::error::Error;

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
    use ethers::types::Address;
    use ethers::utils::format_ether;
    use std::str::FromStr;

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

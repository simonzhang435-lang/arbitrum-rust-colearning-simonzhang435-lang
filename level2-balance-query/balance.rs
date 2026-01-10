use ethers::providers::{Http,Middleware, Provider};
use ethers::types::Address;
use ethers::utils::format_ether;
use std::error::Error;
use std::str::FromStr;

/// 使用 ethers-rs 查询 ETH 余额（返回 ETH 单位）
pub async fn query_eth_balance() -> Result<String, Box<dyn Error>> {
    // 1. 创建 Provider
    let provider = Provider::<Http>::try_from("https://arbitrum-sepolia-rpc.publicnode.com")?;

    // 2. 解析地址
    let address = Address::from_str("0x7531d89aeffAc1B42DfF2e4B0Af1862d89041C35")?;

    // 3. 查询余额（单位：wei）
    let balance_wei = provider.get_balance(address, None).await?;

    // 4. wei -> ETH（人类可读）
    let balance_eth = format_ether(balance_wei);

        // 5. 组合返回字符串
    let address_str = format!("{:#x}", address);

    let result = format!(
        " 钱包地址：{} 余额 wei单位为: {} | ETH单位为: {} ",
        address_str,
        balance_wei,
        balance_eth
    );

    Ok(result)

}

use ethers::types::U256;

/// 将 Wei 转换为 Gwei (1 Gwei = 10^9 Wei)
pub fn wei_to_gwei(wei: U256) -> f64 {
    let gwei_divisor = U256::from(1_000_000_000u64);
    let gwei = wei.as_u128() as f64 / gwei_divisor.as_u128() as f64;
    gwei
}

/// 将 Wei 转换为 ETH (1 ETH = 10^18 Wei)
pub fn wei_to_eth(wei: U256) -> f64 {
    let eth_divisor = U256::from(1_000_000_000_000_000_000u128);
    let eth = wei.as_u128() as f64 / eth_divisor.as_u128() as f64;
    eth
}

/// 将 Gwei 转换为 Wei (1 Gwei = 10^9 Wei)
#[allow(dead_code)]
pub fn gwei_to_wei(gwei: f64) -> U256 {
    let wei = gwei * 1_000_000_000.0;
    U256::from(wei as u128)
}

/// 将 ETH 转换为 Wei (1 ETH = 10^18 Wei)
#[allow(dead_code)]
pub fn eth_to_wei(eth: f64) -> U256 {
    let wei = eth * 1_000_000_000_000_000_000.0;
    U256::from(wei as u128)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wei_to_gwei() {
        let wei = U256::from(1_000_000_000u64);
        assert_eq!(wei_to_gwei(wei), 1.0);
    }

    #[test]
    fn test_wei_to_eth() {
        let wei = U256::from(1_000_000_000_000_000_000u128);
        assert_eq!(wei_to_eth(wei), 1.0);
    }

    #[test]
    fn test_gwei_to_wei() {
        let gwei = 1.0;
        assert_eq!(gwei_to_wei(gwei), U256::from(1_000_000_000u64));
    }

    #[test]
    fn test_eth_to_wei() {
        let eth = 1.0;
        assert_eq!(eth_to_wei(eth), U256::from(1_000_000_000_000_000_000u128));
    }
}

// ============ 地址工具函数 ============

use ethers::types::Address;
use std::str::FromStr;

/// 解析地址字符串
pub fn parse_address(addr_str: &str) -> Result<Address, Box<dyn std::error::Error>> {
    let address = Address::from_str(addr_str)?;
    Ok(address)
}

/// 格式化地址为字符串（带 0x 前缀）
pub fn format_address(address: Address) -> String {
    format!("{:#x}", address)
}

#[cfg(test)]
mod address_tests {
    use super::*;

    #[test]
    fn test_parse_address() {
        let addr_str = "0x7531d89aeffAc1B42DfF2e4B0Af1862d89041C35";
        let result = parse_address(addr_str);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_address() {
        let addr_str = "0x7531d89aeffAc1B42DfF2e4B0Af1862d89041C35";
        let address = parse_address(addr_str).unwrap();
        let formatted = format_address(address);
        assert_eq!(formatted.to_lowercase(), addr_str.to_lowercase());
    }
}


use ethers::types::U256;

/// 网络配置结构体
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    /// 网络名称
    pub name: String,
    /// RPC 节点 URL
    pub rpc_url: String,
    /// 链 ID
    pub chain_id: u64,
    /// 基础 ETH 转账的 Gas 限额
    pub base_gas_limit: u64,
    /// 查询余额的钱包地址（可选）
    pub wallet_address: Option<String>,
    /// HelloWeb3 合约地址（可选）
    pub hello_web3_contract: Option<String>,
}

impl NetworkConfig {
    /// Arbitrum Sepolia 测试网配置
    pub fn arbitrum_sepolia() -> Self {
        Self {
            name: "Arbitrum Sepolia".to_string(),
            rpc_url: "https://sepolia-rollup.arbitrum.io/rpc".to_string(),
            chain_id: 421614,
            base_gas_limit: 21_000,
            wallet_address: Some("0x7531d89aeffAc1B42DfF2e4B0Af1862d89041C35".to_string()),
            hello_web3_contract: Some("0x3f1f78ED98Cd180794f1346F5bD379D5Ec47DE90".to_string()),
        }
    }

    /// Arbitrum One 主网配置
    #[allow(dead_code)]
    pub fn arbitrum_mainnet() -> Self {
        Self {
            name: "Arbitrum One".to_string(),
            rpc_url: "https://arb1.arbitrum.io/rpc".to_string(),
            chain_id: 42161,
            base_gas_limit: 21_000,
            wallet_address: None,  // 主网地址需要用户配置
            hello_web3_contract: None,  // 主网合约需要用户配置
        }
    }

    /// 自定义网络配置
    #[allow(dead_code)]
    pub fn custom(
        name: String,
        rpc_url: String,
        chain_id: u64,
        base_gas_limit: u64,
        wallet_address: Option<String>,
        hello_web3_contract: Option<String>,
    ) -> Self {
        Self {
            name,
            rpc_url,
            chain_id,
            base_gas_limit,
            wallet_address,
            hello_web3_contract,
        }
    }
}

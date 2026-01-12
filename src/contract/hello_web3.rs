use alloy::primitives::Address;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::sol;
use std::error::Error;

// 定义 HelloWeb3 合约 ABI
sol! {
    #[sol(rpc)]
    contract HelloWeb3 {
        function hello_web3() pure public returns(string memory);
    }
}

/// 调用 HelloWeb3 合约的 hello_web3 方法
pub async fn call_hello_web3(
    rpc_url: &str,
    contract_address: &str,
) -> Result<String, Box<dyn Error>> {
    // 使用 alloy 创建 HTTP provider
    let provider = ProviderBuilder::new().connect_http(rpc_url.parse()?);
    
    // 解析合约地址
    let address: Address = contract_address.parse()?;
    
    // 创建合约实例
    let contract = HelloWeb3::new(address, provider);
    
    // 调用合约方法
    let result = contract.hello_web3().call().await?;
    
    // 返回结果
    Ok(result)
}

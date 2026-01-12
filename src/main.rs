mod config;
mod net;
mod utils;
mod contract;  // 新增合约模块

use config::NetworkConfig;
use net::{create_provider, estimate_transfer_fee, get_gas_price};
use std::error::Error;
use utils::{wei_to_eth, wei_to_gwei};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {


    // 使用配置模块获取网络配置
    let config = NetworkConfig::arbitrum_sepolia();
    println!("✓ 网络: {}", config.name);
    println!("✓ Chain ID: {}", config.chain_id);

    // 使用网络模块创建 Provider
    let provider = create_provider(&config)?;
    println!("✓ 已连接到 RPC: {}", config.rpc_url);

        // TASK1: 调用智能合约
    println!("\n=== TASK1 智能合约调用 ===\n");
    
    if let Some(contract_addr) = &config.hello_web3_contract {
        match contract::call_hello_web3(&config.rpc_url, contract_addr).await {
            Ok(result) => println!("TASK1_合约返回: {}", result),
            Err(e) => eprintln!("❌ 合约调用失败: {}", e),
        }
    } else {
        println!("⚠️  未配置合约地址，跳过合约调用");
    }

    // 查询地址余额
    println!("\n=== TASK2 钱包余额查询 ===\n");
    
    if let Some(wallet_address) = &config.wallet_address {
        match net::query_eth_balance(&provider, wallet_address).await {
            Ok(balance_info) => println!("{}", balance_info),
            Err(e) => eprintln!("❌ 查询余额失败: {}", e),
        }
    } else {
        println!("⚠️  未配置钱包地址，跳过余额查询");
    }

    println!("===TASK3 Arbitrum Sepolia 测试网 Gas 费用预估 ===\n");
    // 获取当前 Gas 价格
    let gas_price = get_gas_price(&provider).await?;
    let gas_price_gwei = wei_to_gwei(gas_price);
    println!(
        "✓ 当前 Gas 价格: {:.4} Gwei ({} Wei)",
        gas_price_gwei, gas_price
    );

    // 基础转账 Gas 限额
    println!("✓ 基础转账 Gas 限额: {} Gas", config.base_gas_limit);

    // 计算预估 Gas 费用
    let estimated_gas_fee = estimate_transfer_fee(gas_price, config.base_gas_limit);
    let gas_fee_gwei = wei_to_gwei(estimated_gas_fee);
    let gas_fee_eth = wei_to_eth(estimated_gas_fee);

    println!("\n📊 预估 Gas 费用:");
    println!("  ├─ {:.4} Gwei", gas_fee_gwei);
    println!("  ├─ {:.10} ETH", gas_fee_eth);
    println!("  └─ {} Wei", estimated_gas_fee);

    println!("\n💡 计算公式: Gas费 = Gas价格 × Gas限额");
    println!(
        "   {} Wei × {} = {} Wei",
        gas_price, config.base_gas_limit, estimated_gas_fee
    );

    Ok(())
}

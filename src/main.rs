use ethers::contract::abigen;
mod config;
mod net;
mod utils;
mod contract;  // 新增合约模块
use config::NetworkConfig;
use net::{create_provider, estimate_transfer_fee, get_gas_price,execute_transfer};
use std::error::Error;
use utils::{wei_to_eth, wei_to_gwei};
use std::sync::Arc;


// 1. 生成合约绑定
// 从 erc20_abi.json 文件生成类型安全的合约接口
abigen!(
    ERC20Contract,
    "src/erc20_abi.json"
);


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


    // TASK4: ETH 转账
    println!("\n=== TASK4 ETH 转账 ===\n");
    
    // 转账金额 (ETH)
    let transfer_amount = 0.0001; 

    if let Some(target_addr) = &config.target_address {
        println!("📝 准备转账:");
        println!("   目标地址: {}", target_addr);
        println!("   转账金额: {} ETH", transfer_amount);
        
        // 执行转账
        match net::execute_transfer(provider.clone(), target_addr, transfer_amount, config.chain_id).await {
            Ok(tx_hash) => {
                println!("✅ 转账成功!");
                println!("   交易 Hash: {}", tx_hash);
                println!("   浏览器查看: https://sepolia.arbiscan.io/tx/{}", tx_hash);
            },
            Err(e) => eprintln!("❌ 转账失败: {}", e),
        }

    } else {
        println!("⚠️  未配置目标地址，跳过转账");
    }


    println!("=== TASK5 Arbitrum 简单合约交互 (只读) ===\n");

    // 1. 加载配置
    let config = NetworkConfig::arbitrum_sepolia();
    println!("✓ 网络: {}", config.name);
    
    // 2. 连接 RPC
    //let provider = create_provider(&config)?;
    let provider_arc = Arc::new(provider);
    println!("✓ 已连接到 RPC");

    // 3. 目标合约: WETH (Arbitrum Sepolia)
    //let contract_address_str = "0x980B62Da83eFf3D4576C647993b0c1D7faf17c73";
    let contract_address_str = "0xbC47901f4d2C5fc871ae0037Ea05c3F614690781";
    let contract_address: ethers::types::Address = contract_address_str.parse()?;
    println!("✓ 目标合约地址: {}", contract_address_str);

    // 4. 实例化合约
    // 注意：这里我们使用 只读 的 Arc<Provider>
    let contract = ERC20Contract::new(contract_address, provider_arc.clone());

    // 5. 调用只读方法
    println!("\n📊 正在读取合约状态...");

    // 调用 name()
    let name = contract.name().call().await?;
    println!("  🔹 合约名称 (name): {}", name);

    // 调用 symbol()
    let symbol = contract.symbol().call().await?;
    println!("  🔹 代币符号 (symbol): {}", symbol);

    // 调用 decimals()
    let decimals = contract.decimals().call().await?;
    println!("  🔹 精度 (decimals): {}", decimals);

    // 调用 totalSupply()
    let total_supply = contract.total_supply().call().await?;
    // 简单的格式化，除以 10^decimals
    let total_supply_fmt = utils::wei_to_eth(total_supply); // 假设精度是18，WETH通常是
    println!("  🔹 总供应量 (totalSupply): {} (Wei: {})", total_supply_fmt, total_supply);

    println!("\n✅ 合约交互成功！");

    Ok(())
}

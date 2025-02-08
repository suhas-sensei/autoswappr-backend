use autoswappr_backend::utils::ethereum::get_token_allowance;
use ethers::types::{Address, U256};
use ethers::providers::{Provider, Http};
use std::sync::Arc;
use std::str::FromStr;

// Helper function to create test provider
fn get_test_provider() -> Arc<Provider<Http>> {
    println!("Creating provider connection...");
    let provider = Provider::<Http>::try_from(
        "https://eth-mainnet.g.alchemy.com/v2/WzbJ87UBUy1zLNQeq-V21MNrv24ML_EP"
    ).expect("Failed to create provider");
    println!("Provider created successfully");
    Arc::new(provider)
}

#[tokio::test]
async fn test_get_token_allowance_zero() {
    let provider = get_test_provider();
    
    // USDC token address on mainnet
    let token = Address::from_str("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48").unwrap();
    let owner = Address::from_str("0x0000000000000000000000000000000000000000").unwrap();
    let spender = Address::from_str("0x0000000000000000000000000000000000000000").unwrap();
    
    let result = get_token_allowance(provider, token, owner, spender).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), U256::zero());
}

#[tokio::test]
async fn test_get_token_allowance_max() {
    let provider = get_test_provider();
    
    println!("Starting allowance test with USDC 3pool pair...");
    
    // USDC token address
    let token = Address::from_str("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48")
        .expect("Invalid USDC address");
    
    // Using Curve.fi DAI/USDC/USDT Pool contract as owner
    let owner = Address::from_str("0xbEbc44782C7dB0a1A60Cb6fe97d0b483032FF1C7")
        .expect("Invalid owner address");
    
    // Using Curve.fi Pool Registry as spender
    let spender = Address::from_str("0x90E00ACe148ca3b23Ac1bC8C240C2a7Dd9c2d7f5")
        .expect("Invalid spender address");

    println!("Checking allowance for:");
    println!("Token (USDC): {:?}", token);
    println!("Owner (3pool): {:?}", owner);
    println!("Spender (Registry): {:?}", spender);

    let result = get_token_allowance(provider, token, owner, spender).await;
    
    match &result {
        Ok(amount) => println!("Successfully retrieved allowance: {}", amount),
        Err(e) => println!("Error retrieving allowance: {:?}", e),
    }

    assert!(result.is_ok(), "Failed to get allowance");
    let allowance = result.unwrap();

    println!("Final allowance value: {}", allowance);
    
    // For Curve pools, we expect either max approval or at least a large value
    let max_approval = U256::from_dec_str("115792089237316195423570985008687907853269984665640564039457584007913129639935").unwrap();
    assert!(
        allowance > U256::from(1_000_000_000_000_u64) || allowance == max_approval,
        "Expected significant allowance, got: {}", 
        allowance
    );
}

#[tokio::test]
async fn test_get_token_allowance_invalid_token() {
    let provider = get_test_provider();
    
    // Invalid token address (zero address)
    let token = Address::from_str("0x0000000000000000000000000000000000000000").unwrap();
    let owner = Address::from_str("0x0000000000000000000000000000000000000000").unwrap();
    let spender = Address::from_str("0x0000000000000000000000000000000000000000").unwrap();
    
    let result = get_token_allowance(provider, token, owner, spender).await;
    assert!(result.is_err());
    
    // Verify error message
    if let Err(e) = result {
        println!("Expected error received: {:?}", e);
    }
}
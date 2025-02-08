use ethers::types::{Address, U256};
use ethers::providers::{Provider, Http, Middleware};
use ethers::contract::abigen;
use std::sync::Arc;
use eyre::{Result, eyre};

// Generate type-safe bindings for ERC20
abigen!(
    IERC20,
    r#"[
        function allowance(address owner, address spender) external view returns (uint256)
    ]"#,
);

/// Gets the current allowance for an ERC-20 token between an owner and spender
/// 
/// # Arguments
/// * provider - The ethers provider instance
/// * token_address - The address of the ERC-20 token contract
/// * owner - The address of the token holder
/// * spender - The address authorized to spend tokens
/// 
/// # Returns
/// * Result<U256> - The current allowance amount
pub async fn get_token_allowance(
    provider: Arc<Provider<Http>>,
    token_address: Address,
    owner: Address, 
    spender: Address
) -> Result<U256> {
    // Check if the token contract exists
    if Provider::get_code(&provider, token_address, None).await?.is_empty() {
        return Err(eyre!("Token contract does not exist"));
    }

    // Create contract instance
    let contract = IERC20::new(token_address, provider.clone());
    
    // Call allowance function with error handling
    let amount = contract.allowance(owner, spender)
        .call()
        .await
        .map_err(|e| eyre!("Failed to get allowance: {:?}", e))?;
    
    Ok(amount)
}
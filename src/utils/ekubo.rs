use serde::Deserialize;
use starknet::accounts::Account;
use starknet::core::codec::{Decode, Encode};
use starknet::core::types::{BlockId, BlockTag, Call, Felt, U256};
use starknet::macros::selector;

use super::starknet::{contract_address_felt, signer_account};

#[derive(Debug, PartialEq, Eq, Deserialize, Clone, Encode, Decode)]
pub struct PoolKey {
    pub token0: Felt,
    pub token1: Felt,
    pub fee: u128,
    pub tick_spacing: u128,
    pub extension: Felt,
}

impl PoolKey {
    pub fn new(token0: Felt, token1: Felt) -> Self {
        PoolKey {
            token0,
            token1,
            fee: 170141183460469235273462165868118016,
            tick_spacing: 1000,
            extension: Felt::ZERO,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Encode, Decode)]
pub struct I129 {
    pub mag: u128,
    pub sign: bool,
}

impl I129 {
    pub fn new(mag: u128, sign: bool) -> Self {
        I129 { mag, sign }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
pub struct SwapParameters {
    pub amount: I129,
    pub is_token1: bool,
    pub sqrt_ratio_limit: U256,
    pub skip_ahead: u128,
}

impl SwapParameters {
    pub fn new(amount: I129, is_token1: bool) -> Self {
        SwapParameters {
            amount,
            is_token1,
            sqrt_ratio_limit: U256::from(18446748437148339061u128),
            skip_ahead: 0,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
pub struct SwapData {
    pub params: SwapParameters,
    pub pool_key: PoolKey,
    pub caller: Felt,
}

impl SwapData {
    pub fn new(params: SwapParameters, pool_key: PoolKey, caller: Felt) -> Self {
        SwapData {
            params,
            pool_key,
            caller,
        }
    }
}

type EkuboResponse = Result<
    starknet::core::types::InvokeTransactionResult,
    starknet::accounts::AccountError<
        starknet::accounts::single_owner::SignError<starknet::signers::local_wallet::SignError>,
    >,
>;

pub async fn ekubo_swap(token0: Felt, token1: Felt, swap_amount: u128) -> EkuboResponse {
    let mut account = signer_account();
    let contract_address = contract_address_felt();

    let pool_key = PoolKey::new(token0, token1);
    let swap_parameters = SwapParameters::new(I129::new(swap_amount, false), false);
    let swap_data = SwapData::new(swap_parameters, pool_key, account.address());

    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    let mut serialized = vec![];
    swap_data.encode(&mut serialized).unwrap();

    let transfer_call = Call {
        to: token0,
        selector: selector!("transfer"),
        calldata: vec![contract_address, Felt::from(swap_amount), Felt::ZERO],
    };

    let swap_call = Call {
        to: contract_address,
        selector: selector!("swap"),
        calldata: serialized,
    };

    account
        .execute_v3(vec![transfer_call, swap_call])
        .send()
        .await
}

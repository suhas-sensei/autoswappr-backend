use serde::{Deserialize, Serialize};
use starknet::accounts::Account;
use starknet::core::types::{BlockId, BlockTag, Call, Felt};
use starknet::macros::selector;

use super::starknet::{contract_address_felt, signer_account};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Route {
    token_from: Felt,
    token_to: Felt,
    exchange_address: Felt,
    percent: u128,
    additional_swap_params: Vec<Felt>,
}

#[derive(Debug, Clone, Copy)]
pub struct TokenFrom {
    address: Felt,
    amount: u128,
}

#[derive(Debug, Clone, Copy)]
pub struct TokenTo {
    address: Felt,
    amount: u128,
    min_amount: u128,
}

type AnvuResponse = Result<
    starknet::core::types::InvokeTransactionResult,
    starknet::accounts::AccountError<
        starknet::accounts::single_owner::SignError<starknet::signers::local_wallet::SignError>,
    >,
>;

pub async fn anvu_swap(
    token_from: TokenFrom,
    token_to: TokenTo,
    beneficiary: Felt,
    integrator_fee_amount_bps: u128,
    integrator_fee_recipient: Felt,
    routes: Vec<Route>,
) -> AnvuResponse {
    let mut account = signer_account();
    let contract_address = contract_address_felt();

    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    let routes_calldata: Vec<Felt> = routes
        .clone()
        .into_iter()
        .flat_map(|route| {
            let mut route_data = vec![
                route.token_from,
                route.token_to,
                route.exchange_address,
                Felt::from(route.percent),
                Felt::from(route.additional_swap_params.len()),
            ];
            route_data.extend(route.additional_swap_params);
            route_data
        })
        .collect();

    let approve_call = Call {
        to: token_from.address,
        selector: selector!("approve"),
        calldata: vec![contract_address, Felt::from(token_from.amount)],
    };

    let swap_call = Call {
        to: contract_address,
        selector: selector!("anvu_swap"),
        calldata: [
            token_from.address,
            token_from.amount.into(),
            token_to.address,
            token_to.amount.into(),
            token_to.min_amount.into(),
            beneficiary,
            integrator_fee_amount_bps.into(),
            integrator_fee_recipient,
            Felt::from(routes.len()),
        ]
        .into_iter()
        .chain(routes_calldata)
        .collect(),
    };

    account
        .execute_v3(vec![approve_call, swap_call])
        .send()
        .await
}

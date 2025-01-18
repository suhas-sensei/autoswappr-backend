use std::{env::var, sync::LazyLock};

use starknet::{
    accounts::{ExecutionEncoding, SingleOwnerAccount},
    core::{
        chain_id,
        types::{BlockId, BlockTag, Felt, FunctionCall},
    },
    macros::selector,
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Provider, Url,
    },
    signers::{LocalWallet, SigningKey},
};

static RPC_URL: LazyLock<String> = LazyLock::new(|| var("RPC_URL").unwrap());
static CONTRACT_ADDRESS: LazyLock<String> = LazyLock::new(|| var("CONTRACT_ADDRESS").unwrap());

fn rpc_provider() -> JsonRpcClient<HttpTransport> {
    JsonRpcClient::new(HttpTransport::new(Url::parse(&RPC_URL).unwrap()))
}

pub fn contract_address_felt() -> Felt {
    Felt::from_hex(&CONTRACT_ADDRESS).unwrap()
}

// Define an enum for supported tokens
#[derive(Debug)]
pub enum TokenType {
    ETH,
    STRK,
}

impl TokenType {
    fn get_selector(&self) -> Felt {
        match self {
            TokenType::ETH => selector!("get_eth_usd_price"),
            TokenType::STRK => selector!("get_strk_usd_price"),
        }
    }
}

pub async fn get_token_usd_price_and_decimal(token: TokenType) -> (u64, u64) {
    let provider = rpc_provider();
    let contract_address = contract_address_felt();

    let call_result = provider
        .call(
            FunctionCall {
                contract_address,
                entry_point_selector: token.get_selector(),
                calldata: vec![],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap_or_else(|_| panic!("failed to get {:?} price", token));

    let (price, decimal) = (
        call_result[0].to_le_digits()[0],
        call_result[1].to_le_digits()[0],
    );

    (price, decimal)
}

pub async fn get_eth_usd_price_and_decimal() -> (u64, u64) {
    get_token_usd_price_and_decimal(TokenType::ETH).await
}

pub async fn get_strk_usd_price_and_decimal() -> (u64, u64) {
    get_token_usd_price_and_decimal(TokenType::STRK).await
}

pub fn signer_account() -> SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet> {
    let provider = rpc_provider();
    let private_key = var("PRIVATE_KEY").unwrap();
    let public_key = var("PUBLIC_KEY").unwrap();
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex(&private_key).unwrap(),
    ));
    let address = Felt::from_hex(&public_key).unwrap();
    SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::MAINNET,
        ExecutionEncoding::New,
    )
}

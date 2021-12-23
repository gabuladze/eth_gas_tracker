use crate::types::block::Block;
use crate::types::error::Error as GasTrackerError;
use crate::types::response::{
    BlockDataResponse, BlockNumberResponse, ResponseError, TransactionReceiptResponse,
};
use crate::types::transaction::Transaction;
use serde::Deserialize;

const GETH_RPC_URL: &str = "http://localhost:8545";

type Result<T> = std::result::Result<T, GasTrackerError>;

pub async fn get_block_number() -> Result<String> {
    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    enum Response {
        BlockNumberResponse(BlockNumberResponse),
        Error(ResponseError),
    }
    let request_body = r#"{
        "jsonrpc": "2.0",
        "method": "eth_blockNumber",
        "params": [],
        "id": 0
    }"#;
    let client = reqwest::Client::new();
    let response = client
        .post(GETH_RPC_URL)
        .body(request_body)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json::<Response>()
        .await?;
    println!("{:?}", response);
    match response {
        Response::BlockNumberResponse(r) => Ok(r.result),
        Response::Error(e) => Err(GasTrackerError::ResponseError(e)),
    }
}

pub async fn get_block_by_number(
    block_number: String,
    include_transactions: bool,
) -> Result<Block> {
    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    enum Response {
        BlockDataResponse(BlockDataResponse),
        Error(ResponseError),
    }

    let request_body = format!(
        "{{
            \"jsonrpc\": \"2.0\",
            \"method\": \"eth_getBlockByNumber\",
            \"params\": [\"{}\", {}],
            \"id\": 0
        }}",
        block_number, include_transactions
    );
    let client = reqwest::Client::new();
    let response = client
        .post(GETH_RPC_URL)
        .body(request_body)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json::<Response>()
        .await?;

    match response {
        Response::BlockDataResponse(r) => Ok(r.result),
        Response::Error(e) => Err(GasTrackerError::ResponseError(e)),
    }
}

pub async fn get_transaction_receipt(hash: String) -> Result<Transaction> {
    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    enum Response {
        TransactionReceiptResponse(TransactionReceiptResponse),
        Error(ResponseError),
    }

    let request_body = format!(
        "{{
            \"jsonrpc\": \"2.0\",
            \"method\": \"eth_getTransactionReceipt\",
            \"params\": [\"{}\"],
            \"id\": 0
        }}",
        hash
    );
    let client = reqwest::Client::new();
    let response_body = client
        .post(GETH_RPC_URL)
        .body(request_body)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    let response: Response = serde_json::from_str(&response_body)?;
    match response {
        Response::TransactionReceiptResponse(r) => Ok(r.result),
        Response::Error(e) => Err(GasTrackerError::ResponseError(e)),
    }
}

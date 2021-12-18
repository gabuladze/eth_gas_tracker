use crate::types::{BlockData, BlockDataResponse, BlockNumberResponse, ErrorResponse};

use serde::Deserialize;
use std::error::Error;

const GETH_RPC_URL: &str = "http://localhost:8545";

pub async fn get_block_number() -> Result<String, ErrorResponse> {
    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    enum Response {
        BlockNumberResponse(BlockNumberResponse),
        Error(ErrorResponse),
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

    match response {
        Response::BlockNumberResponse(r) => Ok(r.result),
        Response::Error(e) => Err(e),
    }
}

pub async fn get_block_by_number(
    block_number: String,
    include_transactions: bool,
) -> Result<BlockData, impl Error> {
    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    enum Response {
        BlockDataResponse(BlockDataResponse),
        Error(ErrorResponse),
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
        Response::Error(e) => {
            println!("{:?}", e);
            Err(e)
        }
    }
}

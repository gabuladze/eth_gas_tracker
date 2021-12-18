use reqwest::{Error as ReqwestError, StatusCode};
use serde::Deserialize;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Deserialize)]
pub struct JSONRPCData {
    pub jsonrpc: String,
    pub id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockData {
    pub base_fee_per_gas: String,
    pub difficulty: String,
    pub extra_data: String,
    pub gas_limit: String,
    pub gas_used: String,
    pub hash: String,
    pub logs_bloom: String,
    pub miner: String,
    pub mix_hash: String,
    pub nonce: String,
    pub number: String,
    pub parent_hash: String,
    pub receipts_root: String,
    pub sha3_uncles: String,
    pub size: String,
    pub state_root: String,
    pub timestamp: String,
    pub total_difficulty: String,
    pub transactions: Vec<String>,
    pub transactions_root: String,
    pub uncles: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ErrorData {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    #[serde(flatten)]
    pub json_rpc: JSONRPCData,
    pub error: ErrorData,
}

#[derive(Debug, Deserialize)]
pub struct BlockDataResponse {
    #[serde(flatten)]
    pub json_rpc: JSONRPCData,
    pub result: BlockData,
}

#[derive(Debug, Deserialize)]
pub struct BlockNumberResponse {
    #[serde(flatten)]
    pub json_rpc: JSONRPCData,
    pub result: String,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "code = {} message = {}",
            self.error.code, self.error.message
        )
    }
}
impl Error for ErrorResponse {}

impl ErrorResponse {
    fn new(code: String, message: String) -> Self {
        Self {
            json_rpc: JSONRPCData {
                jsonrpc: String::from("2.0"),
                id: 0,
            },
            error: ErrorData { code, message },
        }
    }
}

impl From<ReqwestError> for ErrorResponse {
    fn from(error: ReqwestError) -> Self {
        let code = String::from((error.status().unwrap_or(StatusCode::BAD_REQUEST)).as_str());
        let message = format!("{}", error);
        ErrorResponse::new(code, message)
    }
}

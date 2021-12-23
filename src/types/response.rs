use super::block::Block;
use super::transaction::Transaction;
use serde::Deserialize;
use std::error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Deserialize)]
pub struct JSONRPCData {
    pub jsonrpc: String,
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct BlockDataResponse {
    #[serde(flatten)]
    pub json_rpc: JSONRPCData,
    pub result: Block,
}

#[derive(Debug, Deserialize)]
pub struct BlockNumberResponse {
    #[serde(flatten)]
    pub json_rpc: JSONRPCData,
    pub result: String,
}

#[derive(Debug, Deserialize)]
pub struct TransactionReceiptResponse {
    #[serde(flatten)]
    pub json_rpc: JSONRPCData,
    pub result: Transaction,
}

#[derive(Debug, Deserialize)]
pub struct ErrorData {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ResponseError {
    #[serde(flatten)]
    pub json_rpc: JSONRPCData,
    pub error: ErrorData,
}

impl Display for ResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "code = {:?} message = {:?}",
            self.error.code, self.error.message
        )
    }
}

impl error::Error for ResponseError {}

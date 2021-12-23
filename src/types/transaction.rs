use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub block_hash: String,
    pub block_number: String,
    pub contract_address: String,
    pub cumulative_gas_used: String,
    pub effective_gas_price: String,
    pub from: String,
    pub gas_used: String,
    pub logs: Vec<String>,
    pub logs_bloom: String,
    pub status: String,
    pub to: String,
    pub transaction_hash: String,
    pub transaction_index: String,
    pub r#type: String,
}

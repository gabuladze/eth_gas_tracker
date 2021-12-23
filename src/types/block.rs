use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
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

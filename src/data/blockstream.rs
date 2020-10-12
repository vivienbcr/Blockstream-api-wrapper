//! blockstream reference all data structures provided by Blocksteam API.
//! Official API documentation is available at [Blockstream Esplora API](https://github.com/Blockstream/esplora/blob/master/API.md)
//! Amounts are always represented in satoshis.
use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct BlockFormat {
    pub id: String,
    pub height: u32,
    pub version: u32,
    pub timestamp: u32,
    pub bits: u32,
    pub nonce: u32,
    pub difficulty: u32,
    pub merkle_root: String,
    pub tx_count: u32,
    pub size: u32,
    pub weight: u32,
    pub previousblockhash: String,
}
#[derive(Deserialize, Debug)]
pub struct BlockStatus {
    pub in_best_chain: bool,
    pub next_best: String,
    pub height: u32,
}
#[derive(Deserialize, Debug)]
pub struct VoutFormat {
    pub scriptpubkey: String,
    pub scriptpubkey_asm: String,
    pub scriptpubkey_type: String,
    pub scriptpubkey_address: Option<String>,
    pub value: u32,
}
#[derive(Deserialize, Debug)]
pub struct VinFormat {
    pub txid: String,
    pub vout: u32,
    pub is_coinbase: bool,
    pub scriptsig: String,
    pub scriptsig_asm: String,
    //FIXME
    // inner_redeemscript_asm: String,
    // inner_witnessscript_asm: String,
    pub sequence: u32,
    //FIXME
    // witness[]
    // #[serde(skip_deserializing)]
    pub prevout: Option<VoutFormat>,
}
#[derive(Deserialize, Debug)]
pub struct TxStatusFormat {
    pub confirmed: bool,
    pub block_height: Option<u32>,
    pub block_hash: Option<String>,
    pub block_time: u32,
}
#[derive(Deserialize, Debug)]
pub struct UtxoFormat {
    pub txid: String,
    pub vout: u16,
    pub status: TxStatusFormat,
    pub value: u32,
}
#[derive(Deserialize, Debug)]
pub struct TransactionFormat {
    pub txid: String,
    pub version: u32,
    pub locktime: u32,
    pub size: u32,
    pub weight: u32,
    pub fee: u32,
    pub vin: Vec<VinFormat>,
    pub vout: Vec<VoutFormat>,
    pub status: TxStatusFormat,
}
#[derive(Deserialize, Debug)]
pub struct MerkleProofFormat {
    pub block_height: u32,
    pub  merkle: Vec<String>,
    pub pos: u32,
}
#[derive(Deserialize, Debug)]
pub struct OutspentFormat {
    pub spent: bool,
    pub txid: Option<String>,
    pub vin: Option<u32>,
    pub status: Option<TxStatusFormat>,
}
#[derive(Deserialize, Debug)]
pub struct AddressInfoFormat {
    pub address: Option<String>,
    pub chain_stats: ChainMempoolStats,
    pub mempool_stats: ChainMempoolStats,
    pub scripthash: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct ChainMempoolStats {
    pub funded_txo_count: i32,
    pub funded_txo_sum: i32,
    pub spent_txo_count: i32,
    pub spent_txo_sum: i32,
    pub tx_count: i32,
}
#[derive(Deserialize, Debug)]
pub struct MemPoolFormat {
    pub count: u32,
    pub vsize: u32,
    pub total_fee: u32,
    pub fee_histogram: Vec<Vec<f32>>,
}
#[derive(Deserialize, Debug)]
pub struct MempoolTxFormat {
    pub txid: String,
    pub fee: u32,
    pub vsize: u32,
    pub value: u64,
}
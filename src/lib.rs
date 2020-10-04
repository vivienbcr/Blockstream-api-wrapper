use bitcoin::hash_types::BlockHash;
use bitcoin::{Block, BlockHeader};
extern crate reqwest;
extern crate serde;
use serde::Deserialize;
use std::collections::HashMap;

pub struct Client {
    pub url: String,
}
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
pub struct VoutFormat {
    scriptpubkey: String,
    scriptpubkey_asm: String,
    scriptpubkey_type: String,
    scriptpubkey_address: Option<String>,
    value: u32,
}

#[derive(Deserialize, Debug)]
pub struct VinFormat {
    txid: String,
    vout: u32,
    is_coinbase: bool,
    scriptsig: String,
    scriptsig_asm: String,
    //FIXME
    // inner_redeemscript_asm: String,
    // inner_witnessscript_asm: String,
    sequence: u32,
    //FIXME
    // witness[]
    // #[serde(skip_deserializing)]
    prevout: Option<VoutFormat>,
}
#[derive(Deserialize, Debug)]
pub struct TxStatusFormat {
    confirmed: bool,
    block_height: u32,
    block_hash: String,
    block_time: u32,
}
#[derive(Deserialize, Debug)]
pub struct TransactionFormat {
    txid: String,
    version: u32,
    locktime: u32,
    size: u32,
    weight: u32,
    fee: u32,
    vin: Vec<VinFormat>,
    vout: Vec<VoutFormat>,
    status: TxStatusFormat,
}

impl Client {
    pub fn new(url: String) -> Self {
        Client { url }
    }
    // GET /block/:hash

    // Returns information about a block.
    // Available fields: id, height, version, timestamp, bits, nonce, merkle_root, tx_count, size, weight and previousblockhash. Elements-based chains have an additional proof field. See block format for more details.
    // The response from this endpoint can be cached indefinitely.
    pub fn get_block(&self, hash: String) -> Result<BlockFormat, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}{}", self.url, "/block/", hash);
        let resp: BlockFormat = reqwest::blocking::get(&request_url)?.json()?;
        Ok(resp)
    }
    // GET /block/:hash/status

    // Returns the block status.
    // Available fields: in_best_chain (boolean, false for orphaned blocks), next_best (the hash of the next block, only available for blocks in the best chain).
    pub fn get_block_status(
        &self,
        hash: String,
    ) -> Result<BlockFormat, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}{}{}", self.url, "/block/", hash, "/status");
        let resp: BlockFormat = reqwest::blocking::get(&request_url)?.json()?;
        Ok(resp)
    }
    // GET /block/:hash/txs[/:start_index]

    // Returns a list of transactions in the block (up to 25 transactions beginning at start_index).
    // Transactions returned here do not have the status field, since all the transactions share the same block and confirmation status.
    // The response from this endpoint can be cached indefinitely.
    pub fn get_block_txs(
        &self,
        hash: String,
        start_index: Option<u32>,
    ) -> Result<Vec<TransactionFormat>, Box<dyn std::error::Error>> {
        let i = start_index.unwrap_or(0);
        let mut request_url = format!("{}{}{}{}", self.url, "/block/", hash, "/txs");
        match i {
            i if i != 0 => request_url.push_str(&format!("/{}", i.to_string())),
            _ => (),
        }
        let resp: Vec<TransactionFormat> = reqwest::blocking::get(&request_url)?.json()?;
        Ok(resp)
    }
    // GET /block/:hash/txids

    // Returns a list of all txids in the block.

    // The response from this endpoint can be cached indefinitely.
    // GET /block/:hash/txid/:index

    // Returns the transaction at index :index within the specified block.

    // The response from this endpoint can be cached indefinitely.
    // GET /block/:hash/raw

    // Returns the raw block representation in binary.

    // The response from this endpoint can be cached indefinitely.
    // GET /block-height/:height

    // Returns the hash of the block currently at height.
    // GET /blocks[/:start_height]

    // Returns the 10 newest blocks starting at the tip or at start_height if specified.
    // GET /blocks/tip/height

    // Returns the height of the last block.
    // GET /blocks/tip/hash

    // Returns the hash of the last block.
}

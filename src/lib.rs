// use bitcoin::hash_types::BlockHash;
// use bitcoin::{Block, BlockHeader};
extern crate reqwest;
pub use serde;
use serde::Deserialize;
#[derive(Debug)]
pub struct ApiClient {
    pub url: String,
    pub reqwest: reqwest::blocking::Client,
}
#[derive(Debug)]
pub struct ClientOptions {
    pub headers: Option<HeadersOptions>,
}
#[derive(Debug)]
pub struct HeadersOptions {
    pub authorization: Option<String>,
}
impl ApiClient {
    pub fn new(
        url: &str,
        options: Option<ClientOptions>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut client_builder = reqwest::blocking::ClientBuilder::new();
        // Find options
        match options {
            // Build headers
            Some(ClientOptions { headers, .. }) => {
                let mut headers_map = reqwest::header::HeaderMap::new();
                match headers {
                    // header::AUTHORIZATION
                    Some(HeadersOptions {
                        authorization: Some(authorization),
                    }) => {
                        headers_map.insert(
                            reqwest::header::AUTHORIZATION,
                            reqwest::header::HeaderValue::from_str(&authorization).unwrap(),
                        );
                    }
                    _ => (),
                }
                client_builder = client_builder.default_headers(headers_map);
            }
            None => (),
        }
        let build = client_builder
            .build()
            .unwrap_or(reqwest::blocking::Client::new());

        Ok(ApiClient {
            url: url.to_string(),
            reqwest: build,
        })
    }
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
pub struct BlockStatus {
    pub in_best_chain: bool,
    pub next_best: String,
    pub height: u32,
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
    confirmed: bool,
    block_height: Option<u32>,
    block_hash: Option<String>,
    block_time: u32,
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
    block_height: u32,
    merkle: Vec<String>,
    pos: u32,
}
#[derive(Deserialize, Debug)]
pub struct OutspentFormat {
    pub spent: bool,
    pub txid: Option<String>,
    pub vin: Option<u32>,
    pub status: Option<TxStatusFormat>,
}
#[derive(Deserialize, Debug)]
pub struct AddressInfoFormat{
    pub address:Option<String>,
    pub chain_stats : ChainMempoolStats,
    pub mempool_stats: ChainMempoolStats,
    pub scripthash : Option<String>
    
}
#[derive(Deserialize, Debug)]
pub struct ChainMempoolStats {
    pub funded_txo_count: i32,
    pub funded_txo_sum: i32,
    pub spent_txo_count: i32,
    pub spent_txo_sum: i32,
    pub tx_count: i32
}

impl ApiClient {
    // GET /block/:hash

    // Returns information about a block.
    // Available fields: id, height, version, timestamp, bits, nonce, merkle_root, tx_count, size, weight and previousblockhash. Elements-based chains have an additional proof field. See block format for more details.
    // The response from this endpoint can be cached indefinitely.
    pub fn get_block(&self, hash: &str) -> Result<BlockFormat, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}{}", self.url, "/block/", hash);
        let resp: BlockFormat = self.reqwest.get(&request_url).send()?.json()?;
        Ok(resp)
    }
    // GET /block/:hash/status

    // Returns the block status.
    // Available fields: in_best_chain (boolean, false for orphaned blocks), next_best (the hash of the next block, only available for blocks in the best chain).
    pub fn get_block_status(&self, hash: &str) -> Result<BlockStatus, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}{}{}", self.url, "/block/", hash, "/status");
        let resp: BlockStatus = self.reqwest.get(&request_url).send()?.json()?;
        Ok(resp)
    }
    // GET /block/:hash/txs[/:start_index]

    // Returns a list of transactions in the block (up to 25 transactions beginning at start_index).
    // Transactions returned here do not have the status field, since all the transactions share the same block and confirmation status.
    // The response from this endpoint can be cached indefinitely.
    pub fn get_block_txs(
        &self,
        hash: &str,
        start_index: Option<i32>,
    ) -> Result<Vec<TransactionFormat>, Box<dyn std::error::Error>> {
        let i = start_index.unwrap_or(0);
        let mut request_url = format!("{}{}{}{}", self.url, "/block/", hash, "/txs");
        match i {
            i if i != 0 => request_url.push_str(&format!("/{}", i.to_string())),
            _ => (),
        }
        let resp: Vec<TransactionFormat> = self.reqwest.get(&request_url).send()?.json()?;
        Ok(resp)
    }
    // GET /block/:hash/txids

    // Returns a list of all txids in the block.
    // The response from this endpoint can be cached indefinitely.
    pub fn get_block_txids(&self, hash: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}{}{}", self.url, "/block/", hash, "/txids");
        let resp: Vec<String> = self.reqwest.get(&request_url).send()?.json()?;
        Ok(resp)
    }
    // GET /block/:hash/txid/:index

    // Returns the transaction at index :index within the specified block.
    // The response from this endpoint can be cached indefinitely.
    pub fn get_block_txid_at_index(
        &self,
        hash: &str,
        index: i32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let request_url = format!(
            "{}{}{}{}{}",
            self.url,
            "/block/",
            hash,
            "/txid/",
            index.to_string()
        );
        let resp: String = self.reqwest.get(&request_url).send()?.text()?;
        Ok(resp.clone())
    }
    // GET /block/:hash/raw

    // Returns the raw block representation in binary.
    // The response from this endpoint can be cached indefinitely.
    pub fn get_block_raw_format(&self, hash: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}{}{}", self.url, "/block/", hash, "/raw");
        let resp = self.reqwest.get(&request_url).send()?.bytes()?.to_vec();
        Ok(resp)
    }
    // GET /block-height/:height

    // Returns the hash of the block currently at height.
    pub fn get_block_height(&self, height: i32) -> Result<String, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}{}", self.url, "/block-height/", height);
        let resp = self.reqwest.get(&request_url).send()?.text()?;
        Ok(resp)
    }
    // GET /blocks[/:start_height]

    // Returns the 10 newest blocks starting at the tip or at start_height if specified.
    pub fn get_blocks(
        &self,
        start_height: i32,
    ) -> Result<Vec<BlockFormat>, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}{}", self.url, "/blocks/", start_height);
        let resp = self.reqwest.get(&request_url).send()?.json()?;
        Ok(resp)
    }

    // GET /blocks/tip/height

    // Returns the height of the last block.
    pub fn get_blocks_tip_height(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}", self.url, "/blocks/tip/height");
        let resp = self.reqwest.get(&request_url).send()?.text()?.parse()?;
        Ok(resp)
    }
    // GET /blocks/tip/hash

    // Returns the hash of the last block.
    pub fn get_blocks_tip_hash(&self) -> Result<String, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}", self.url, "/blocks/tip/hash");
        let resp = self.reqwest.get(&request_url).send()?.text()?;
        Ok(resp)
    }
    // GET /tx/:txid

    // Returns information about the transaction.
    // Available fields: txid, version, locktime, size, weight, fee, vin, vout and status (see transaction format for details).
    pub fn get_tx(&self, txid: &str) -> Result<TransactionFormat, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}{}", self.url, "/tx/", txid);
        let resp = self.reqwest.get(&request_url).send()?.json()?;
        Ok(resp)
    }
    //     GET /tx/:txid/status

    // Returns the transaction confirmation status.
    // Available fields: confirmed (boolean), block_height (optional) and block_hash (optional).
    pub fn get_tx_status(&self, txid: &str) -> Result<TxStatusFormat, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}{}{}", self.url, "/tx/", txid, "/status");
        let resp = self.reqwest.get(&request_url).send()?.json()?;
        Ok(resp)
    }
    // GET /tx/:txid/hex
    // GET /tx/:txid/raw

    // Returns the raw transaction in hex or as binary data.
    pub fn get_tx_raw(&self, txid: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}{}{}", self.url, "/tx/", txid, "/raw");
        let resp = self.reqwest.get(&request_url).send()?.bytes()?.to_vec();
        Ok(resp)
    }
    pub fn get_tx_hex(&self, txid: &str) -> Result<String, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}{}{}", self.url, "/tx/", txid, "/raw");
        let resp = self.reqwest.get(&request_url).send()?.text()?;
        Ok(resp)
    }

    // GET /tx/:txid/merkleblock-proof

    // Returns a merkle inclusion proof for the transaction using bitcoind's merkleblock format.

    // Note: This endpoint is not currently available for Liquid/Elements-based chains.
    pub fn get_tx_merkleblock_proof(
        &self,
        txid: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}{}{}", self.url, "/tx/", txid, "/merkleblock-proof");
        let resp = self.reqwest.get(&request_url).send()?.text()?;
        Ok(resp)
    }
    // GET /tx/:txid/merkle-proof

    // Returns a merkle inclusion proof for the transaction using Electrum's blockchain.transaction.get_merkle format.
    pub fn get_tx_merkle_proof(
        &self,
        txid: &str,
    ) -> Result<MerkleProofFormat, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}{}{}", self.url, "/tx/", txid, "/merkle-proof");
        let resp = self.reqwest.get(&request_url).send()?.json()?;
        Ok(resp)
    }
    // GET /tx/:txid/outspend/:vout

    // Returns the spending status of a transaction output.
    // Available fields: spent (boolean), txid (optional), vin (optional) and status (optional, the status of the spending tx).
    pub fn get_tx_outspend(
        &self,
        txid: &str,
        vout: Option<i32>,
    ) -> Result<OutspentFormat, Box<dyn std::error::Error>> {
        let request_url = format!(
            "{}{}{}{}{}",
            self.url,
            "/tx/",
            txid,
            "/outspend/",
            vout.unwrap().to_string()
        );
        let resp = self.reqwest.get(&request_url).send()?.json()?;
        Ok(resp)
    }
    // GET /tx/:txid/outspends

    // Returns the spending status of all transaction outputs.
    pub fn get_tx_outspends(
        &self,
        txid: &str,
    ) -> Result<Vec<OutspentFormat>, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}{}{}", self.url, "/tx/", txid, "/outspends");
        let resp = self.reqwest.get(&request_url).send()?.json()?;
        Ok(resp)
    }

    // POST /tx

    // Broadcast a raw transaction to the network.
    // The transaction should be provided as hex in the request body. The txid will be returned on success.
    pub fn post_tx(&self, hex_transaction: &str) -> Result<String, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}", self.url, "/tx");
        let resp = self
            .reqwest
            .post(&request_url)
            .body(hex_transaction.to_string())
            .send()?
            .text()?;
        Ok(resp)
    }

    // GET /address/:address
    // GET /scripthash/:hash

    // Get information about an address/scripthash.
    // Available fields: address/scripthash, chain_stats and mempool_stats.
    // {chain,mempool}_stats each contain an object with tx_count, funded_txo_count, funded_txo_sum, spent_txo_count and spent_txo_sum.
    // Elements-based chains don't have the {funded,spent}_txo_sum fields.
    pub fn get_address(
        &self,
        address: &str,
    ) -> Result<AddressInfoFormat, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}{}", self.url, "/address/", address);
        let resp = self.reqwest.get(&request_url).send()?.json()?;
        Ok(resp)
    }
    pub fn get_script_hash(
        &self,
        scripthash: &str,
    ) -> Result<AddressInfoFormat, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}{}", self.url, "/scripthash/", scripthash);
        let resp = self.reqwest.get(&request_url).send()?.json()?;
        Ok(resp)
    }

    // GET /address/:address/txs
    // GET /scripthash/:hash/txs

    // Get transaction history for the specified address/scripthash, sorted with newest first.
    // Returns up to 50 mempool transactions plus the first 25 confirmed transactions. You can request more confirmed transactions using :last_seen_txid(see below).
    pub fn get_address_txs(
        &self,
        address: &str,
    ) -> Result<Vec<TransactionFormat>, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}{}{}", self.url, "/address/", address,"/txs");
        let resp = self.reqwest.get(&request_url).send()?.json()?;
        Ok(resp)
    }
    pub fn get_script_hash_txs(
        &self,
        scripthash: &str,
    ) -> Result<Vec<TransactionFormat>, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}{}{}", self.url, "/scripthash/", scripthash,"/txs");
        let resp = self.reqwest.get(&request_url).send()?.json()?;
        Ok(resp)
    }
    
    // GET /address/:address/txs/chain[/:last_seen_txid]
    // GET /scripthash/:hash/txs/chain[/:last_seen_txid]

    // Get confirmed transaction history for the specified address/scripthash, sorted with newest first.
    // Returns 25 transactions per page. More can be requested by specifying the last txid seen by the previous query.
    pub fn get_address_txs_chain(
        &self,
        address: &str,
        txid : Option<&str>
    ) -> Result<Vec<TransactionFormat>, Box<dyn std::error::Error>> {
        let mut request_url = format!("{}{}{}{}", self.url, "/address/", address,"/txs/chain");
        match txid {
            Some(txid)=> request_url.push_str(&format!("/{}",txid)),
            _ => ()
        }
        let resp = self.reqwest.get(&request_url).send()?.json()?;
        Ok(resp)
    }
    pub fn get_script_hash_txs_chain(
        &self,
        scripthash: &str,
        txid : Option<&str>
    ) -> Result<Vec<TransactionFormat>, Box<dyn std::error::Error>> {
        let mut request_url = format!("{}{}{}{}", self.url, "/scripthash/", scripthash,"/txs/chain");
        match txid {
            Some(txid)=> request_url.push_str(&format!("/{}",txid)),
            _ => ()
        }
        let resp = self.reqwest.get(&request_url).send()?.json()?;
        Ok(resp)
    }
    // GET /address/:address/txs/mempool
    // GET /scripthash/:hash/txs/mempool

    // Get unconfirmed transaction history for the specified address/scripthash.

    // Returns up to 50 transactions (no paging).
    pub fn get_address_txs_mempool(
        &self,
        address: &str,
    ) -> Result<Vec<TransactionFormat>, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}{}{}", self.url, "/address/", address,"/txs/mempool");
        let resp = self.reqwest.get(&request_url).send()?.json()?;
        Ok(resp)
    }
    pub fn get_script_hash_txs_mempool(
        &self,
        scripthash: &str,
    ) -> Result<Vec<TransactionFormat>, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}{}{}", self.url, "/scripthash/", scripthash,"/txs/mempool");
        let resp = self.reqwest.get(&request_url).send()?.json()?;
        Ok(resp)
    }
    // GET /address/:address/utxo
    // GET /scripthash/:hash/utxo

    // Get the list of unspent transaction outputs associated with the address/scripthash.

    // Available fields: txid, vout, value and status (with the status of the funding tx).

    // Elements-based chains have a valuecommitment field that may appear in place of value, plus the following additional fields: asset/assetcommitment, nonce/noncecommitment, surjection_proof and range_proof.
    // GET /address-prefix/:prefix

    // Search for addresses beginning with :prefix.

    // Returns a JSON array with up to 10 results.
}

#[cfg(test)]
mod test {
    use super::*;
    static ENDPOINT_URL: &str = "https://blockstream.info/testnet/api/";
    fn default_client() -> ApiClient {
        return ApiClient::new(ENDPOINT_URL, None).unwrap();
    }
    #[test]
    fn get_block() {
        let client = default_client();
        let response =
            client.get_block("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7");
        assert_eq!(
            response.unwrap().previousblockhash,
            "000000000000002b6f0830e1b92c6e59f18d147c0370a3425c91be21e0b1ff85"
        );
    }
    #[test]
    fn get_block_status() {
        let client = default_client();
        let response = client
            .get_block_status("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7");
        assert_eq!(response.unwrap().in_best_chain, true);
    }
    #[test]
    fn get_block_txs_with_and_without_index() {
        let client = default_client();
        let first_txs_index = client.get_block_txs(
            "000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7",
            None,
        );
        let second_txs_index = client.get_block_txs(
            "000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7",
            Some(25),
        );
        assert_eq!(
            first_txs_index
                .unwrap()
                .iter()
                .position(|tx| tx.txid
                    == "bdbaa506c8903918b407fca86bd3498cd7794000b22cddeb1c87c2d9eb8fab62")
                .unwrap(),
            0
        );
        assert_eq!(
            second_txs_index
                .unwrap()
                .iter()
                .position(|tx| tx.txid
                    == "a9e7e29b703e667311fb2453e694f17d178822cc2fc4fe4db8cfb8df81898845")
                .unwrap(),
            0
        );
    }
    #[test]
    fn get_block_txids() {
        let client = default_client();
        let txids_list = client
            .get_block_txids("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7");
        assert_ne!(txids_list.unwrap().len(), 0);
    }
    #[test]
    fn get_block_txid_at_index() {
        let client = default_client();
        let txid = client.get_block_txid_at_index(
            "000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7",
            2,
        );
        assert_eq!(
            txid.unwrap(),
            "4799bfae158a166c76d8ddbb45f3f4da9c5fe06d6b9a3a61867651d51a099df0"
        );
    }

    #[test]
    fn get_block_raw_format() {
        let client = default_client();
        let response = client
            .get_block_raw_format(
                "000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7",
            )
            .unwrap();
        assert_ne!(response.iter().count(), 0);
    }
    #[test]
    fn get_block_height() {
        let client = default_client();
        let block_hash = client.get_block_height(424242).unwrap();
        let block = client.get_block(&block_hash);
        assert_eq!(block.unwrap().height, 424242);
    }
    #[test]
    // Return 10 blocks from start_height
    fn get_blocks() {
        let client = default_client();
        let blocks = client.get_blocks(1234).unwrap();
        assert_eq!(blocks.iter().count(), 10);
    }
    #[test]
    // Function need return last block height
    fn get_blocks_tip_height() {
        let client = default_client();
        let height = client.get_blocks_tip_height().unwrap();

        assert_eq!(height > 1838109, true);
    }
    #[test]
    // Verify function return hash
    fn get_blocks_tip_hash() {
        let client = default_client();
        let hash = client.get_blocks_tip_hash().unwrap();

        assert_eq!(hash.len(), 64);
    }
    #[test]
    // Check tx version
    fn get_tx() {
        let client = default_client();
        let tx = client
            .get_tx("c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24")
            .unwrap();
        assert_eq!(tx.version, 1);
    }
    #[test]
    // Tx status is confirmed
    fn get_tx_status() {
        let client = default_client();
        let tx_status = client
            .get_tx_status("c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24")
            .unwrap();
        assert_eq!(tx_status.confirmed, true);
    }
    #[test]
    // Tx raw
    fn get_tx_raw() {
        let client = default_client();
        let tx_raw = client
            .get_tx_raw("c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24")
            .unwrap();
        assert_ne!(tx_raw.iter().count(), 0);
    }
    #[test]
    // Tx hex
    fn get_tx_hex() {
        let client = default_client();
        let tx_hex = client
            .get_tx_hex("c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24")
            .unwrap();
        assert_eq!(tx_hex.len() > 1, true);
    }
    #[test]
    fn get_tx_merkleblock_proof() {
        let client = default_client();
        let tx_hex = client
            .get_tx_merkleblock_proof(
                "c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24",
            )
            .unwrap();
        assert_eq!(tx_hex.len() > 1, true);
    }
    #[test]
    fn get_tx_merkle_proof() {
        let client = default_client();
        let merkle_proof = client
            .get_tx_merkle_proof("6814c0b3915a8de663851b9887e0cce7d0d6c6b3f7c28b97ba8a643b72e1b7c3")
            .unwrap();
        assert_eq!(merkle_proof.merkle.iter().count(), 6);
    }
    #[test]
    fn get_tx_outspend() {
        let client = default_client();
        let outspend = client
            .get_tx_outspend(
                "fac9af7f793330af3cc0bce4790d98499c59d47a125af7260edd61d647003316",
                Some(1),
            )
            .unwrap();
        assert_eq!(outspend.spent, true);
    }
    #[test]
    fn get_tx_outspends() {
        let client = default_client();
        let outpends = client
            .get_tx_outspends("fac9af7f793330af3cc0bce4790d98499c59d47a125af7260edd61d647003316")
            .unwrap();
        assert_eq!(outpends.iter().count(), 3);
    }
    #[test]
    fn post_tx() {
        let client = default_client();
        let resp = client.post_tx("010000000001010000000000000000000000000000000000000000000000000000000000000000ffffffff2003220d1c04d6d37c5f0877fffb9a4b3500000d2f6e6f64655374726174756d2f00000000030000000000000000266a24aa21a9ed61dc942663feda48033d1026d2fa8acf0f098870202c541bffa7771e8dc51e159b0e2801000000001976a914dfdf4d53296fac595dc33d8ac7216ba516b8dcc588ac8ffd0200000000001976a914bfcc245931cbad63d09f62df43bcab989991014e88ac0120000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
        assert_eq!(resp.contains("Transaction already in block chain"), true)
    }
    #[test]
    fn get_address() {
        let client = default_client();
        let address = client.get_address("2MvJVm11phGoxEekPB8Hw2Tksb57eVRGHC5").unwrap();
        assert_eq!(address.address.unwrap() == "2MvJVm11phGoxEekPB8Hw2Tksb57eVRGHC5", true)
    }
    #[test]
    fn get_script_hash() {
        let client = default_client();
        let address = client.get_script_hash("c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c").unwrap();
        assert_eq!(address.scripthash.unwrap() == "c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c", true)
    }
    #[test]
    fn get_address_txs() {
        let client = default_client();
        let tx_list = client.get_address_txs("2MvJVm11phGoxEekPB8Hw2Tksb57eVRGHC5").unwrap();
        assert_eq!(tx_list.iter().count() > 0, true)
    }
    #[test]
    fn get_script_hash_txs() {
        let client = default_client();
        let tx_list = client.get_script_hash_txs("c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c").unwrap();
        assert_eq!(tx_list.iter().count() > 0, false)
    }
    #[test]
    fn get_address_txs_chain() {
        let client = default_client();
        let tx_list = client.get_address_txs_chain("n1vgV8XmoggmRXzW3hGD8ZNTAgvhcwT4Gk",Some("d0075b62f8b3e464472b8edecf56083ca3e9e8424f5f332ed2f9045d7fcccddc")).unwrap();
        let tx_list_from_index = client.get_address_txs_chain("n1vgV8XmoggmRXzW3hGD8ZNTAgvhcwT4Gk",Some(&tx_list[1].txid)).unwrap();
        assert_eq!(tx_list[2].txid  == tx_list_from_index[0].txid, true)
    }
    #[test]
    fn get_script_hash_txs_chain() {
        let client = default_client();
        let tx_list = client.get_script_hash_txs_chain("c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c",None).unwrap();
        assert_eq!(tx_list.iter().count() > 0, false)
    }
    #[test]
    fn get_address_txs_mempool() {
        let client = default_client();
        let tx_list = client.get_address_txs_mempool("2MvJVm11phGoxEekPB8Hw2Tksb57eVRGHC5").unwrap();
        assert_eq!(tx_list.iter().count() == 0, true)
    }
    #[test]
    fn get_script_hash_txs_mempool() {
        let client = default_client();
        let tx_list = client.get_script_hash_txs_mempool("c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c").unwrap();
        assert_eq!(tx_list.iter().count() == 0, true)
    }

}

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
    block_height: u32,
    block_hash: String,
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
        let response = client.get_block_raw_format(
            "000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7",
        );
        assert_ne!(response.unwrap().iter().count(), 0);
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
}

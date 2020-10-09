use std::collections::HashMap;

use super::client::ApiClient;
use crate::data::blockstream::{
    AddressInfoFormat, BlockFormat, BlockStatus, MemPoolFormat, MempoolTxFormat, MerkleProofFormat,
    OutspentFormat, TransactionFormat, TxStatusFormat, UtxoFormat,
};

impl ApiClient {
    // GET /block/:hash

    // Returns information about a block.
    // Available fields: id, height, version, timestamp, bits, nonce, merkle_root, tx_count, size, weight and previousblockhash. Elements-based chains have an additional proof field. See block format for more details.
    // The response from this endpoint can be cached indefinitely.

    pub async fn get_block(&self, hash: &str) -> Result<BlockFormat, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}{}", self.url, "/block/", hash);
        let resp: BlockFormat = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }
        // GET /block/:hash/status

        // Returns the block status.
        // Available fields: in_best_chain (boolean, false for orphaned blocks), next_best (the hash of the next block, only available for blocks in the best chain).
        #[allow(dead_code)]
        pub async fn get_block_status(&self, hash: &str) -> Result<BlockStatus, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}{}{}", self.url, "/block/", hash, "/status");
            let resp: BlockStatus = self.reqwest.get(&request_url).send().await?.json().await?;
            Ok(resp)
        }
        // GET /block/:hash/txs[/:start_index]

        // Returns a list of transactions in the block (up to 25 transactions beginning at start_index).
        // Transactions returned here do not have the status field, since all the transactions share the same block and confirmation status.
        // The response from this endpoint can be cached indefinitely.
        #[allow(dead_code)]
        pub async fn get_block_txs(
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
            let resp: Vec<TransactionFormat> = self.reqwest.get(&request_url).send().await?.json().await?;
            Ok(resp)
        }
        // GET /block/:hash/txids

        // Returns a list of all txids in the block.
        // The response from this endpoint can be cached indefinitely.
        #[allow(dead_code)]
        pub async fn get_block_txids(&self, hash: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}{}{}", self.url, "/block/", hash, "/txids");
            let resp: Vec<String> = self.reqwest.get(&request_url).send().await?.json().await?;
            Ok(resp)
        }
        // GET /block/:hash/txid/:index

        // Returns the transaction at index :index within the specified block.
        // The response from this endpoint can be cached indefinitely.
        #[allow(dead_code)]
        pub async fn get_block_txid_at_index(
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
            let resp: String = self.reqwest.get(&request_url).send().await?.text().await?;
            Ok(resp.clone())
        }
        // GET /block/:hash/raw

        // Returns the raw block representation in binary.
        // The response from this endpoint can be cached indefinitely.
        #[allow(dead_code)]
        pub async fn get_block_raw_format(&self, hash: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}{}{}", self.url, "/block/", hash, "/raw");
            let resp = self.reqwest.get(&request_url).send().await?.bytes().await?.to_vec();
            Ok(resp)
        }
        // GET /block-height/:height

        // Returns the hash of the block currently at height.
        #[allow(dead_code)]
        pub async fn get_block_height(&self, height: i32) -> Result<String, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}{}", self.url, "/block-height/", height);
            let resp = self.reqwest.get(&request_url).send().await?.text().await?;
            Ok(resp)
        }
        // GET /blocks[/:start_height]

        // Returns the 10 newest blocks starting at the tip or at start_height if specified.
        #[allow(dead_code)]
        pub async fn get_blocks(
            &self,
            start_height: i32,
        ) -> Result<Vec<BlockFormat>, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}{}", self.url, "/blocks/", start_height);
            let resp = self.reqwest.get(&request_url).send().await?.json().await?;
            Ok(resp)
        }

        // GET /blocks/tip/height

        // Returns the height of the last block.
        #[allow(dead_code)]
        pub async fn get_blocks_tip_height(&self) -> Result<i32, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}", self.url, "/blocks/tip/height");
            let resp = self.reqwest.get(&request_url).send().await?.text().await?.parse()?;
            Ok(resp)
        }
        // GET /blocks/tip/hash

        // Returns the hash of the last block.
        #[allow(dead_code)]
        pub async fn get_blocks_tip_hash(&self) -> Result<String, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}", self.url, "/blocks/tip/hash");
            let resp = self.reqwest.get(&request_url).send().await?.text().await?;
            Ok(resp)
        }
        // GET /tx/:txid

        // Returns information about the transaction.
        // Available fields: txid, version, locktime, size, weight, fee, vin, vout and status (see transaction format for details).
        #[allow(dead_code)]
        pub async fn get_tx(&self, txid: &str) -> Result<TransactionFormat, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}{}", self.url, "/tx/", txid);
            let resp = self.reqwest.get(&request_url).send().await?.json().await?;
            Ok(resp)
        }
        //     GET /tx/:txid/status

        // Returns the transaction confirmation status.
        // Available fields: confirmed (boolean), block_height (optional) and block_hash (optional).
        #[allow(dead_code)]
        pub async fn get_tx_status(&self, txid: &str) -> Result<TxStatusFormat, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}{}{}", self.url, "/tx/", txid, "/status");
            let resp = self.reqwest.get(&request_url).send().await?.json().await?;
            Ok(resp)
        }
        // GET /tx/:txid/hex
        // GET /tx/:txid/raw

        // Returns the raw transaction in hex or as binary data.
        #[allow(dead_code)]
        pub async fn get_tx_raw(&self, txid: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}{}{}", self.url, "/tx/", txid, "/raw");
            let resp = self.reqwest.get(&request_url).send().await?.bytes().await?.to_vec();
            Ok(resp)
        }
        #[allow(dead_code)]
        pub async fn get_tx_hex(&self, txid: &str) -> Result<String, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}{}{}", self.url, "/tx/", txid, "/raw");
            let resp = self.reqwest.get(&request_url).send().await?.text().await?;
            Ok(resp)
        }

        // GET /tx/:txid/merkleblock-proof

        // Returns a merkle inclusion proof for the transaction using bitcoind's merkleblock format.

        // Note: This endpoint is not currently available for Liquid/Elements-based chains.
        #[allow(dead_code)]
        pub async fn get_tx_merkleblock_proof(
            &self,
            txid: &str,
        ) -> Result<String, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}{}{}", self.url, "/tx/", txid, "/merkleblock-proof");
            let resp = self.reqwest.get(&request_url).send().await?.text().await?;
            Ok(resp)
        }
        // GET /tx/:txid/merkle-proof

        // Returns a merkle inclusion proof for the transaction using Electrum's blockchain.transaction.get_merkle format.
        #[allow(dead_code)]
        pub async fn get_tx_merkle_proof(
            &self,
            txid: &str,
        ) -> Result<MerkleProofFormat, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}{}{}", self.url, "/tx/", txid, "/merkle-proof");
            let resp = self.reqwest.get(&request_url).send().await?.json().await?;
            Ok(resp)
        }
        // GET /tx/:txid/outspend/:vout

        // Returns the spending status of a transaction output.
        // Available fields: spent (boolean), txid (optional), vin (optional) and status (optional, the status of the spending tx).
        #[allow(dead_code)]
        pub async fn get_tx_outspend(
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
            let resp = self.reqwest.get(&request_url).send().await?.json().await?;
            Ok(resp)
        }
        // GET /tx/:txid/outspends

        // Returns the spending status of all transaction outputs.
        #[allow(dead_code)]
        pub async fn get_tx_outspends(
            &self,
            txid: &str,
        ) -> Result<Vec<OutspentFormat>, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}{}{}", self.url, "/tx/", txid, "/outspends");
            let resp = self.reqwest.get(&request_url).send().await?.json().await?;
            Ok(resp)
        }

        // POST /tx

        // Broadcast a raw transaction to the network.
        // The transaction should be provided as hex in the request body. The txid will be returned on success.
        #[allow(dead_code)]
        pub async fn post_tx(&self, hex_transaction: &str) -> Result<String, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}", self.url, "/tx");
            let resp = self
                .reqwest
                .post(&request_url)
                .body(hex_transaction.to_string())
                .send().await?
                .text().await?;
            Ok(resp)
        }

        // GET /address/:address
        // GET /scripthash/:hash

        // Get information about an address/scripthash.
        // Available fields: address/scripthash, chain_stats and mempool_stats.
        // {chain,mempool}_stats each contain an object with tx_count, funded_txo_count, funded_txo_sum, spent_txo_count and spent_txo_sum.
        // Elements-based chains don't have the {funded,spent}_txo_sum fields.
        #[allow(dead_code)]
        pub async fn get_address(
            &self,
            address: &str,
        ) -> Result<AddressInfoFormat, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}{}", self.url, "/address/", address);
            let resp = self.reqwest.get(&request_url).send().await?.json().await?;
            Ok(resp)
        }
        #[allow(dead_code)]
        pub async fn get_script_hash(
            &self,
            scripthash: &str,
        ) -> Result<AddressInfoFormat, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}{}", self.url, "/scripthash/", scripthash);
            let resp = self.reqwest.get(&request_url).send().await?.json().await?;
            Ok(resp)
        }

        // GET /address/:address/txs
        // GET /scripthash/:hash/txs

        // Get transaction history for the specified address/scripthash, sorted with newest first.
        // Returns up to 50 mempool transactions plus the first 25 confirmed transactions. You can request more confirmed transactions using :last_seen_txid(see below).
        #[allow(dead_code)]
        pub async fn get_address_txs(
            &self,
            address: &str,
        ) -> Result<Vec<TransactionFormat>, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}{}{}", self.url, "/address/", address, "/txs");
            let resp = self.reqwest.get(&request_url).send().await?.json().await?;
            Ok(resp)
        }
        #[allow(dead_code)]
        pub async fn get_script_hash_txs(
            &self,
            scripthash: &str,
        ) -> Result<Vec<TransactionFormat>, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}{}{}", self.url, "/scripthash/", scripthash, "/txs");
            let resp = self.reqwest.get(&request_url).send().await?.json().await?;
            Ok(resp)
        }
        // GET /address/:address/txs/chain[/:last_seen_txid]
        // GET /scripthash/:hash/txs/chain[/:last_seen_txid]

        // Get confirmed transaction history for the specified address/scripthash, sorted with newest first.
        // Returns 25 transactions per page. More can be requested by specifying the last txid seen by the previous query.
        #[allow(dead_code)]
        pub async fn get_address_txs_chain(
            &self,
            address: &str,
            txid: Option<&str>,
        ) -> Result<Vec<TransactionFormat>, Box<dyn std::error::Error>> {
            let mut request_url = format!("{}{}{}{}", self.url, "/address/", address, "/txs/chain");
            match txid {
                Some(txid) => request_url.push_str(&format!("/{}", txid)),
                _ => (),
            }
            let resp = self.reqwest.get(&request_url).send().await?.json().await?;
            Ok(resp)
        }
        #[allow(dead_code)]
        pub async fn get_script_hash_txs_chain(
            &self,
            scripthash: &str,
            txid: Option<&str>,
        ) -> Result<Vec<TransactionFormat>, Box<dyn std::error::Error>> {
            let mut request_url = format!(
                "{}{}{}{}",
                self.url, "/scripthash/", scripthash, "/txs/chain"
            );
            match txid {
                Some(txid) => request_url.push_str(&format!("/{}", txid)),
                _ => (),
            }
            let resp = self.reqwest.get(&request_url).send().await?.json().await?;
            Ok(resp)
        }
        // GET /address/:address/txs/mempool
        // GET /scripthash/:hash/txs/mempool

        // Get unconfirmed transaction history for the specified address/scripthash.

        // Returns up to 50 transactions (no paging).
        #[allow(dead_code)]
        pub async fn get_address_txs_mempool(
            &self,
            address: &str,
        ) -> Result<Vec<TransactionFormat>, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}{}{}", self.url, "/address/", address, "/txs/mempool");
            let resp = self.reqwest.get(&request_url).send().await?.json().await?;
            Ok(resp)
        }
        #[allow(dead_code)]
        pub async fn get_script_hash_txs_mempool(
            &self,
            scripthash: &str,
        ) -> Result<Vec<TransactionFormat>, Box<dyn std::error::Error>> {
            let request_url = format!(
                "{}{}{}{}",
                self.url, "/scripthash/", scripthash, "/txs/mempool"
            );
            let resp = self.reqwest.get(&request_url).send().await?.json().await?;
            Ok(resp)
        }
        // GET /address/:address/utxo
        // GET /scripthash/:hash/utxo

        // Get the list of unspent transaction outputs associated with the address/scripthash.

        // Available fields: txid, vout, value and status (with the status of the funding tx).

        // Elements-based chains have a valuecommitment field that may appear in place of value, plus the following additional fields: asset/assetcommitment, nonce/noncecommitment, surjection_proof and range_proof.
        #[allow(dead_code)]
        pub async fn get_address_utxo(
            &self,
            address: &str,
        ) -> Result<Vec<UtxoFormat>, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}{}{}", self.url, "/address/", address, "/utxo");
            let resp = self.reqwest.get(&request_url).send().await?.json().await?;
            Ok(resp)
        }
        #[allow(dead_code)]
        pub async fn get_script_hash_utxo(
            &self,
            scripthash: &str,
        ) -> Result<Vec<UtxoFormat>, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}{}{}", self.url, "/scripthash/", scripthash, "/utxo");
            let resp = self.reqwest.get(&request_url).send().await?.json().await?;
            Ok(resp)
        }
        // GET /address-prefix/:prefix

        // This feature is disabled by default on custom api
        // Search for addresses beginning with :prefix.
        // Returns a JSON array with up to 10 results.
        #[allow(dead_code)]
        pub async fn get_address_prefix(
            &self,
            prefix: &str,
        ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}{}", self.url, "/address-prefix/", prefix);
            let resp = self.reqwest.get(&request_url).send().await?.json().await?;
            Ok(resp)
        }
        //     GET /mempool

        // Get mempool backlog statistics. Returns an object with:
        //     count: the number of transactions in the mempool
        //     vsize: the total size of mempool transactions in virtual bytes
        //     total_fee: the total fee paid by mempool transactions in satoshis
        //     fee_histogram: mempool fee-rate distribution histogram
        //     An array of (feerate, vsize) tuples, where each entry's vsize is the total vsize of transactions paying more than feerate but less than the previous entry's feerate (except for the first entry, which has no upper bound). This matches the format used by the Electrum RPC protocol for mempool.get_fee_histogram.

        // Example output:

        // {
        //   "count": 8134,
        //   "vsize": 3444604,
        //   "total_fee":29204625,
        //   "fee_histogram": [[53.01, 102131], [38.56, 110990], [34.12, 138976], [24.34, 112619], [3.16, 246346], [2.92, 239701], [1.1, 775272]]
        // }

        //     In this example, there are transactions weighting a total of 102,131 vbytes that are paying more than 53 sat/vB, 110,990 vbytes of transactions paying between 38 and 53 sat/vB, 138,976 vbytes paying between 34 and 38, etc.
        #[allow(dead_code)]
        pub async fn get_mempool(&self) -> Result<MemPoolFormat, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}", self.url, "/mempool");
            let resp = self.reqwest.get(&request_url).send().await?.json().await?;
            Ok(resp)
        }
        // GET /mempool/txids

        /// Get the full list of txids in the mempool as an array.
        /// The order of the txids is arbitrary and does not match bitcoind's.
        #[allow(dead_code)]
        pub async fn get_mempool_txids(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}", self.url, "/mempool/txids");
            let resp = self.reqwest.get(&request_url).send().await?.json().await?;
            Ok(resp)
        }

        // GET /mempool/recent

        // Get a list of the last 10 transactions to enter the mempool.

        // Each transaction object contains simplified overview data, with the following fields: txid, fee, vsize and value
        // Fee estimates
        #[allow(dead_code)]
        pub async fn get_mempool_recent(&self) -> Result<Vec<MempoolTxFormat>, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}", self.url, "/mempool/recent");
            let resp = self.reqwest.get(&request_url).send().await?.json().await?;
            Ok(resp)
        }
        // GET /fee-estimates

        // Get an object where the key is the confirmation target (in number of blocks) and the value is the estimated feerate (in sat/vB).

        // The available confirmation targets are 1-25, 144, 504 and 1008 blocks.

        // For example: { "1": 87.882, "2": 87.882, "3": 87.882, "4": 87.882, "5": 81.129, "6": 68.285, ..., "144": 1.027, "504": 1.027, "1008": 1.027 }

        #[allow(dead_code)]
        pub async fn fee_estimate(&self) -> Result<HashMap<String, f32>, Box<dyn std::error::Error>> {
            let request_url = format!("{}{}", self.url, "/fee-estimates");
            let resp = self.reqwest.get(&request_url).send().await?.json().await?;
            Ok(resp)
        }
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio_test;
    static ENDPOINT_URL: &str = "https://blockstream.info/testnet/api/";
    fn default_client() -> ApiClient {
        return ApiClient::new(ENDPOINT_URL, None).unwrap();
    }
    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }
    #[test]
    fn get_block() {
        let client = default_client();
        let response =
            aw!(client
                .get_block("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7"));
        assert_eq!(response.is_err(), false);
    }
    #[test]
    fn get_block_status() {
        let client = default_client();
        let response =  aw!(client
            .get_block_status("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7"));
        assert_eq!(response.is_err(), false);
    }
    #[test]
    fn get_block_txs_with_and_without_index() {
        let client = default_client();
        let first_txs_index =  aw!(client.get_block_txs(
            "000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7",
            None,
        ));
        let second_txs_index =  aw!(client.get_block_txs(
            "000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7",
            Some(25),
        ));
        assert_eq!(first_txs_index.is_err(), false);
        assert_eq!(second_txs_index.is_err(), false);
    }
    #[test]
    fn get_block_txids() {
        let client = default_client();
        let txids_list =  aw!(client
            .get_block_txids("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7"));
        assert_eq!(txids_list.is_err(), false);
    }
    #[test]
    fn get_block_txid_at_index() {
        let client = default_client();
        let txid =  aw!(client.get_block_txid_at_index(
            "000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7",
            2,
        ));
        assert_eq!(txid.is_err(), false);
    }

    #[test]
    fn get_block_raw_format() {
        let client = default_client();
        let response =  aw!(client.get_block_raw_format(
            "000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7",
        ));
        assert_eq!(response.is_err(), false);
    }
    #[test]
    fn get_block_height() {
        let client = default_client();
        let block_hash =  aw!(client.get_block_height(424242)).unwrap();
        let block =  aw!(client.get_block(&block_hash));
        assert_eq!(block.is_err(), false);
    }
    #[test]
    // Return 10 blocks from start_height
    fn get_blocks() {
        let client = default_client();
        let blocks =  aw!(client.get_blocks(1234));
        assert_eq!(blocks.is_err(), false);
    }
    #[test]
    // Function need return last block height
    fn get_blocks_tip_height() {
        let client = default_client();
        let height =  aw!(client.get_blocks_tip_height());

        assert_eq!(height.is_err(), false);
    }
    #[test]
    // Verify function return hash
    fn get_blocks_tip_hash() {
        let client = default_client();
        let hash =  aw!(client.get_blocks_tip_hash());

        assert_eq!(hash.is_err(), false);
    }
    #[test]
    // Check tx version
    fn get_tx() {
        let client = default_client();
        let tx =  aw!(client.get_tx("c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24"));
        assert_eq!(tx.is_err(), false);
    }
    #[test]
    // Tx status is confirmed
    fn get_tx_status() {
        let client = default_client();
        let tx_status =  aw!(client
            .get_tx_status("c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24"));
        assert_eq!(tx_status.is_err(), false);
    }
    #[test]
    // Tx raw
    fn get_tx_raw() {
        let client = default_client();
        let tx_raw =
        aw!(client.get_tx_raw("c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24"));
        assert_eq!(tx_raw.is_err(), false);
    }
    #[test]
    // Tx hex
    fn get_tx_hex() {
        let client = default_client();
        let tx_hex =
        aw!(client.get_tx_hex("c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24"));
        assert_eq!(tx_hex.is_err(), false);
    }
    #[test]
    fn get_tx_merkleblock_proof() {
        let client = default_client();
        let tx_hex =  aw!(client.get_tx_merkleblock_proof(
            "c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24",
        ));
        assert_eq!(tx_hex.is_err(), false);
    }
    #[test]
    fn get_tx_merkle_proof() {
        let client = default_client();
        let merkle_proof =  aw!(client.get_tx_merkle_proof(
            "6814c0b3915a8de663851b9887e0cce7d0d6c6b3f7c28b97ba8a643b72e1b7c3",
        ));
        assert_eq!(merkle_proof.is_err(), false);
    }
    #[test]
    fn get_tx_outspend() {
        let client = default_client();
        let outspend =  aw!(client.get_tx_outspend(
            "fac9af7f793330af3cc0bce4790d98499c59d47a125af7260edd61d647003316",
            Some(1),
        ));
        assert_eq!(outspend.is_err(), false);
    }
    #[test]
    fn get_tx_outspends() {
        let client = default_client();
        let outpends =  aw!(client
            .get_tx_outspends("fac9af7f793330af3cc0bce4790d98499c59d47a125af7260edd61d647003316"));
        assert_eq!(outpends.is_err(), false);
    }
    #[test]
    fn post_tx() {
        let client = default_client();
        let resp =  aw!(client.post_tx("010000000001010000000000000000000000000000000000000000000000000000000000000000ffffffff2003220d1c04d6d37c5f0877fffb9a4b3500000d2f6e6f64655374726174756d2f00000000030000000000000000266a24aa21a9ed61dc942663feda48033d1026d2fa8acf0f098870202c541bffa7771e8dc51e159b0e2801000000001976a914dfdf4d53296fac595dc33d8ac7216ba516b8dcc588ac8ffd0200000000001976a914bfcc245931cbad63d09f62df43bcab989991014e88ac0120000000000000000000000000000000000000000000000000000000000000000000000000"));
        assert_eq!(resp.is_err(), false)
    }
    #[test]
    fn get_address() {
        let client = default_client();
        let address =  aw!(client.get_address("2MvJVm11phGoxEekPB8Hw2Tksb57eVRGHC5"));
        assert_eq!(address.is_err(), false)
    }
    #[test]
    fn get_script_hash() {
        let client = default_client();
        let address =  aw!(client
            .get_script_hash("c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c"));
        assert_eq!(address.is_err(), false)
    }
    #[test]
    fn get_address_txs() {
        let client = default_client();
        let tx_list =  aw!(client.get_address_txs("2MvJVm11phGoxEekPB8Hw2Tksb57eVRGHC5"));
        assert_eq!(tx_list.is_err(), false)
    }
    #[test]
    fn get_script_hash_txs() {
        let client = default_client();
        let tx_list =  aw!(client.get_script_hash_txs(
            "c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c",
        ));
        assert_eq!(tx_list.is_err(), false)
    }
    #[test]
    fn get_address_txs_chain() {
        let client = default_client();
        let tx_list =  aw!(client.get_address_txs_chain(
            "n1vgV8XmoggmRXzW3hGD8ZNTAgvhcwT4Gk",
            Some("d0075b62f8b3e464472b8edecf56083ca3e9e8424f5f332ed2f9045d7fcccddc"),
        ));
        let tx_list_from_index =  aw!(client.get_address_txs_chain(
            "n1vgV8XmoggmRXzW3hGD8ZNTAgvhcwT4Gk",
            Some(&tx_list.unwrap()[1].txid),
        ));
        assert_eq!(tx_list_from_index.is_err(), false)
    }
    #[test]
    fn get_script_hash_txs_chain() {
        let client = default_client();
        let tx_list =  aw!(client.get_script_hash_txs_chain(
            "c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c",
            None,
        ));
        assert_eq!(tx_list.is_err(), false)
    }
    #[test]
    fn get_address_txs_mempool() {
        let client = default_client();
        let tx_list =  aw!(client.get_address_txs_mempool("2MvJVm11phGoxEekPB8Hw2Tksb57eVRGHC5"));
        assert_eq!(tx_list.is_err(), false)
    }
    #[test]
    fn get_script_hash_txs_mempool() {
        let client = default_client();
        let tx_list =  aw!(client.get_script_hash_txs_mempool(
            "c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c",
        ));
        assert_eq!(tx_list.is_err(), false)
    }
    #[test]
    fn get_address_utxo() {
        let client = default_client();
        let utxo =  aw!(client.get_address_utxo("2NDcM3CGUTwqFL7y8BSBJTYJ9kToeXawkUF"));
        assert_eq!(utxo.is_err(), false)
    }
    #[test]
    fn get_script_hash_utxo() {
        let client = default_client();
        let utxo =  aw!(client.get_script_hash_utxo(
            "c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c",
        ));
        assert_eq!(utxo.is_err(), false)
    }
    #[test]
    fn get_address_prefix() {
        let client = default_client();
        let addresses =  aw!(client.get_address_prefix("2NDcM"));
        assert_eq!(addresses.is_err(), false)
    }
    // fee_estimate(get_mempool_recent(get_mempool_txids(get_mempool
    #[test]
    fn get_mempool() {
        let client = default_client();
        let mempool =  aw!(client.get_mempool());
        assert_eq!(mempool.is_err(), false)
    }
    #[test]
    fn get_mempool_txids() {
        let client = default_client();
        let mempool_txids =  aw!(client.get_mempool_txids());
        assert_eq!(mempool_txids.is_err(), false)
    }
    #[test]
    fn get_mempool_recent() {
        let client = default_client();
        let mempool_txids =  aw!(client.get_mempool_recent());
        assert_eq!(mempool_txids.is_err(), false)
    }
    #[test]
    fn fee_estimate() {
        let client = default_client();
        let fee =  aw!(client.fee_estimate());
        assert_eq!(fee.is_err(), false)
    }
}

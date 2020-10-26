use std::collections::HashMap;

use super::client::ApiClient;
use crate::data::blockstream::{
    AddressInfoFormat, BlockFormat, BlockStatus, MemPoolFormat, MempoolTxFormat, MerkleProofFormat,
    OutspentFormat, TransactionFormat, TxStatusFormat, UtxoFormat,
};

impl ApiClient {
    /// get_block Returns information about a block.
    ///
    /// Route : GET /block/:hash. Available fields:
    ///
    /// Elements-based chains have an additional proof field. See block format for more details.
    /// The response from this endpoint can be cached indefinitely.
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_block("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7").await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````
    pub async fn get_block(&self, hash: &str) -> Result<BlockFormat, Box<dyn std::error::Error>> {
        let request_url = format!("{}/block/{}", self.url, hash);
        let resp: BlockFormat = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }
    /// get_block_status Returns the block status.
    ///
    /// Route : GET /block/:hash/status. Available fields:
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_block_status("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7").await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````
    pub async fn get_block_status(
        &self,
        hash: &str,
    ) -> Result<BlockStatus, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}{}{}", self.url, "/block/", hash, "/status");
        let resp: BlockStatus = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }
    /// get_block_txs Returns a list of transactions in the block (up to 25 transactions beginning at start_index).
    ///
    /// Route : GET /block/:hash/txs[/:start_index]
    ///
    /// Transactions returned here do not have the status field, since all the transactions share the same block and confirmation status.
    /// The response from this endpoint can be cached indefinitely.
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_block_txs("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7", Some(25)).await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````
    pub async fn get_block_txs(
        &self,
        hash: &str,
        start_index: Option<i32>, // Why Option ?
    ) -> Result<Vec<TransactionFormat>, Box<dyn std::error::Error>> {
        let request_url = if let Some(i) = start_index {
            format!("{}/block/{}/txs/{}", self.url, hash, i)
        } else {
            format!("{}/block/{}/txs", self.url, hash)
        };
        let resp: Vec<TransactionFormat> =
            self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }
    /// get_block_txids Returns a list of all txids in the block.
    ///
    /// Route : GET /block/:hash/txids
    ///
    ///The response from this endpoint can be cached indefinitely.
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_block_txids("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7").await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````
    pub async fn get_block_txids(
        &self,
        hash: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let request_url = format!("{}{}{}{}", self.url, "/block/", hash, "/txids");
        let resp: Vec<String> = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }
    /// get_block_txid_at_index Returns the transaction at index :index within the specified block.
    ///
    /// Route : GET /block/:hash/txid/:index
    ///
    /// The response from this endpoint can be cached indefinitely.
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_block_txid_at_index("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7",25).await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````
    pub async fn get_block_txid_at_index(
        &self,
        hash: &str,
        index: i32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let request_url = format!("{}/block/{}/txid/{}", self.url, hash, index);
        let resp: String = self.reqwest.get(&request_url).send().await?.text().await?;
        Ok(resp)
    }
    /// get_block_raw_format Returns the raw block representation in binary.
    ///
    /// Route : GET /block/:hash/raw
    ///
    /// The response from this endpoint can be cached indefinitely.
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_block_raw_format("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7").await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````

    pub async fn get_block_raw_format(
        &self,
        hash: &str,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let request_url = format!("{}/block/{}/raw", self.url, hash);
        let resp = self
            .reqwest
            .get(&request_url)
            .send()
            .await?
            .bytes()
            .await?
            .to_vec();
        Ok(resp)
    }

    /// get_block_height Returns the hash of the block currently at height.
    ///
    /// Route : GET /block-height/:height
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_block_height(424242).await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````
    pub async fn get_block_height(
        &self,
        height: i32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let request_url = format!("{}/block-height/{}", self.url, height);
        let resp = self.reqwest.get(&request_url).send().await?.text().await?;
        Ok(resp)
    }
    /// get_blocks Returns the 10 newest blocks starting at the tip or at start_height if specified.
    ///
    /// Route : GET /blocks[/:start_height]
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_blocks(1234).await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````

    pub async fn get_blocks(
        &self,
        start_height: i32,
    ) -> Result<Vec<BlockFormat>, Box<dyn std::error::Error>> {
        let request_url = format!("{}/blocks/{}", self.url, start_height);
        let resp = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }
    /// get_blocks_tip_height Returns the height of the last block.
    ///
    /// Route : GET /blocks/tip/height
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_blocks_tip_height().await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````

    pub async fn get_blocks_tip_height(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let request_url = format!("{}/blocks/tip/height", self.url);
        let resp = self
            .reqwest
            .get(&request_url)
            .send()
            .await?
            .text()
            .await?
            .parse()?;
        Ok(resp)
    }
    /// get_blocks_tip_hash Returns the hash of the last block.
    ///
    /// Route : GET /blocks/tip/hash
    ///
    ///
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_blocks_tip_height().await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````

    pub async fn get_blocks_tip_hash(&self) -> Result<String, Box<dyn std::error::Error>> {
        let request_url = format!("{}/blocks/tip/hash", self.url);
        let resp = self.reqwest.get(&request_url).send().await?.text().await?;
        Ok(resp)
    }
    /// get_tx Returns information about the transaction. Available fields: txid, version, locktime, size, weight, fee, vin, vout and status (see transaction format for details).
    ///
    /// Route : GET /tx/:txid
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_tx("c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24").await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````

    pub async fn get_tx(
        &self,
        txid: &str,
    ) -> Result<TransactionFormat, Box<dyn std::error::Error>> {
        let request_url = format!("{}/tx/{}", self.url, txid);
        let resp = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }
    /// get_tx_status Returns the transaction confirmation status. Available fields: confirmed (boolean), block_height (optional) and block_hash (optional).
    ///
    /// Route : GET /tx/:txid/status
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_tx_status("c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24").await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````
    pub async fn get_tx_status(
        &self,
        txid: &str,
    ) -> Result<TxStatusFormat, Box<dyn std::error::Error>> {
        let request_url = format!("{}/tx/{}/status", self.url, txid);
        let resp = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }

    /// get_tx_raw Returns the raw transaction as binary data.
    ///
    /// Route : GET /tx/:txid/raw
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_tx_raw("c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24").await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````
    pub async fn get_tx_raw(&self, txid: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let request_url = format!("{}/tx/{}/raw", self.url, txid);
        let resp = self
            .reqwest
            .get(&request_url)
            .send()
            .await?
            .bytes()
            .await?
            .to_vec();
        Ok(resp)
    }

    /// get_tx_hex Returns the raw transaction in hex
    ///
    /// Route : GET /tx/:txid/hex
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_tx_hex("c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24").await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````
    pub async fn get_tx_hex(&self, txid: &str) -> Result<String, Box<dyn std::error::Error>> {
        let request_url = format!("{}/tx/{}/raw", self.url, txid);
        let resp = self.reqwest.get(&request_url).send().await?.text().await?;
        Ok(resp)
    }
    /// get_tx_merkleblock_proof Returns a merkle inclusion proof for the transaction using bitcoind's merkleblock format.
    /// Note: This endpoint is not currently available for Liquid/Elements-based chains.
    /// Route : GET /tx/:txid/merkleblock-proof
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_tx_merkleblock_proof("c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24").await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````
    pub async fn get_tx_merkleblock_proof(
        &self,
        txid: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let request_url = format!("{}/tx/{}/merkleblock-proof", self.url, txid);
        let resp = self.reqwest.get(&request_url).send().await?.text().await?;
        Ok(resp)
    }

    /// get_tx_merkle_proof Returns a merkle inclusion proof for the transaction using Electrum's blockchain.transaction.get_merkle format.
    ///
    /// Route : GET /tx/:txid/merkle-proof
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_tx_merkle_proof("c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24").await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````
    pub async fn get_tx_merkle_proof(
        &self,
        txid: &str,
    ) -> Result<MerkleProofFormat, Box<dyn std::error::Error>> {
        let request_url = format!("{}/tx/{}/merkle-proof", self.url, txid);
        let resp = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }

    /// get_tx_outspend Returns the spending status of a transaction output.
    /// Available fields: spent (boolean), txid (optional), vin (optional) and status (optional, the status of the spending tx).
    ///
    /// Route : GET /tx/:txid/outspend/:vout
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_tx_outspend("fac9af7f793330af3cc0bce4790d98499c59d47a125af7260edd61d647003316",Some(1)).await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````
    pub async fn get_tx_outspend(
        &self,
        txid: &str,
        vout: Option<i32>,
    ) -> Result<OutspentFormat, Box<dyn std::error::Error>> {
        let request_url = if let Some(vout_idx) = vout {
            format!("{}/tx/{}/outspend/{}", self.url, txid, vout_idx)
        } else {
            format!("{}/tx/{}/outspend", self.url, txid) // FIXME: not sure if this exist
        };
        let resp = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }

    /// get_tx_outspends Returns the spending status of all transaction outputs.
    ///
    /// Route : GET /tx/:txid/outspends
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_tx_outspends("fac9af7f793330af3cc0bce4790d98499c59d47a125af7260edd61d647003316").await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````

    pub async fn get_tx_outspends(
        &self,
        txid: &str,
    ) -> Result<Vec<OutspentFormat>, Box<dyn std::error::Error>> {
        let request_url = format!("{}/tx/{}/outspends", self.url, txid);
        let resp = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }

    /// post_tx Broadcast a raw transaction to the network.
    /// The transaction should be provided as hex in the request body. The txid will be returned on success.
    ///
    /// Route : POST /tx
    ///
    pub async fn post_tx(
        &self,
        hex_transaction: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let request_url = format!("{}/tx", self.url);
        let resp = self
            .reqwest
            .post(&request_url)
            .body(hex_transaction.to_string())
            .send()
            .await?
            .text()
            .await?;
        Ok(resp)
    }

    /// get_address Get information about an address
    /// Available fields: address/scripthash, chain_stats and mempool_stats.
    /// {chain,mempool}_stats each contain an object with tx_count, funded_txo_count, funded_txo_sum, spent_txo_count and spent_txo_sum.
    /// Elements-based chains don't have the {funded,spent}_txo_sum fields.
    ///
    /// Route : GET /address/:address
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_address("2MvJVm11phGoxEekPB8Hw2Tksb57eVRGHC5").await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````
    pub async fn get_address(
        &self,
        address: &str,
    ) -> Result<AddressInfoFormat, Box<dyn std::error::Error>> {
        let request_url = format!("{}/address/{}", self.url, address);
        let resp = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }

    /// get_script_hash Get information about an scripthash
    /// Available fields: scripthash, chain_stats and mempool_stats.
    /// {chain,mempool}_stats each contain an object with tx_count, funded_txo_count, funded_txo_sum, spent_txo_count and spent_txo_sum.
    /// Elements-based chains don't have the {funded,spent}_txo_sum fields.
    ///
    /// Route : GET /scripthash/:hash
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_script_hash("c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c").await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````
    pub async fn get_script_hash(
        &self,
        scripthash: &str,
    ) -> Result<AddressInfoFormat, Box<dyn std::error::Error>> {
        let request_url = format!("{}/scripthash/{}", self.url, scripthash);
        let resp = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }

    /// get_address_txs Get transaction history for the specified address/scripthash, sorted with newest first.
    /// Returns up to 50 mempool transactions plus the first 25 confirmed transactions. You can request more confirmed transactions using :last_seen_txid(see below).
    ///
    /// Route : GET /address/:address/txs
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_address_txs("2MvJVm11phGoxEekPB8Hw2Tksb57eVRGHC5").await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````
    pub async fn get_address_txs(
        &self,
        address: &str,
    ) -> Result<Vec<TransactionFormat>, Box<dyn std::error::Error>> {
        let request_url = format!("{}/address/{}/txs", self.url, address);
        let resp = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }
    /// get_script_hash_txs Get transaction history for the specified address/scripthash, sorted with newest first.
    /// Returns up to 50 mempool transactions plus the first 25 confirmed transactions. You can request more confirmed transactions using :last_seen_txid(see below).
    ///
    /// Route : GET /scripthash/:hash/txs
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_script_hash_txs("c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c").await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````

    pub async fn get_script_hash_txs(
        &self,
        scripthash: &str,
    ) -> Result<Vec<TransactionFormat>, Box<dyn std::error::Error>> {
        let request_url = format!("{}/scripthash/{}/txs", self.url, scripthash);
        let resp = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }

    /// get_address_txs_chain Get confirmed transaction history for the specified address/scripthash, sorted with newest first.
    /// Returns 25 transactions per page. More can be requested by specifying the last txid seen by the previous query.
    ///
    /// Route : GET /address/:address/txs/chain[/:last_seen_txid]
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_address_txs_chain("n1vgV8XmoggmRXzW3hGD8ZNTAgvhcwT4Gk",Some("d0075b62f8b3e464472b8edecf56083ca3e9e8424f5f332ed2f9045d7fcccddc")).await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````
    pub async fn get_address_txs_chain(
        &self,
        address: &str,
        txid: Option<&str>,
    ) -> Result<Vec<TransactionFormat>, Box<dyn std::error::Error>> {
        let request_url = if let Some(id) = txid {
            format!("{}/address/{}/txs/chain/{}", self.url, address, id)
        } else {
            format!("{}/address/{}/txs/chain", self.url, address)
        };
        let resp = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }

    /// get_script_hash_txs_chain Get confirmed transaction history for the specified address/scripthash, sorted with newest first.
    /// Returns 25 transactions per page. More can be requested by specifying the last txid seen by the previous query.
    ///
    /// Route : GET /scripthash/:hash/txs/chain[/:last_seen_txid]
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_script_hash_txs_chain("c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c",None).await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````
    pub async fn get_script_hash_txs_chain(
        &self,
        scripthash: &str,
        txid: Option<&str>,
    ) -> Result<Vec<TransactionFormat>, Box<dyn std::error::Error>> {
        let request_url = if let Some(id) = txid {
            format!("{}/scripthash/{}/txs/chain/{}", self.url, scripthash, id)
        } else {
            format!("{}/scripthash/{}/txs/chain", self.url, scripthash)
        };
        let resp = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }

    /// get_address_txs_mempool Get unconfirmed transaction history for the specified address.
    /// Returns up to 50 transactions (no paging).
    ///
    /// Route : GET /address/:address/txs/mempool
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_address_txs_mempool("2MvJVm11phGoxEekPB8Hw2Tksb57eVRGHC5").await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````
    pub async fn get_address_txs_mempool(
        &self,
        address: &str,
    ) -> Result<Vec<TransactionFormat>, Box<dyn std::error::Error>> {
        let request_url = format!("{}/address/{}/txs/mempool", self.url, address);
        let resp = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }

    /// get_script_hash_txs_mempool Get unconfirmed transaction history for the specified scripthash.
    /// Returns up to 50 transactions (no paging).
    ///
    /// Route : GET /scripthash/:hash/txs/mempool
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_script_hash_txs_mempool("c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c").await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````
    pub async fn get_script_hash_txs_mempool(
        &self,
        scripthash: &str,
    ) -> Result<Vec<TransactionFormat>, Box<dyn std::error::Error>> {
        let request_url = format!("{}/scripthash/{}/txs/mempool", self.url, scripthash,);
        let resp = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }

    /// get_address_utxo Get the list of unspent transaction outputs associated with the address
    /// Available fields: txid, vout, value and status (with the status of the funding tx).
    /// Elements-based chains have a valuecommitment field that may appear in place of value, plus the following additional fields: asset/assetcommitment, nonce/noncecommitment, surjection_proof and range_proof.
    ///
    /// Route : GET /address/:address/utxo
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_address_utxo("2NDcM3CGUTwqFL7y8BSBJTYJ9kToeXawkUF").await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````
    pub async fn get_address_utxo(
        &self,
        address: &str,
    ) -> Result<Vec<UtxoFormat>, Box<dyn std::error::Error>> {
        let request_url = format!("{}/address/{}/utxo", self.url, address);
        let resp = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }

    /// get_script_hash_utxo Get the list of unspent transaction outputs associated with the address
    /// Available fields: txid, vout, value and status (with the status of the funding tx).
    /// Elements-based chains have a valuecommitment field that may appear in place of value, plus the following additional fields: asset/assetcommitment, nonce/noncecommitment, surjection_proof and range_proof.
    ///
    /// Route : GET /scripthash/:hash/utxo
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_script_hash_utxo("c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c").await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````
    pub async fn get_script_hash_utxo(
        &self,
        scripthash: &str,
    ) -> Result<Vec<UtxoFormat>, Box<dyn std::error::Error>> {
        let request_url = format!("{}/scripthash/{}/utxo", self.url, scripthash);
        let resp = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }
    /// get_address_prefix  This feature is disabled by default on custom api Search for addresses beginning with :prefix.
    /// Returns a JSON array with up to 10 results.
    ///
    /// Route : GET /address-prefix/:prefix
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_address_prefix("2NDcM").await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````

    pub async fn get_address_prefix(
        &self,
        prefix: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let request_url = format!("{}/address-prefix/{}", self.url, prefix);
        let resp = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }
    /// get_mempool Get mempool backlog statistics. Returns an object with:
    ///     count: the number of transactions in the mempool
    ///     vsize: the total size of mempool transactions in virtual bytes
    ///     total_fee: the total fee paid by mempool transactions in satoshis
    ///     fee_histogram: mempool fee-rate distribution histogram
    ///     An array of (feerate, vsize) tuples, where each entry's vsize is the total vsize of transactions paying more than feerate but less than the previous entry's feerate (except for the first entry, which has no upper bound). This matches the format used by the Electrum RPC protocol for mempool.get_fee_histogram.
    ///
    /// Route : GET /mempool
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_mempool().await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````
    /// Example output:
    /// ````json
    /// {
    ///   "count": 8134,
    ///   "vsize": 3444604,
    ///   "total_fee":29204625,
    ///   "fee_histogram": [[53.01, 102131], [38.56, 110990], [34.12, 138976], [24.34, 112619], [3.16, 246346], [2.92, 239701], [1.1, 775272]]
    /// }
    /// ````
    /// In this example, there are transactions weighting a total of 102,131 vbytes that are paying more than 53 sat/vB, 110,990 vbytes of transactions paying between 38 and 53 sat/vB, 138,976 vbytes paying between 34 and 38, etc.
    pub async fn get_mempool(&self) -> Result<MemPoolFormat, Box<dyn std::error::Error>> {
        let request_url = format!("{}/mempool", self.url);
        let resp = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }
    /// get_mempool_txids Get the full list of txids in the mempool as an array.
    /// The order of the txids is arbitrary and does not match bitcoind's.
    ///
    /// Route : GET /mempool/txids
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_mempool_txids().await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````
    pub async fn get_mempool_txids(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let request_url = format!("{}/mempool/txids", self.url);
        let resp = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }
    /// get_mempool_recent  Get a list of the last 10 transactions to enter the mempool. Each transaction object contains simplified overview data, with the following fields: txid, fee, vsize and value
    /// Fee estimates
    /// The order of the txids is arbitrary and does not match bitcoind's.
    ///
    /// Route : GET /mempool/recent
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.get_mempool_recent().await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````

    pub async fn get_mempool_recent(
        &self,
    ) -> Result<Vec<MempoolTxFormat>, Box<dyn std::error::Error>> {
        let request_url = format!("{}/mempool/recent", self.url);
        let resp = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }
    /// fee_estimate Get an object where the key is the confirmation target (in number of blocks) and the value is the estimated feerate (in sat/vB).
    /// The available confirmation targets are 1-25, 144, 504 and 1008 blocks.
    /// For example: { "1": 87.882, "2": 87.882, "3": 87.882, "4": 87.882, "5": 81.129, "6": 68.285, ..., "144": 1.027, "504": 1.027, "1008": 1.027 }
    ///
    /// Route : GET /fee-estimates
    ///
    /// Example :
    /// ````rust
    /// use esplora_api;
    ///
    /// #[tokio::main]
    /// async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    ///     let response = client.fee_estimate().await?;
    ///     println!("{:?}",response);
    ///     Ok(())
    /// }
    /// ````

    pub async fn fee_estimate(&self) -> Result<HashMap<String, f32>, Box<dyn std::error::Error>> {
        let request_url = format!("{}/fee-estimates", self.url);
        let resp = self.reqwest.get(&request_url).send().await?.json().await?;
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static ENDPOINT_URL: &str = "https://blockstream.info/testnet/api/";
    fn default_client() -> ApiClient {
        return ApiClient::new(ENDPOINT_URL, None).unwrap();
    }

    #[tokio::test]
    async fn get_block() {
        let client = default_client();
        let response = client
            .get_block("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7")
            .await;
        assert!(response.is_ok());
    }
    #[tokio::test]
    async fn get_block_status() {
        let client = default_client();
        let response = client
            .get_block_status("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7")
            .await;
        assert!(response.is_ok());
    }
    #[tokio::test]
    async fn get_block_txs_with_and_without_index() {
        let client = default_client();
        let first_txs_index = client
            .get_block_txs(
                "000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7",
                None,
            )
            .await;
        let second_txs_index = client
            .get_block_txs(
                "000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7",
                Some(25),
            )
            .await;
        assert!(first_txs_index.is_ok());
        assert!(second_txs_index.is_ok());
    }
    #[tokio::test]
    async fn get_block_txids() {
        let client = default_client();
        let txids_list = client
            .get_block_txids("000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7")
            .await;
        assert!(txids_list.is_ok());
    }
    #[tokio::test]
    async fn get_block_txid_at_index() {
        let client = default_client();
        let txid = client
            .get_block_txid_at_index(
                "000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7",
                2,
            )
            .await;
        assert!(txid.is_ok());
    }

    #[tokio::test]
    async fn get_block_raw_format() {
        let client = default_client();
        let response = client
            .get_block_raw_format(
                "000000000000003aaa3b99e31ed1cac4744b423f9e52ada4971461c81d4192f7",
            )
            .await;
        assert_eq!(response.is_err(), false);
    }
    #[tokio::test]
    async fn get_block_height() {
        let client = default_client();
        let block_hash = client.get_block_height(424242).await.unwrap();
        let block = client.get_block(&block_hash).await;
        assert!(block.is_ok());
    }
    #[tokio::test]
    // Return 10 blocks from start_height
    async fn get_blocks() {
        let client = default_client();
        let blocks = client.get_blocks(1234).await;
        assert!(blocks.is_ok());
    }
    #[tokio::test]
    // Function need return last block height
    async fn get_blocks_tip_height() {
        let client = default_client();
        let height = client.get_blocks_tip_height().await;

        assert!(height.is_ok());
    }
    #[tokio::test]
    // Verify function return hash
    async fn get_blocks_tip_hash() {
        let client = default_client();
        let hash = client.get_blocks_tip_hash().await;

        assert!(hash.is_ok());
    }
    #[tokio::test]
    // Check tx version
    async fn get_tx() {
        let client = default_client();
        let tx = client
            .get_tx("c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24")
            .await;
        assert_eq!(tx.is_err(), false);
    }
    #[tokio::test]
    // Tx status is confirmed
    async fn get_tx_status() {
        let client = default_client();
        let tx_status = client
            .get_tx_status("c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24")
            .await;
        assert!(tx_status.is_ok());
    }
    #[tokio::test]
    // Tx raw
    async fn get_tx_raw() {
        let client = default_client();
        let tx_raw = client
            .get_tx_raw("c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24")
            .await;
        assert!(tx_raw.is_ok());
    }
    #[tokio::test]
    // Tx hex
    async fn get_tx_hex() {
        let client = default_client();
        let tx_hex = client
            .get_tx_hex("c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24")
            .await;
        assert!(tx_hex.is_ok());
    }
    #[tokio::test]
    async fn get_tx_merkleblock_proof() {
        let client = default_client();
        let tx_hex = client
            .get_tx_merkleblock_proof(
                "c9ee6eff3d73d6cb92382125c3207f6447922b545d4d4e74c47bfeb56fff7d24",
            )
            .await;
        assert!(tx_hex.is_ok());
    }
    #[tokio::test]
    async fn get_tx_merkle_proof() {
        let client = default_client();
        let merkle_proof = client
            .get_tx_merkle_proof("6814c0b3915a8de663851b9887e0cce7d0d6c6b3f7c28b97ba8a643b72e1b7c3")
            .await;
        assert!(merkle_proof.is_ok());
    }
    #[tokio::test]
    async fn get_tx_outspend() {
        let client = default_client();
        let outspend = client
            .get_tx_outspend(
                "fac9af7f793330af3cc0bce4790d98499c59d47a125af7260edd61d647003316",
                Some(1),
            )
            .await;
        assert!(outspend.is_ok());
    }
    #[tokio::test]
    async fn get_tx_outspends() {
        let client = default_client();
        let outpends = client
            .get_tx_outspends("fac9af7f793330af3cc0bce4790d98499c59d47a125af7260edd61d647003316")
            .await;
        assert!(outpends.is_ok());
    }
    #[tokio::test]
    async fn post_tx() {
        let client = default_client();
        let resp =  client.post_tx("010000000001010000000000000000000000000000000000000000000000000000000000000000ffffffff2003220d1c04d6d37c5f0877fffb9a4b3500000d2f6e6f64655374726174756d2f00000000030000000000000000266a24aa21a9ed61dc942663feda48033d1026d2fa8acf0f098870202c541bffa7771e8dc51e159b0e2801000000001976a914dfdf4d53296fac595dc33d8ac7216ba516b8dcc588ac8ffd0200000000001976a914bfcc245931cbad63d09f62df43bcab989991014e88ac0120000000000000000000000000000000000000000000000000000000000000000000000000").await;
        assert!(resp.is_ok());
    }
    #[tokio::test]
    async fn get_address() {
        let client = default_client();
        let address = client
            .get_address("2MvJVm11phGoxEekPB8Hw2Tksb57eVRGHC5")
            .await;
        assert!(address.is_ok())
    }
    #[tokio::test]
    async fn get_script_hash() {
        let client = default_client();
        let address = client
            .get_script_hash("c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c")
            .await;
        assert!(address.is_ok())
    }
    #[tokio::test]
    async fn get_address_txs() {
        let client = default_client();
        let tx_list = client
            .get_address_txs("2MvJVm11phGoxEekPB8Hw2Tksb57eVRGHC5")
            .await;
        assert!(tx_list.is_ok())
    }
    #[tokio::test]
    async fn get_script_hash_txs() {
        let client = default_client();
        let tx_list = client
            .get_script_hash_txs("c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c")
            .await;
        assert!(tx_list.is_ok())
    }
    #[tokio::test]
    async fn get_address_txs_chain() {
        let client = default_client();
        let tx_list = client
            .get_address_txs_chain(
                "n1vgV8XmoggmRXzW3hGD8ZNTAgvhcwT4Gk",
                Some("d0075b62f8b3e464472b8edecf56083ca3e9e8424f5f332ed2f9045d7fcccddc"),
            )
            .await;
        let tx_list_from_index = client
            .get_address_txs_chain(
                "n1vgV8XmoggmRXzW3hGD8ZNTAgvhcwT4Gk",
                Some(&tx_list.unwrap()[1].txid),
            )
            .await;
        assert!(tx_list_from_index.is_ok())
    }
    #[tokio::test]
    async fn get_script_hash_txs_chain() {
        let client = default_client();
        let tx_list = client
            .get_script_hash_txs_chain(
                "c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c",
                None,
            )
            .await;
        assert!(tx_list.is_ok())
    }
    #[tokio::test]
    async fn get_address_txs_mempool() {
        let client = default_client();
        let tx_list = client
            .get_address_txs_mempool("2MvJVm11phGoxEekPB8Hw2Tksb57eVRGHC5")
            .await;
        assert!(tx_list.is_ok())
    }
    #[tokio::test]
    async fn get_script_hash_txs_mempool() {
        let client = default_client();
        let tx_list = client
            .get_script_hash_txs_mempool(
                "c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c",
            )
            .await;
        assert!(tx_list.is_ok())
    }
    #[tokio::test]
    async fn get_address_utxo() {
        let client = default_client();
        let utxo = client
            .get_address_utxo("2NDcM3CGUTwqFL7y8BSBJTYJ9kToeXawkUF")
            .await;
        assert!(utxo.is_ok())
    }
    #[tokio::test]
    async fn get_script_hash_utxo() {
        let client = default_client();
        let utxo = client
            .get_script_hash_utxo(
                "c6598a8e5728c744b9734facbf1e786c3ff5101268739d38b14ea475b60eba3c",
            )
            .await;
        assert!(utxo.is_ok())
    }
    #[tokio::test]
    async fn get_address_prefix() {
        let client = default_client();
        let addresses = client.get_address_prefix("2NDcM").await;
        assert!(addresses.is_ok())
    }

    #[tokio::test]
    async fn get_mempool() {
        let client = default_client();
        let mempool = client.get_mempool().await;
        assert!(mempool.is_ok())
    }
    #[tokio::test]
    async fn get_mempool_txids() {
        let client = default_client();
        let mempool_txids = client.get_mempool_txids().await;
        assert!(mempool_txids.is_ok())
    }
    #[tokio::test]
    async fn get_mempool_recent() {
        let client = default_client();
        let mempool_txids = client.get_mempool_recent().await;
        assert!(mempool_txids.is_ok())
    }
    #[tokio::test]
    async fn fee_estimate() {
        let client = default_client();
        let fee = client.fee_estimate().await;
        assert!(fee.is_ok())
    }
}
